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

mod error;
mod interpreter;
mod lexer;
mod parser;
mod value;

use clap::{Parser, Subcommand};
use error::MewResult;
use rustyline::error::ReadlineError;
use rustyline::Editor as DefaultEditor;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(name = "mew")]
#[command(about = "A cat-themed programming language", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// Runs a .mew file
  Run {
    /// The path to the .mew file
    file_path: String,
  },
  /// Get the version of Mew
  Version,
}

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Some(Commands::Run { file_path }) => {
      if let Err(e) = run_file(file_path) {
        eprintln!("hiss! Error: {}", e);
        process::exit(1);
      }
    }
    Some(Commands::Version) => {
      let version = env!("CARGO_PKG_VERSION");
      println!("Mew Language v{}", version);
    }
    None => {
      if let Err(e) = run_repl() {
        eprintln!("hiss! Error: {}", e);
        process::exit(1);
      }
    }
  }
}

fn run_file(file_path: &str) -> error::MewResult<()> {
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
    Ok(_) => {
      // No need to print anything on successful execution
      Ok(())
    }
    Err(e) => {
      eprintln!("hiss! Error: {}", e);
      process::exit(1);
    }
  }
}

fn run_repl() -> MewResult<()> {
  println!("🐱 Mew Programming Language v{}", env!("CARGO_PKG_VERSION"));
  println!("Type 'exit' or press Ctrl+C to exit");

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
          Ok(value) => println!("{}", value),
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
