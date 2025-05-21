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

use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MewError {
  #[error("Syntax error at {1}: {0}")]
  Syntax(String, Location),

  #[error("Runtime error at {1}: {0}")]
  Runtime(String, Location),

  #[error("Type error at {1}: {0}")]
  Type(String, Location),

  #[error("Name error at {1}: {0}")]
  Name(String, Location),

  #[error("IO error: {0}")]
  IO(#[from] std::io::Error),
}

impl MewError {
  pub fn syntax<T: Into<String>>(message: T) -> Self {
    MewError::Syntax(message.into(), Location::unknown())
  }

  pub fn syntax_at<T: Into<String>>(message: T, location: Location) -> Self {
    MewError::Syntax(message.into(), location)
  }

  pub fn runtime<T: Into<String>>(message: T) -> Self {
    MewError::Runtime(message.into(), Location::unknown())
  }

  #[allow(dead_code)]
  pub fn runtime_at<T: Into<String>>(message: T, location: Location) -> Self {
    MewError::Runtime(message.into(), location)
  }

  pub fn type_error<T: Into<String>>(message: T) -> Self {
    MewError::Type(message.into(), Location::unknown())
  }

  #[allow(dead_code)]
  pub fn type_error_at<T: Into<String>>(message: T, location: Location) -> Self {
    MewError::Type(message.into(), location)
  }

  pub fn name<T: Into<String>>(message: T) -> Self {
    MewError::Name(message.into(), Location::unknown())
  }

  #[allow(dead_code)]
  pub fn name_at<T: Into<String>>(message: T, location: Location) -> Self {
    MewError::Name(message.into(), location)
  }

  pub fn location(&self) -> Option<Location> {
    match self {
      MewError::Syntax(_, loc) => Some(*loc),
      MewError::Runtime(_, loc) => Some(*loc),
      MewError::Type(_, loc) => Some(*loc),
      MewError::Name(_, loc) => Some(*loc),
      MewError::IO(_) => None,
    }
  }
}

pub type MewResult<T> = Result<T, MewError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
  pub line: usize,
  pub column: usize,
}

impl Location {
  pub fn new(line: usize, column: usize) -> Self {
    Self { line, column }
  }

  pub fn unknown() -> Self {
    Self { line: 0, column: 0 }
  }
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.line == 0 && self.column == 0 {
      write!(f, "unknown location")
    } else {
      write!(f, "line {}, column {}", self.line, self.column)
    }
  }
}
