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

use crate::error::{MewError, MewResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
  Null,
  Undefined,
  Number(f64),
  Bool(bool),
  String(String),
  Array(Vec<Value>),
  Object(HashMap<String, Value>),
  Function(Rc<Function>),
  NativeFunction(Rc<NativeFunction>),
}

#[derive(Debug, Clone)]
pub struct Function {
  pub name: Option<String>,
  pub parameters: Vec<String>,
  pub body: Vec<Rc<RefCell<Stmt>>>,
  pub closure: Rc<RefCell<Environment>>,
}

pub type NativeFunctionType = fn(Vec<Value>) -> MewResult<Value>;

pub struct NativeFunction {
  pub name: String,
  pub function: NativeFunctionType,
}

impl fmt::Debug for NativeFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "NativeFunction({})", self.name)
  }
}

impl Value {
  pub fn is_truthy(&self) -> bool {
    match self {
      Value::Null | Value::Undefined => false,
      Value::Bool(b) => *b,
      Value::Number(n) => *n != 0.0 && !n.is_nan(),
      Value::String(s) => !s.is_empty(),
      Value::Array(a) => !a.is_empty(),
      Value::Object(_) | Value::Function(_) | Value::NativeFunction(_) => true,
    }
  }

  pub fn type_name(&self) -> &'static str {
    match self {
      Value::Null => "null",
      Value::Undefined => "undefined",
      Value::Number(_) => "number",
      Value::Bool(_) => "boolean",
      Value::String(_) => "string",
      Value::Array(_) => "array",
      Value::Object(_) => "object",
      Value::Function(_) | Value::NativeFunction(_) => "function",
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Null => write!(f, "null"),
      Value::Undefined => write!(f, "undefined"),
      Value::Number(n) => {
        if n.is_nan() {
          write!(f, "NaN")
        } else if n.is_infinite() {
          if n.is_sign_positive() {
            write!(f, "Infinity")
          } else {
            write!(f, "-Infinity")
          }
        } else if n.fract() == 0.0 {
          write!(f, "{}", *n as i64)
        } else {
          write!(f, "{}", n)
        }
      }
      Value::Bool(b) => write!(f, "{}", b),
      Value::String(s) => write!(f, "{}", s),
      Value::Array(arr) => {
        write!(f, "[")?;
        for (i, val) in arr.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", val)?;
        }
        write!(f, "]")
      }
      Value::Object(obj) => {
        write!(f, "{{")?;
        for (i, (key, val)) in obj.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}: {}", key, val)?;
        }
        write!(f, "}}")
      }
      Value::Function(func) => {
        if let Some(name) = &func.name {
          write!(f, "function {}(...)", name)
        } else {
          write!(f, "function(...)")
        }
      }
      Value::NativeFunction(func) => {
        write!(f, "function {}(...) [native]", func.name)
      }
    }
  }
}

// Statement and Expression types - will be defined in detail in the parser module
#[derive(Debug, Clone)]
pub enum Stmt {
  // Statement types will be defined here
  Expression(Expr),
  Print(Expr),
  VarDeclaration(String, Option<Expr>, bool), // name, initializer, is_const
  Block(Vec<Rc<RefCell<Stmt>>>),
  If(Expr, Rc<RefCell<Stmt>>, Option<Rc<RefCell<Stmt>>>),
  While(Expr, Rc<RefCell<Stmt>>),
  Function(String, Vec<String>, Vec<Rc<RefCell<Stmt>>>),
  Return(Option<Expr>),
  Break,
  Continue,
  Switch(Expr, Vec<(Option<Expr>, Vec<Rc<RefCell<Stmt>>>)>),
}

#[derive(Debug, Clone)]
pub enum Expr {
  // Expression types will be defined here
  Literal(Value),
  Variable(String),
  Assignment(String, Box<Expr>),
  Binary(Box<Expr>, BinaryOp, Box<Expr>),
  Unary(UnaryOp, Box<Expr>),
  Call(Box<Expr>, Vec<Expr>),
  Get(Box<Expr>, String),
  Set(Box<Expr>, String, Box<Expr>),
  ArrayLiteral(Vec<Expr>),
  ObjectLiteral(Vec<(String, Expr)>),
  Function(Option<String>, Vec<String>, Vec<Rc<RefCell<Stmt>>>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Eq,
  NotEq,
  Lt,
  Lte,
  Gt,
  Gte,
  And,
  Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
  Minus,
  Not,
}

// Environment for variable scoping
#[derive(Debug, Clone)]
pub struct Environment {
  values: HashMap<String, (Value, bool)>, // (value, is_const)
  enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
      enclosing: None,
    }
  }

  pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(enclosing),
    }
  }

  pub fn define(&mut self, name: &str, value: Value, is_const: bool) {
    self.values.insert(name.to_string(), (value, is_const));
  }

  pub fn assign(&mut self, name: &str, value: Value) -> MewResult<()> {
    if let Some((ref mut val, is_const)) = self.values.get_mut(name) {
      if *is_const {
        return Err(MewError::runtime(format!(
          "Cannot reassign to constant '{}'",
          name
        )));
      }
      *val = value;
      return Ok(());
    }

    if let Some(enclosing) = &self.enclosing {
      return enclosing.borrow_mut().assign(name, value);
    }

    Err(MewError::name(format!("Undefined variable '{}'", name)))
  }

  pub fn get(&self, name: &str) -> MewResult<Value> {
    if let Some((value, _)) = self.values.get(name) {
      return Ok(value.clone());
    }

    if let Some(enclosing) = &self.enclosing {
      return enclosing.borrow().get(name);
    }

    Err(MewError::name(format!("Undefined variable '{}'", name)))
  }
}
