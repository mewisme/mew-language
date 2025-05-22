use crate::error::MewResult;
use crate::interpreter;
use crate::value;
use rustyline::error::ReadlineError;
use rustyline::Editor as DefaultEditor;
use std::fs;
use std::path::Path;
use std::process;

pub fn run_file(file_path: &str) -> crate::error::MewResult<()> {
  // Check file extension
  if !file_path.ends_with(".mew") {
    eprintln!("hiss! File must have .mew extension");
    process::exit(1);
  }

  // Check if file exists
  let path = Path::new(file_path);
  if !path.exists() {
    eprintln!("hiss! File not found: {}", file_path);
    process::exit(1);
  }

  // Read file content
  let content = fs::read_to_string(path)?;
  
  // Interpret the file
  match interpreter::interpret(&content) {
    Ok(_) => Ok(()),
    Err(e) => {
      eprintln!("hiss! Error: {}", e);
      
      // If we have a location, show the relevant line of code
      if let Some(location) = e.location() {
        if location.line > 0 {
          let lines: Vec<&str> = content.lines().collect();
          if location.line <= lines.len() {
            let line_content = lines[location.line - 1];
            eprintln!("\n{}", line_content);
            // Print a caret pointing to the error position
            if location.column > 0 {
              let pointer = " ".repeat(location.column - 1) + "^";
              eprintln!("{}", pointer);
            }
          }
        }
      }
      
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

  // Initialize a persistent state for the REPL
  let mut persistent_state = String::new();

  loop {
    let readline = rl.readline("🐾 > ");
    match readline {
      Ok(line) => {
        if line.trim() == "exit" || line.trim() == "quit" || line.trim() == "bye" || line.trim() == "q" {
          println!("Goodbye!");
          break;
        }

        let _ = rl.add_history_entry(line.as_str());

        let trimmed_line = line.trim();
        
        // Try to directly interpret as a literal value or variable name
        let is_simple_value = 
            // Check if it's a number
            trimmed_line.parse::<f64>().is_ok() || 
            // Check if it's a string in quotes
            (trimmed_line.starts_with('"') && trimmed_line.ends_with('"')) ||
            (trimmed_line.starts_with('\'') && trimmed_line.ends_with('\'')) ||
            // Check for special keywords
            trimmed_line == "true" || 
            trimmed_line == "false" || 
            trimmed_line == "null" || 
            trimmed_line == "undefined" || 
            trimmed_line == "NaN" || 
            trimmed_line == "Infinity";

        // If it looks like a literal value/variable and there's no semicolon, add one
        let mut line_to_interpret = if is_simple_value {
            format!("{};", trimmed_line)
        } else {
            line.clone()
        };
        
        if !line_to_interpret.ends_with(';') {
            line_to_interpret.push(';');
        };

        // Add current line to the persistent state
        persistent_state.push_str(&line_to_interpret);
        persistent_state.push('\n');

        // Interpret the accumulated code
        match interpreter::interpret(&persistent_state) {
          Ok(value) => {
            // Only print the return value if it's not undefined and the line
            // wasn't already a print statement (to avoid double printing)
            if !matches!(value, value::Value::Undefined) {
              println!("{}", value);
              // Only remove the last line if it was a simple value expression, not modifying code
              if is_simple_value {
                // Remove the last line from persistent_state
                let mut lines: Vec<&str> = persistent_state.lines().collect();
                if !lines.is_empty() {
                  lines.pop();
                  persistent_state = lines.join("\n");
                  if !persistent_state.is_empty() {
                    persistent_state.push('\n');
                  }
                }
              }
            }
          }
          Err(e) => {
            eprintln!("hiss! {}", e);
            
            // If there's a location, show the relevant line of code
            if let Some(location) = e.location() {
              if location.line > 0 {
                let lines: Vec<&str> = persistent_state.lines().collect();
                if location.line <= lines.len() {
                  let line_content = lines[location.line - 1];
                  eprintln!("\n{}", line_content);
                  // Print a caret pointing to the error position
                  if location.column > 0 {
                    let pointer = " ".repeat(location.column - 1) + "^";
                    eprintln!("{}", pointer);
                  }
                }
              }
            }
            
            // If there's an error, remove the last line we added to prevent carrying forward errors
            // Split by lines, remove the last entry, and then rejoin
            let mut lines: Vec<&str> = persistent_state.lines().collect();
            if !lines.is_empty() {
              lines.pop();
              persistent_state = lines.join("\n");
              if !persistent_state.is_empty() {
                persistent_state.push('\n');
              }
            }
          }
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
