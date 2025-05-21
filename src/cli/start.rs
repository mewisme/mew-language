use std::fs;
use std::path::Path;

use crate::cli::run_file;
use crate::error::{MewError, MewResult};

pub fn handle_start() -> MewResult<()> {
  let config_path = Path::new("mew.toml");
  if !config_path.exists() {
    return Err(MewError::runtime(format!(
      "Could not find mew.toml in current directory"
    )));
  }

  let config_content = fs::read_to_string(config_path)?;

  let start_path = extract_start_path(&config_content)
    .ok_or_else(|| MewError::runtime("Start path not defined in mew.toml"))?;

  run_file(&start_path)
}

fn extract_start_path(toml_content: &str) -> Option<String> {
  for line in toml_content.lines() {
    let line = line.trim();
    if line.starts_with("start") {
      let parts: Vec<&str> = line.split('=').collect();
      if parts.len() == 2 {
        let value = parts[1].trim();
        let value = value.trim_matches('"').trim_matches('\'');
        let full_path = Path::new(value).to_str().unwrap().to_string();
        println!("🐱 Starting project from: {}", full_path);
        return Some(full_path);
      }
    }
  }
  None
}
