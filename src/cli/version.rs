use std::thread;
use std::time::Duration;

use crate::cli::upgrade;

pub fn handle_version() {
  let version = env!("CARGO_PKG_VERSION");
  println!("🐱 Mew Programming Language v{}", version);

  let version_clone = version.to_string();
  thread::spawn(move || match upgrade::check_for_updates(&version_clone) {
    Ok(Some(latest_version)) => {
      println!("A new version is available: v{}", latest_version);
      if cfg!(target_os = "windows") {
        println!(
          "To update, run command: powershell -c \"irm https://mewis.me/install.ps1 | iex\""
        );
      } else {
        println!("To update, run command: curl -s https://mewis.me/install.sh | bash");
      }
    }
    Ok(None) => {
      println!("You are running the latest version");
    }
    Err(err) => {
      println!("Failed to check for updates: {}", err);
    }
  });

  thread::sleep(Duration::from_millis(100));
}
