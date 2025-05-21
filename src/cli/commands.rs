use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mew")]
#[command(about = "A cat-themed programming language", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Runs a .mew file
  Run {
    /// The path to the .mew file
    file_path: String,
  },
  /// Get the version of Mew
  Version,
  /// Upgrade Mew to the latest version
  Upgrade {
    /// Force upgrade even if already on the latest version
    #[arg(short, long)]
    force: bool,
  },
  /// Initialize a new Mew project
  Init {
    /// Optional project name (skips the prompt)
    name: Option<String>,
  },
  /// Run the start script defined in mew.toml
  Start,
}
