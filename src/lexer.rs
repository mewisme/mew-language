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

use crate::error::{Location, MewError, MewResult};
use std::fmt;
use std::str::FromStr;

/// TokenKind defines all the possible token types for our language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  // Keywords
  Const,
  Let,
  Var,
  If,
  ElseIf,
  Else,
  For,
  While,
  Do,
  Break,
  Continue,
  Switch,
  Case,
  Default,
  Function,
  In,
  Of,
  Return,
  Print,
  Public,
  Import,
  From,

  // Literals
  Number(f64),
  Boolean(bool),
  String(String),
  Null,
  Undefined,
  NaN,
  Infinity,

  // Identifiers
  Identifier(String),

  // Operators
  Plus,
  Minus,
  Star,
  Slash,
  Percent,
  EqualEqual,
  BangEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  And,
  Or,
  Bang,
  Equal,
  PlusEqual,
  MinusEqual,
  StarEqual,
  SlashEqual,
  PercentEqual,
  Increment,
  Decrement,

  // Punctuation
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Comma,
  Dot,
  Semicolon,
  Colon,
  Arrow,

  // End of file
  Eof,
}

/// A token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub location: Location,
}

impl Token {
  pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
    Self {
      kind,
      lexeme,
      location: Location::new(line, column),
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?} '{}' at {}", self.kind, self.lexeme, self.location)
  }
}

/// MewLexer processes source code into tokens
pub struct MewLexer {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
  column: usize,
}

impl MewLexer {
  pub fn new(source: &str) -> Self {
    Self {
      source: source.to_string(),
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
      column: 1,
    }
  }

  pub fn scan_tokens(&mut self) -> MewResult<Vec<Token>> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token()?;
    }

    // Add EOF token
    self.tokens.push(Token::new(
      TokenKind::Eof,
      "".to_string(),
      self.line,
      self.column,
    ));

    Ok(self.tokens.clone())
  }

  fn scan_token(&mut self) -> MewResult<()> {
    let c = self.advance();

    match c {
      // Single-character tokens
      '(' => self.add_token(TokenKind::LeftParen),
      ')' => self.add_token(TokenKind::RightParen),
      '{' => self.add_token(TokenKind::LeftBrace),
      '}' => self.add_token(TokenKind::RightBrace),
      '[' => self.add_token(TokenKind::LeftBracket),
      ']' => self.add_token(TokenKind::RightBracket),
      ',' => self.add_token(TokenKind::Comma),
      '.' => self.add_token(TokenKind::Dot),
      ';' => self.add_token(TokenKind::Semicolon),
      ':' => self.add_token(TokenKind::Colon),

      // Operators that might be two characters
      '+' => {
        if self.match_char('=') {
          self.add_token(TokenKind::PlusEqual)
        } else if self.match_char('+') {
          self.add_token(TokenKind::Increment)
        } else {
          self.add_token(TokenKind::Plus)
        }
      }
      '-' => {
        if self.match_char('=') {
          self.add_token(TokenKind::MinusEqual)
        } else if self.match_char('-') {
          self.add_token(TokenKind::Decrement)
        } else {
          self.add_token(TokenKind::Minus)
        }
      }
      '*' => {
        if self.match_char('=') {
          self.add_token(TokenKind::StarEqual)
        } else {
          self.add_token(TokenKind::Star)
        }
      }
      '/' => {
        if self.match_char('/') {
          // Comment goes until end of line
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
        } else if self.match_char('*') {
          // Multi-line comment
          loop {
            // Handle nested comments and ensure proper advancing
            if self.is_at_end() {
              return Err(MewError::syntax("Unterminated multi-line comment"));
            } else if self.peek() == '*' && self.peek_next() == '/' {
              self.advance(); // consume '*'
              self.advance(); // consume '/'
              break;
            } else if self.peek() == '\n' {
              self.line += 1;
              self.column = 1;
            }
            self.advance();
          }
        } else if self.match_char('=') {
          self.add_token(TokenKind::SlashEqual)
        } else {
          self.add_token(TokenKind::Slash)
        }
      }
      '%' => {
        if self.match_char('=') {
          self.add_token(TokenKind::PercentEqual)
        } else {
          self.add_token(TokenKind::Percent)
        }
      }
      '!' => {
        if self.match_char('=') {
          self.add_token(TokenKind::BangEqual)
        } else {
          self.add_token(TokenKind::Bang)
        }
      }
      '=' => {
        if self.match_char('=') {
          self.add_token(TokenKind::EqualEqual)
        } else if self.match_char('>') {
          self.add_token(TokenKind::Arrow)
        } else {
          self.add_token(TokenKind::Equal)
        }
      }
      '<' => {
        if self.match_char('=') {
          self.add_token(TokenKind::LessEqual)
        } else {
          self.add_token(TokenKind::Less)
        }
      }
      '>' => {
        if self.match_char('=') {
          self.add_token(TokenKind::GreaterEqual)
        } else {
          self.add_token(TokenKind::Greater)
        }
      }
      '&' => {
        if self.match_char('&') {
          self.add_token(TokenKind::And)
        } else {
          return Err(MewError::syntax(format!(
            "Unexpected character '&' at line {}, column {}",
            self.line,
            self.column - 1
          )));
        }
      }
      '|' => {
        if self.match_char('|') {
          self.add_token(TokenKind::Or)
        } else {
          return Err(MewError::syntax(format!(
            "Unexpected character '|' at line {}, column {}",
            self.line,
            self.column - 1
          )));
        }
      }

      // String literals
      '"' => self.string('"')?,
      '\'' => self.string('\'')?,

      // Whitespace
      ' ' | '\r' | '\t' | '\n' | '\x0C' => {
        if c == '\n' {
          self.line += 1;
          self.column = 1;
        }
        // Skip whitespace
      }

      // Numbers or identifiers
      _ => {
        if c.is_ascii_digit() {
          self.number()?;
        } else if c.is_ascii_alphabetic() || c == '_' {
          self.identifier()?;
        } else {
          return Err(MewError::syntax(format!(
            "Unexpected character '{}' at line {}, column {}",
            c,
            self.line,
            self.column - 1
          )));
        }
      }
    }

    Ok(())
  }

  fn string(&mut self, quote: char) -> MewResult<()> {
    let mut value = String::new();

    // Continue until we hit the closing quote
    while self.peek() != quote && !self.is_at_end() {
      let c = self.advance();

      if c == '\\' && !self.is_at_end() {
        // Handle escape sequences
        let next = self.advance();
        match next {
          'n' => value.push('\n'),
          't' => value.push('\t'),
          'r' => value.push('\r'),
          '\\' => value.push('\\'),
          '\'' => value.push('\''),
          '"' => value.push('"'),
          _ => {
            let msg = format!(
              "Invalid escape sequence at line {}, column {}",
              self.line,
              self.column - 1
            );
            return Err(MewError::syntax(msg));
          }
        }
      } else if c == '\n' {
        self.line += 1;
        self.column = 1;
        value.push(c);
      } else {
        value.push(c);
      }
    }

    if self.is_at_end() {
      return Err(MewError::syntax(format!(
        "Unterminated string at line {}, column {}",
        self.line, self.column
      )));
    }

    // Consume the closing quote
    self.advance();

    // Create the token with the string value
    let lexeme = self.source[self.start..self.current].to_string();
    self.tokens.push(Token::new(
      TokenKind::String(value),
      lexeme,
      self.line,
      self.column - (self.current - self.start),
    ));

    Ok(())
  }

  fn number(&mut self) -> MewResult<()> {
    // Consume all digits
    while self.peek().is_ascii_digit() {
      self.advance();
    }

    // Look for a decimal part
    if self.peek() == '.' && self.peek_next().is_ascii_digit() {
      // Consume the dot
      self.advance();

      // Consume the fractional part
      while self.peek().is_ascii_digit() {
        self.advance();
      }
    }

    // Convert to actual number
    let lexeme = self.source[self.start..self.current].to_string();
    let value = match f64::from_str(&lexeme) {
      Ok(v) => v,
      Err(_) => {
        return Err(MewError::syntax(format!(
          "Invalid number '{}' at line {}, column {}",
          lexeme,
          self.line,
          self.column - lexeme.len()
        )))
      }
    };

    self.tokens.push(Token::new(
      TokenKind::Number(value),
      lexeme,
      self.line,
      self.column - (self.current - self.start),
    ));

    Ok(())
  }

  fn identifier(&mut self) -> MewResult<()> {
    // Consume all alphanumeric characters and underscores
    while self.peek().is_ascii_alphanumeric() || self.peek() == '_' || self.peek() == '?' {
      self.advance();
    }

    // Check if this is a keyword
    let lexeme = self.source[self.start..self.current].to_string();
    let kind = match lexeme.as_str() {
      // Keywords
      "catst" => TokenKind::Const,
      "catlt" => TokenKind::Let,
      "catv" => TokenKind::Var,
      "meow?" => TokenKind::If,
      "meowse?" => TokenKind::ElseIf,
      "hiss" => TokenKind::Else,
      "fur" => TokenKind::For,
      "mewhile" => TokenKind::While,
      "mewdo" => TokenKind::Do,
      "clawt" => TokenKind::Break,
      "meownext" => TokenKind::Continue,
      "catwalk" => TokenKind::Switch,
      "claw" => TokenKind::Case,
      "default" => TokenKind::Default,
      "cat" => TokenKind::Function,
      "in" => TokenKind::In,
      "of" => TokenKind::Of,
      "return" => TokenKind::Return,
      "purr" => TokenKind::Print,
      "pub" => TokenKind::Public,
      "import" => TokenKind::Import,
      "from" => TokenKind::From,

      // Special literals
      "null" => TokenKind::Null,
      "undefined" => TokenKind::Undefined,
      "NaN" => TokenKind::NaN,
      "Infinity" => TokenKind::Infinity,
      "true" => TokenKind::Boolean(true),
      "false" => TokenKind::Boolean(false),

      // Identifier (not a keyword)
      _ => TokenKind::Identifier(lexeme.clone()),
    };

    self.tokens.push(Token::new(
      kind,
      lexeme,
      self.line,
      self.column - (self.current - self.start),
    ));

    Ok(())
  }

  // Helper methods

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn advance(&mut self) -> char {
    let c = self.source.chars().nth(self.current).unwrap_or('\0');
    self.current += 1;
    self.column += 1;
    c
  }

  fn match_char(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }

    if self.source.chars().nth(self.current).unwrap_or('\0') != expected {
      return false;
    }

    self.current += 1;
    self.column += 1;
    true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      return '\0';
    }
    self.source.chars().nth(self.current).unwrap_or('\0')
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0';
    }
    self.source.chars().nth(self.current + 1).unwrap_or('\0')
  }

  fn add_token(&mut self, kind: TokenKind) {
    let lexeme = self.source[self.start..self.current].to_string();
    self.tokens.push(Token::new(
      kind,
      lexeme,
      self.line,
      self.column - (self.current - self.start),
    ));
  }
}
