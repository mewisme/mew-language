use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::error::MewResult;

pub fn handle_init(provided_name: Option<String>) -> MewResult<()> {
  let project_name = if let Some(name) = provided_name {
    name
  } else {
    print!("🐱 Enter project name (default: mew): ");
    io::stdout().flush()?;

    #[cfg(windows)]
    {
      io::stderr().flush()?;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let project_name = input.trim();

    if project_name.is_empty() {
      "mew".to_string()
    } else {
      project_name.to_string()
    }
  };

  let project_dir = Path::new(&project_name);
  if project_dir.exists() {
    eprintln!("hiss! Directory '{}' already exists", project_name);
    return Ok(());
  }

  fs::create_dir(project_dir)?;
  let src_dir = project_dir.join("src");
  fs::create_dir(&src_dir)?;
  let main_file = src_dir.join("main.mew");
  fs::write(main_file, "purr(\"Welcome to Mew Programming Language!\")")?;

  let toml_content = format!(
    "[package]\n\
     name = \"{}\"\n\
     version = \"0.1.0\"\n\
     description = \"A Mew language project\"\n\
     author = \"\"\n\
     start = \"src/main.mew\"",
    project_name
  );
  let config_file = project_dir.join("mew.toml");
  fs::write(config_file, toml_content)?;

  println!("🐱 Created new Mew project: {}", project_name);
  println!("To run your project:");
  println!("  cd {}", project_name);
  println!("  mew start");

  Ok(())
}
