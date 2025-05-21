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

mod cli;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod value;

use clap::Parser;
use cli::{Cli, Commands};
use std::process;

fn main() {
  let cli = Cli::parse();

  if let Some(file_path) = cli.file_path {
    if let Err(e) = cli::run_file(&file_path) {
      eprintln!("hiss! Error: {}", e);
      process::exit(1);
    }
    return;
  }

  match &cli.command {
    Some(Commands::Version) => {
      cli::handle_version();
    }
    Some(Commands::Upgrade { force }) => {
      let _ = cli::handle_upgrade(*force);
    }
    Some(Commands::Init { name }) => {
      if let Err(e) = cli::handle_init(name.clone()) {
        eprintln!("hiss! Error: {}", e);
        process::exit(1);
      }
    }
    Some(Commands::Start) => {
      if let Err(e) = cli::handle_start() {
        eprintln!("hiss! Error: {}", e);
        process::exit(1);
      }
    }
    None => {
      if let Err(e) = cli::run_repl() {
        eprintln!("hiss! Error: {}", e);
        process::exit(1);
      }
    }
  }
}
