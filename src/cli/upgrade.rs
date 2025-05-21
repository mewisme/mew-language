// This file is part of Mew Language.
//
// Mew Language is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mew Language is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mew Language.  If not, see <https://www.gnu.org/licenses/>.

use reqwest;
use semver::Version;
use serde::Deserialize;
use std::fs;
use std::io::{copy, Cursor};
use std::process;
use tempfile;
use zip::ZipArchive;

#[derive(Deserialize)]
struct GitHubRelease {
  tag_name: String,
}

#[derive(Deserialize)]
struct GitHubTag {
  name: String,
}

struct VersionInfo {
  version: String,
  tag_name: String,
}

pub fn check_upgrade_completed() {
  let user_temp_dir = std::env::temp_dir();
  let upgrade_marker = user_temp_dir.join("mew_upgraded.txt");

  if upgrade_marker.exists() {
    if let Ok(content) = fs::read_to_string(&upgrade_marker) {
      println!("✅ Mew was successfully upgraded to v{}!", content.trim());
    } else {
      println!("✅ Mew was successfully upgraded!");
    }

    let _ = fs::remove_file(upgrade_marker);
  }
}

pub fn handle_upgrade(force: bool) -> Result<(), Box<dyn std::error::Error>> {
  let current_version = env!("CARGO_PKG_VERSION");

  let latest_version_info = match get_latest_version_info(current_version)? {
    Some(info) => info,
    None => {
      if !force {
        println!(
          "You are already running the latest version (v{}).",
          current_version
        );
        return Ok(());
      } else {
        println!("Force upgrading to the latest version even though you're already up to date.\n");
        get_latest_version_info_force(current_version)?
      }
    }
  };

  let temp_dir = tempfile::Builder::new().prefix("mew-upgrade").tempdir()?;
  let download_url = get_platform_download_url(&latest_version_info.tag_name)?;

  let content = download_with_retry(&download_url, 3)?;

  let temp_target = temp_dir.path().join("mew.new");

  let cursor = Cursor::new(content);
  let mut archive = match ZipArchive::new(cursor) {
    Ok(archive) => archive,
    Err(e) => {
      println!("Error opening archive: {}", e);
      println!("This might indicate a corrupted download or unsupported archive format.");
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("Failed to open ZIP archive: {}", e),
      )));
    }
  };

  let binary_name = if cfg!(target_os = "windows") {
    "mew.exe"
  } else {
    "mew"
  };
  let mut binary_path = None;

  for i in 0..archive.len() {
    let file = archive.by_index(i)?;
    let name = file.name();

    if name.ends_with(binary_name) {
      binary_path = Some(name.to_string());
      break;
    }
  }

  let binary_path = binary_path.ok_or_else(|| -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      format!("Could not find '{}' in the release archive", binary_name),
    ))
  })?;

  let mut binary_file = archive.by_name(&binary_path)?;
  let mut temp_file = fs::File::create(&temp_target)?;
  copy(&mut binary_file, &mut temp_file)?;

  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&temp_target)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&temp_target, perms)?;
  }

  #[cfg(target_os = "windows")]
  {
    let mut command = process::Command::new("powershell");
    command.arg("-c");
    command.arg(format!("irm https://mewis.me/install.ps1 | iex"));
    let output = command.status()?;

    if !output.success() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Installation failed with exit code: {:?}", output.code()),
      )));
    }

    return Ok(());
  }

  #[cfg(not(target_os = "windows"))]
  {
    fs::rename(&temp_target, &target_path)?;
    println!("Mew has been upgraded to v{}!", latest_version_info.version);
    return Ok(());
  }

  #[allow(unreachable_code)]
  Ok(())
}

fn get_latest_version_info(
  current_version: &str,
) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
  let client = reqwest::blocking::Client::new();

  let release_resp = client
    .get("https://api.github.com/repos/mewisme/mew-language/releases/latest")
    .header("User-Agent", format!("Mew-Language/{}", current_version))
    .send()?;

  if release_resp.status() == reqwest::StatusCode::NOT_FOUND {
    return get_latest_tag_info(client, current_version);
  }

  if !release_resp.status().is_success() {
    return Err(
      format!(
        "Failed to fetch release info: HTTP {}",
        release_resp.status()
      )
      .into(),
    );
  }

  let release_info: GitHubRelease = release_resp.json()?;
  let latest_version = release_info.tag_name.trim_start_matches('v').to_string();

  let current_semver = Version::parse(current_version)?;
  let latest_semver = Version::parse(&latest_version)?;

  if latest_semver > current_semver {
    Ok(Some(VersionInfo {
      version: latest_version,
      tag_name: release_info.tag_name,
    }))
  } else {
    Ok(None)
  }
}

fn get_latest_version_info_force(
  current_version: &str,
) -> Result<VersionInfo, Box<dyn std::error::Error>> {
  let client = reqwest::blocking::Client::new();

  let release_resp = client
    .get("https://api.github.com/repos/mewisme/mew-language/releases/latest")
    .header("User-Agent", format!("Mew-Language/{}", current_version))
    .send()?;

  if release_resp.status() == reqwest::StatusCode::NOT_FOUND {
    let tags_resp = client
      .get("https://api.github.com/repos/mewisme/mew-language/tags")
      .header("User-Agent", format!("Mew-Language/{}", current_version))
      .send()?;

    if !tags_resp.status().is_success() {
      return Err(format!("Failed to fetch tags: HTTP {}", tags_resp.status()).into());
    }

    let tags: Vec<GitHubTag> = tags_resp.json()?;

    if tags.is_empty() {
      return Err("No releases or tags found".into());
    }

    let tag = &tags[0];
    return Ok(VersionInfo {
      version: tag.name.trim_start_matches('v').to_string(),
      tag_name: tag.name.clone(),
    });
  }

  if !release_resp.status().is_success() {
    return Err(
      format!(
        "Failed to fetch release info: HTTP {}",
        release_resp.status()
      )
      .into(),
    );
  }

  let release_info: GitHubRelease = release_resp.json()?;
  let latest_version = release_info.tag_name.trim_start_matches('v').to_string();

  Ok(VersionInfo {
    version: latest_version,
    tag_name: release_info.tag_name,
  })
}

fn get_latest_tag_info(
  client: reqwest::blocking::Client,
  current_version: &str,
) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
  let tags_resp = client
    .get("https://api.github.com/repos/mewisme/mew-language/tags")
    .header("User-Agent", format!("Mew-Language/{}", current_version))
    .send()?;

  if !tags_resp.status().is_success() {
    return Err(format!("Failed to fetch tags: HTTP {}", tags_resp.status()).into());
  }

  let tags: Vec<GitHubTag> = tags_resp.json()?;

  if tags.is_empty() {
    return Ok(None);
  }

  let latest_tag = &tags[0].name;
  let latest_version = latest_tag.trim_start_matches('v').to_string();

  let current_semver = Version::parse(current_version)?;
  match Version::parse(&latest_version) {
    Ok(latest_semver) => {
      if latest_semver > current_semver {
        Ok(Some(VersionInfo {
          version: latest_version,
          tag_name: latest_tag.clone(),
        }))
      } else {
        Ok(None)
      }
    }
    Err(_) => Ok(None),
  }
}

fn get_platform_download_url(tag_name: &str) -> Result<String, Box<dyn std::error::Error>> {
  let base_url = format!(
    "https://github.com/mewisme/mew-language/releases/download/{}",
    tag_name
  );

  let file_name = if cfg!(target_os = "windows") {
    "mew-windows-x86_64.zip"
  } else if cfg!(target_os = "macos") {
    "mew-macos-x86_64.zip"
  } else if cfg!(target_os = "linux") {
    "mew-linux-x86_64.zip"
  } else {
    return Err(Box::new(std::io::Error::new(
      std::io::ErrorKind::Unsupported,
      format!("Unsupported platform"),
    )));
  };

  Ok(format!("{}/{}", base_url, file_name))
}

fn download_with_retry(
  url: &str,
  max_retries: u8,
) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
  let mut retries = 0;
  let mut last_error: Option<Box<dyn std::error::Error>> = None;

  while retries < max_retries {
    match reqwest::blocking::get(url) {
      Ok(response) => {
        if response.status().is_success() {
          return match response.bytes() {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
              println!("Error reading response body: {}", e);
              last_error = Some(Box::new(e));
              retries += 1;
              if retries < max_retries {
                println!("Retrying download ({}/{})", retries, max_retries);
                std::thread::sleep(std::time::Duration::from_secs(1));
              }
              continue;
            }
          };
        } else {
          println!("HTTP error: {}", response.status());
          last_error = Some(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP error: {}", response.status()),
          )));
          retries += 1;
          if retries < max_retries {
            println!("Retrying download ({}/{})", retries, max_retries);
            std::thread::sleep(std::time::Duration::from_secs(1));
          }
        }
      }
      Err(e) => {
        println!("Download error: {}", e);
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, e.to_string());
        last_error = Some(Box::new(io_error));
        retries += 1;
        if retries < max_retries {
          println!("Retrying download ({}/{})", retries, max_retries);
          std::thread::sleep(std::time::Duration::from_secs(1));
        }
      }
    }
  }

  Err(last_error.unwrap_or_else(|| {
    Box::new(std::io::Error::new(
      std::io::ErrorKind::Other,
      "Failed to download file after multiple attempts",
    ))
  }))
}

pub fn check_for_updates(
  current_version: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
  let client = reqwest::blocking::Client::new();

  let release_resp = client
    .get("https://api.github.com/repos/mewisme/mew-language/releases/latest")
    .header("User-Agent", format!("Mew-Language/{}", current_version))
    .send()?;

  if release_resp.status() == reqwest::StatusCode::NOT_FOUND {
    let version_info = get_latest_tag_info(client, current_version)?;
    return Ok(version_info.map(|info| info.version));
  }

  if !release_resp.status().is_success() {
    return Err(
      format!(
        "Failed to fetch release info: HTTP {}",
        release_resp.status()
      )
      .into(),
    );
  }

  let release_info: GitHubRelease = release_resp.json()?;
  let latest_version = release_info.tag_name.trim_start_matches('v').to_string();

  let current_semver = Version::parse(current_version)?;
  let latest_semver = Version::parse(&latest_version)?;

  if latest_semver > current_semver {
    Ok(Some(latest_version))
  } else {
    Ok(None)
  }
}
