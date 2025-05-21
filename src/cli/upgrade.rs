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
use std::{process, time::Duration};

#[derive(Deserialize)]
struct GitHubRelease {
  tag_name: String,
}

fn create_api_client() -> Result<reqwest::blocking::Client, reqwest::Error> {
  let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(30))
    .user_agent(format!(
      "mew-language-compiler/{}",
      env!("CARGO_PKG_VERSION")
    ))
    .danger_accept_invalid_certs(false) // Don't accept invalid certs
    .https_only(true) // Force HTTPS
    .tcp_keepalive(Some(Duration::from_secs(60)))
    .connection_verbose(true)
    .build()?;

  Ok(client)
}

fn get_latest_version() -> Result<GitHubRelease, Box<dyn std::error::Error>> {
  let client = create_api_client()?;

  let release_resp = client
    .get("https://api.github.com/repos/mewisme/mew-language/releases/latest")
    .send()?;

  if !release_resp.status().is_success() {
    return Err(
      format!(
        "Failed to fetch release info: HTTP {}",
        release_resp.status()
      )
      .into(),
    );
  }

  Ok(release_resp.json()?)
}

pub fn handle_upgrade(force: bool) -> Result<(), Box<dyn std::error::Error>> {
  let current_version = env!("CARGO_PKG_VERSION");

  let release_info = match get_latest_version() {
    Ok(info) => info,
    Err(e) => {
      println!("Error checking for updates: {}", e);
      if force {
        return Err(format!("Failed to check for updates: {}", e).into());
      } else {
        println!("Continuing with current version (v{}).", current_version);
        return Ok(());
      }
    }
  };

  let latest_version = release_info.tag_name.trim_start_matches('v').to_string();
  let current_semver = Version::parse(current_version)?;
  let latest_semver = Version::parse(&latest_version)?;

  if latest_semver <= current_semver && !force {
    println!(
      "You are already running the latest version (v{}).",
      current_version
    );
    return Ok(());
  }

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

pub fn check_for_updates(
  current_version: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
  let release_info = match get_latest_version() {
    Ok(release_info) => release_info,
    Err(e) => {
      println!("Error connecting to GitHub: {}", e);
      return Err(format!("Failed to connect to GitHub: {}", e).into());
    }
  };

  let latest_version = release_info.tag_name.trim_start_matches('v').to_string();

  let current_semver = Version::parse(current_version)?;
  let latest_semver = Version::parse(&latest_version)?;

  if latest_semver > current_semver {
    Ok(Some(latest_version))
  } else {
    Ok(None)
  }
}
