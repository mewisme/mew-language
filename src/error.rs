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
  #[error("Syntax error: {0}")]
  Syntax(String),

  #[error("Runtime error: {0}")]
  Runtime(String),

  #[error("Type error: {0}")]
  Type(String),

  #[error("Name error: {0}")]
  Name(String),

  #[error("IO error: {0}")]
  IO(#[from] std::io::Error),
}

impl MewError {
  pub fn syntax<T: Into<String>>(message: T) -> Self {
    MewError::Syntax(message.into())
  }

  pub fn runtime<T: Into<String>>(message: T) -> Self {
    MewError::Runtime(message.into())
  }

  pub fn type_error<T: Into<String>>(message: T) -> Self {
    MewError::Type(message.into())
  }

  pub fn name<T: Into<String>>(message: T) -> Self {
    MewError::Name(message.into())
  }
}

pub type MewResult<T> = Result<T, MewError>;

// Location information for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
  pub line: usize,
  pub column: usize,
}

impl Location {
  pub fn new(line: usize, column: usize) -> Self {
    Self { line, column }
  }
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "line {}, column {}", self.line, self.column)
  }
}
