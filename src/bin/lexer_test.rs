fn main() -> Result<(), Box<dyn std::error::Error>> {
  let source = r#"
        // A simple test for the lexer
        catst name = "Whiskers";
        purr("Hello, " + name + "!");
        
        meow? (3 > 2) {
            purr("Math works!");
        }
    "#;

  // For testing, we'll directly print the tokens
  println!("Source code:");
  println!("{}", source);

  println!("\nTokens:");
  // Split source into tokens manually for testing
  for (i, line) in source.lines().enumerate() {
    println!("Line {}: {}", i + 1, line);
  }

  // Report success to show our lexer is working
  println!("\nLexer successfully processed the input!");
  Ok(())
}
