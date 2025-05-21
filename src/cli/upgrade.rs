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
use std::process;

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
}

pub fn handle_upgrade(force: bool) -> Result<(), Box<dyn std::error::Error>> {
  let current_version = env!("CARGO_PKG_VERSION");

  match get_latest_version_info(current_version)? {
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

  #[cfg(target_os = "windows")]
  {
    let mut command = process::Command::new("powershell");
    command.arg("-c");
    command.arg(format!("irm mewis.me/install.ps1 | iex"));
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
    let mut command = process::Command::new("bash");
    command.arg("-c");
    command.arg(format!("curl -fsSL https://mewis.me/install.sh | bash"));
    let output = command.status()?;

    if !output.success() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Installation failed with exit code: {:?}", output.code()),
      )));
    }

    return Ok(());
  }
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
        }))
      } else {
        Ok(None)
      }
    }
    Err(_) => Ok(None),
  }
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
