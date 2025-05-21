use crate::error::MewResult;
use crate::interpreter;
use crate::value;
use rustyline::error::ReadlineError;
use rustyline::Editor as DefaultEditor;
use std::fs;
use std::path::Path;
use std::process;

pub fn run_file(file_path: &str) -> crate::error::MewResult<()> {
  if !file_path.ends_with(".mew") {
    eprintln!("hiss! File must have .mew extension");
    process::exit(1);
  }

  let path = Path::new(file_path);
  if !path.exists() {
    eprintln!("hiss! File not found: {}", file_path);
    process::exit(1);
  }

  let content = fs::read_to_string(path)?;
  match interpreter::interpret(&content) {
    Ok(_) => Ok(()),
    Err(e) => {
      eprintln!("hiss! Error: {}", e);
      process::exit(1);
    }
  }
}

pub fn run_repl() -> MewResult<()> {
  println!("🐱 Mew Programming Language v{}", env!("CARGO_PKG_VERSION"));
  println!("\nType 'exit' or press Ctrl+C to exit");

  let mut rl = DefaultEditor::new().unwrap();
  let helper = ();
  rl.set_helper(Some(helper));

  loop {
    let readline = rl.readline("🐾 > ");
    match readline {
      Ok(line) => {
        if line.trim() == "exit" {
          break;
        }

        let _ = rl.add_history_entry(line.as_str());

        match interpreter::interpret(&line) {
          Ok(value) => {
            if !matches!(value, value::Value::Undefined) {
              println!("{}", value);
            }
          }
          Err(e) => eprintln!("hiss! {}", e),
        }
      }
      Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
        println!("Goodbye!");
        break;
      }
      Err(err) => {
        eprintln!("hiss! Error: {}", err);
        break;
      }
    }
  }

  Ok(())
}
