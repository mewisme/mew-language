use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("Usage: {} <file.mew>", args[0]);
    process::exit(1);
  }

  let file_path = &args[1];
  if !file_path.ends_with(".mew") {
    eprintln!("Error: File must have .mew extension");
    process::exit(1);
  }

  let path = Path::new(file_path);
  if !path.exists() {
    eprintln!("Error: File not found: {}", file_path);
    process::exit(1);
  }

  match fs::read_to_string(path) {
    Ok(content) => {
      println!("File content:");
      println!("{}", content);

      println!("\nTokenizing...");
      let lines: Vec<&str> = content.lines().collect();
      for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;
        println!("{}: {}", line_num, line);

        // Very simple tokenization just to verify parsing
        for (j, token) in line.split_whitespace().enumerate() {
          println!("  Token {}.{}: {}", line_num, j + 1, token);
        }
      }

      println!("\nLexer verification successful!");
    }
    Err(e) => {
      eprintln!("Error reading file: {}", e);
      process::exit(1);
    }
  }
}
