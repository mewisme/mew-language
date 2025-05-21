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
use crate::value::{BinaryOp, Environment, Expr, Function, NativeFunction, Stmt, UnaryOp, Value};
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
  environment: Rc<RefCell<Environment>>,
  globals: Rc<RefCell<Environment>>,
}

impl Interpreter {
  pub fn new() -> Self {
    let globals = Rc::new(RefCell::new(Environment::new()));

    // Define native functions
    let mut interp = Self {
      globals: globals.clone(),
      environment: globals.clone(),
    };

    interp.define_native_functions();

    interp
  }

  fn define_native_functions(&mut self) {
    // Native print function
    self.globals.borrow_mut().define(
      "print",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "print".to_string(),
        function: Self::native_print,
      })),
      true,
    );

    // Define toString for all value types
    self.globals.borrow_mut().define(
      "toString",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "toString".to_string(),
        function: Self::native_to_string,
      })),
      true,
    );

    // Define purr as alias for print (cat-themed print function)
    self.globals.borrow_mut().define(
      "purr",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "purr".to_string(),
        function: Self::native_print,
      })),
      true,
    );

    // Create Object global with static methods
    let mut object_methods = HashMap::new();

    // Add Object.keys method
    object_methods.insert(
      "keys".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "keys".to_string(),
        function: Self::native_object_keys,
      })),
    );

    // Add Object.values method
    object_methods.insert(
      "values".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "values".to_string(),
        function: Self::native_object_values,
      })),
    );

    // Define the Object global
    self
      .globals
      .borrow_mut()
      .define("Object", Value::Object(object_methods), true);

    // Native time function
    self.globals.borrow_mut().define(
      "time",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "time".to_string(),
        function: Self::native_time,
      })),
      true,
    );

    // Type checking functions
    self.globals.borrow_mut().define(
      "isNumber",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isNumber".to_string(),
        function: Self::native_is_number,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isString",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isString".to_string(),
        function: Self::native_is_string,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isBoolean",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isBoolean".to_string(),
        function: Self::native_is_boolean,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isNull",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isNull".to_string(),
        function: Self::native_is_null,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isUndefined",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isUndefined".to_string(),
        function: Self::native_is_undefined,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isArray",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isArray".to_string(),
        function: Self::native_is_array,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isObject",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isObject".to_string(),
        function: Self::native_is_object,
      })),
      true,
    );

    self.globals.borrow_mut().define(
      "isFunction",
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "isFunction".to_string(),
        function: Self::native_is_function,
      })),
      true,
    );

    let mut mewth_methods = HashMap::new();

    // Mewth.pounce (floor)
    mewth_methods.insert(
      "pounce".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "pounce".to_string(),
        function: Self::native_mewth_pounce,
      })),
    );

    // Mewth.leap (ceil)
    mewth_methods.insert(
      "leap".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "leap".to_string(),
        function: Self::native_mewth_leap,
      })),
    );

    // Mewth.curl (round)
    mewth_methods.insert(
      "curl".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "curl".to_string(),
        function: Self::native_mewth_curl,
      })),
    );

    // Mewth.lick (abs)
    mewth_methods.insert(
      "lick".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "lick".to_string(),
        function: Self::native_mewth_lick,
      })),
    );

    // Mewth.alpha (max)
    mewth_methods.insert(
      "alpha".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "alpha".to_string(),
        function: Self::native_mewth_alpha,
      })),
    );

    // Mewth.kitten (min)
    mewth_methods.insert(
      "kitten".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "kitten".to_string(),
        function: Self::native_mewth_kitten,
      })),
    );

    // Mewth.chase (random)
    mewth_methods.insert(
      "chase".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "chase".to_string(),
        function: Self::native_mewth_chase,
      })),
    );

    // Mewth.hiss (sqrt)
    mewth_methods.insert(
      "dig".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "dig".to_string(),
        function: Self::native_mewth_dig,
      })),
    );

    // Mewth.scratch (pow)
    mewth_methods.insert(
      "scratch".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "scratch".to_string(),
        function: Self::native_mewth_scratch,
      })),
    );

    // Mewth.tailDirection (sign)
    mewth_methods.insert(
      "tailDirection".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "tailDirection".to_string(),
        function: Self::native_mewth_tail_direction,
      })),
    );

    // Add Mewth.PI constant (equivalent to Math.PI)
    mewth_methods.insert("PI".to_string(), Value::Number(std::f64::consts::PI));

    // Define the Mewth global object
    self
      .globals
      .borrow_mut()
      .define("Mewth", Value::Object(mewth_methods), true);

    let mut cat_time_methods = HashMap::new();

    cat_time_methods.insert(
      "now".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "now".to_string(),
        function: Self::native_cat_time_now,
      })),
    );

    cat_time_methods.insert(
      "wakeUp".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "wakeUp".to_string(),
        function: Self::native_cat_time_wake_up,
      })),
    );

    cat_time_methods.insert(
      "fullYear".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "fullYear".to_string(),
        function: Self::native_cat_time_full_year,
      })),
    );

    cat_time_methods.insert(
      "month".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "month".to_string(),
        function: Self::native_cat_time_month,
      })),
    );

    cat_time_methods.insert(
      "day".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "day".to_string(),
        function: Self::native_cat_time_day,
      })),
    );

    cat_time_methods.insert(
      "weekday".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "weekday".to_string(),
        function: Self::native_cat_time_weekday,
      })),
    );

    cat_time_methods.insert(
      "hours".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "hours".to_string(),
        function: Self::native_cat_time_hours,
      })),
    );

    cat_time_methods.insert(
      "minutes".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "minutes".to_string(),
        function: Self::native_cat_time_minutes,
      })),
    );

    cat_time_methods.insert(
      "seconds".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "seconds".to_string(),
        function: Self::native_cat_time_seconds,
      })),
    );

    cat_time_methods.insert(
      "milliseconds".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "milliseconds".to_string(),
        function: Self::native_cat_time_milliseconds,
      })),
    );

    cat_time_methods.insert(
      "toMeow".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "toMeow".to_string(),
        function: Self::native_cat_time_to_meow,
      })),
    );

    self
      .globals
      .borrow_mut()
      .define("CatTime", Value::Object(cat_time_methods), true);

    // Add MewJ (JSON equivalent) object with methods
    let mut mewj_methods = HashMap::new();

    // MewJ.sniff (JSON.parse)
    mewj_methods.insert(
      "sniff".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "sniff".to_string(),
        function: Self::native_mewj_sniff,
      })),
    );

    // MewJ.mewify (JSON.stringify)
    mewj_methods.insert(
      "mewify".to_string(),
      Value::NativeFunction(Rc::new(NativeFunction {
        name: "mewify".to_string(),
        function: Self::native_mewj_mewify,
      })),
    );

    // Define the MewJ global object
    self
      .globals
      .borrow_mut()
      .define("MewJ", Value::Object(mewj_methods), true);
  }

  // Static native function implementations
  fn native_print(args: Vec<Value>) -> MewResult<Value> {
    for (i, arg) in args.iter().enumerate() {
      if i > 0 {
        print!(" ");
      }
      print!("{}", arg);
    }
    println!();
    Ok(Value::Undefined)
  }

  fn native_time(_args: Vec<Value>) -> MewResult<Value> {
    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_millis();
    Ok(Value::Number(now as f64))
  }

  fn native_is_number(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime("isNumber requires exactly one argument"));
    }
    Ok(Value::Bool(matches!(args[0], Value::Number(_))))
  }

  fn native_is_string(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime("isString requires exactly one argument"));
    }
    Ok(Value::Bool(matches!(args[0], Value::String(_))))
  }

  fn native_is_boolean(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime("isBoolean requires exactly one argument"));
    }
    Ok(Value::Bool(matches!(args[0], Value::Bool(_))))
  }

  fn native_is_null(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime("isNull requires exactly one argument"));
    }
    Ok(Value::Bool(matches!(args[0], Value::Null)))
  }

  fn native_is_undefined(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "isUndefined requires exactly one argument",
      ));
    }
    Ok(Value::Bool(matches!(args[0], Value::Undefined)))
  }

  fn native_is_array(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime("isArray requires exactly one argument"));
    }
    Ok(Value::Bool(matches!(args[0], Value::Array(_))))
  }

  fn native_is_object(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime("isObject requires exactly one argument"));
    }
    Ok(Value::Bool(matches!(args[0], Value::Object(_))))
  }

  fn native_is_function(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "isFunction requires exactly one argument",
      ));
    }
    Ok(Value::Bool(matches!(
      args[0],
      Value::Function(_) | Value::NativeFunction(_)
    )))
  }

  fn native_object_keys(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Object.keys requires exactly one argument",
      ));
    }

    match &args[0] {
      Value::Object(obj) => {
        let keys: Vec<Value> = obj.keys().map(|k| Value::String(k.clone())).collect();

        Ok(Value::Array(keys))
      }
      Value::Array(arr) => {
        let keys: Vec<Value> = (0..arr.len()).map(|i| Value::Number(i as f64)).collect();

        Ok(Value::Array(keys))
      }
      _ => Err(MewError::type_error(format!(
        "Object.keys requires an object or array, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_object_values(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Object.values requires exactly one argument",
      ));
    }

    match &args[0] {
      Value::Object(obj) => {
        let values: Vec<Value> = obj.values().cloned().collect();

        Ok(Value::Array(values))
      }
      Value::Array(arr) => Ok(Value::Array(arr.clone())),
      _ => Err(MewError::type_error(format!(
        "Object.values requires an object or array, got {}",
        args[0].type_name()
      ))),
    }
  }

  pub fn interpret(&mut self, statements: &[Rc<RefCell<Stmt>>]) -> MewResult<Value> {
    let mut result = Value::Null;

    for statement in statements {
      result = self.execute(&statement.borrow())?;
    }

    Ok(result)
  }

  fn execute(&mut self, stmt: &Stmt) -> MewResult<Value> {
    match stmt {
      Stmt::Expression(expr) => self.evaluate(expr),
      Stmt::Print(expr) => {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(Value::Undefined)
      }
      Stmt::VarDeclaration(name, initializer, is_const) => {
        let value = if let Some(expr) = initializer {
          self.evaluate(expr)?
        } else {
          Value::Undefined
        };

        self.environment.borrow_mut().define(name, value, *is_const);
        Ok(Value::Undefined)
      }
      Stmt::Block(statements) => self.execute_block(
        statements,
        Environment::with_enclosing(self.environment.clone()),
      ),
      Stmt::If(condition, then_branch, else_branch) => {
        if self.evaluate(condition)?.is_truthy() {
          self.execute(&then_branch.borrow())
        } else if let Some(else_stmt) = else_branch {
          self.execute(&else_stmt.borrow())
        } else {
          Ok(Value::Undefined)
        }
      }
      Stmt::While(condition, body) => {
        let mut result = Value::Undefined;

        while self.evaluate(condition)?.is_truthy() {
          match self.execute(&body.borrow()) {
            Ok(value) => result = value,
            Err(MewError::Runtime(msg, _)) if msg.contains("break") => break,
            Err(MewError::Runtime(msg, _)) if msg.contains("continue") => continue,
            Err(e) => return Err(e),
          }
        }

        Ok(result)
      }
      Stmt::Function(name, params, body) => {
        let function = Rc::new(Function {
          name: Some(name.clone()),
          parameters: params.clone(),
          body: body.clone(),
          closure: self.environment.clone(),
        });

        self
          .environment
          .borrow_mut()
          .define(name, Value::Function(function), false);

        Ok(Value::Undefined)
      }
      Stmt::Return(value) => {
        let return_value = if let Some(expr) = value {
          self.evaluate(expr)?
        } else {
          Value::Undefined
        };

        Err(MewError::runtime(format!("return:{}", return_value)))
      }
      Stmt::Break => Err(MewError::runtime("break")),
      Stmt::Continue => Err(MewError::runtime("continue")),
      Stmt::Switch(expr, cases) => {
        let value = self.evaluate(expr)?;
        let mut default_case = None;
        let mut _matched = false;

        for (case_value, statements) in cases {
          if let Some(case_expr) = case_value {
            let case_result = self.evaluate(case_expr)?;

            if self.is_equal(&value, &case_result) {
              _matched = true;
              let result = self.execute_statements(statements)?;
              return Ok(result);
            }
          } else {
            default_case = Some(statements);
          }
        }

        if !_matched && default_case.is_some() {
          return self.execute_statements(default_case.unwrap());
        }

        Ok(Value::Undefined)
      }
    }
  }

  fn execute_block(
    &mut self,
    statements: &[Rc<RefCell<Stmt>>],
    environment: Environment,
  ) -> MewResult<Value> {
    let previous = self.environment.clone();
    self.environment = Rc::new(RefCell::new(environment));

    let result = self.execute_statements(statements);

    self.environment = previous;

    result
  }

  fn execute_statements(&mut self, statements: &[Rc<RefCell<Stmt>>]) -> MewResult<Value> {
    let mut result = Value::Undefined;

    for statement in statements {
      match self.execute(&statement.borrow()) {
        Ok(value) => result = value,
        Err(MewError::Runtime(msg, _)) if msg.starts_with("return:") => {
          let value_str = msg.trim_start_matches("return:");

          if value_str == "undefined" {
            return Ok(Value::Undefined);
          } else if value_str == "null" {
            return Ok(Value::Null);
          } else if value_str == "true" {
            return Ok(Value::Bool(true));
          } else if value_str == "false" {
            return Ok(Value::Bool(false));
          } else if let Ok(num) = value_str.parse::<f64>() {
            return Ok(Value::Number(num));
          } else {
            let s = value_str
              .trim_start_matches('"')
              .trim_end_matches('"')
              .to_string();
            return Ok(Value::String(s));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(result)
  }

  fn evaluate(&mut self, expr: &Expr) -> MewResult<Value> {
    match expr {
      Expr::Literal(value) => Ok(value.clone()),
      Expr::Variable(name) => self.environment.borrow().get(name),
      Expr::Assignment(name, value) => {
        let value = self.evaluate(&*value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
      }
      Expr::Binary(left, op, right) => {
        let left = self.evaluate(&*left)?;
        let right = self.evaluate(&*right)?;

        match (op, &left, &right) {
          (BinaryOp::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
          (BinaryOp::Sub, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
          (BinaryOp::Mul, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
          (BinaryOp::Div, Value::Number(a), Value::Number(b)) => {
            if *b == 0.0 {
              Ok(Value::Number(f64::INFINITY))
            } else {
              Ok(Value::Number(a / b))
            }
          }
          (BinaryOp::Mod, Value::Number(a), Value::Number(b)) => {
            if *b == 0.0 {
              Ok(Value::Number(f64::NAN))
            } else {
              Ok(Value::Number(a % b))
            }
          }

          (BinaryOp::Add, Value::String(a), _) => Ok(Value::String(format!("{}{}", a, right))),
          (BinaryOp::Add, _, Value::String(b)) => Ok(Value::String(format!("{}{}", left, b))),

          (BinaryOp::Eq, _, _) => Ok(Value::Bool(self.is_equal(&left, &right))),
          (BinaryOp::NotEq, _, _) => Ok(Value::Bool(!self.is_equal(&left, &right))),
          (BinaryOp::Lt, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
          (BinaryOp::Lte, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
          (BinaryOp::Gt, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
          (BinaryOp::Gte, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),

          (BinaryOp::And, _, _) => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
          (BinaryOp::Or, _, _) => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),

          // Type errors
          _ => Err(MewError::type_error(format!(
            "Cannot apply operator {:?} to {} and {}",
            op,
            left.type_name(),
            right.type_name()
          ))),
        }
      }
      Expr::Ternary(condition, then_expr, else_expr) => {
        let condition_value = self.evaluate(&*condition)?;
        if condition_value.is_truthy() {
          self.evaluate(&*then_expr)
        } else {
          self.evaluate(&*else_expr)
        }
      }
      Expr::Unary(op, expr) => {
        let right = self.evaluate(&*expr)?;

        match (op, &right) {
          (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
          (UnaryOp::Not, value) => match value {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Ok(Value::Bool(!right.is_truthy())),
          },
          _ => Err(MewError::type_error(format!(
            "Cannot apply operator {:?} to {}",
            op,
            right.type_name()
          ))),
        }
      }
      Expr::Call(callee, arguments) => {
        if let Expr::Get(object, _name) = &**callee {
          let object_value = self.evaluate(object)?;
          let method = self.evaluate(callee)?;

          let mut args = Vec::new();

          if let Value::NativeFunction(native) = &method {
            if native.name == "toString" {
              args.push(object_value.clone());
            }
          }

          for arg in arguments {
            args.push(self.evaluate(arg)?);
          }

          self.call_function(method, args)
        } else {
          // Normal function call (not a method call)
          let callee_value = self.evaluate(&*callee)?;

          let mut args = Vec::new();
          for arg in arguments {
            args.push(self.evaluate(arg)?);
          }

          self.call_function(callee_value, args)
        }
      }
      Expr::Get(object, name) => {
        let object_value = self.evaluate(&*object)?;

        if name == "toString" {
          return Ok(Value::NativeFunction(Rc::new(NativeFunction {
            name: "toString".to_string(),
            function: Self::native_to_string,
          })));
        }

        match object_value {
          Value::Object(obj) => {
            if name.starts_with('[') && name.ends_with(']') {
              let key_str = &name[1..name.len() - 1];

              if let Ok(expr_value) = self.evaluate(&Expr::Variable(key_str.to_string())) {
                if let Value::String(key) = expr_value {
                  if let Some(value) = obj.get(&key) {
                    Ok(value.clone())
                  } else {
                    Ok(Value::Undefined)
                  }
                } else if let Value::Number(num) = expr_value {
                  let key = num.to_string();
                  if let Some(value) = obj.get(&key) {
                    Ok(value.clone())
                  } else {
                    Ok(Value::Undefined)
                  }
                } else {
                  Err(MewError::type_error(format!(
                    "Object property name must be a string or number, got: {}",
                    expr_value.type_name()
                  )))
                }
              } else {
                // Try other ways to evaluate the index
                let key = key_str.to_string();
                if let Some(value) = obj.get(&key) {
                  Ok(value.clone())
                } else {
                  Ok(Value::Undefined)
                }
              }
            } else if let Some(value) = obj.get(name) {
              Ok(value.clone())
            } else {
              Ok(Value::Undefined)
            }
          }
          Value::Array(arr) => {
            if name == "length" {
              Ok(Value::Number(arr.len() as f64))
            } else if let Ok(index) = name.parse::<usize>() {
              if index < arr.len() {
                Ok(arr[index].clone())
              } else {
                Err(MewError::runtime(format!("Index out of bounds: {}", index)))
              }
            } else {
              Err(MewError::type_error(format!(
                "Cannot access property '{}' of array",
                name
              )))
            }
          }
          Value::String(s) => {
            if name == "length" {
              Ok(Value::Number(s.len() as f64))
            } else {
              Err(MewError::type_error(format!(
                "Cannot access property '{}' of string",
                name
              )))
            }
          }
          _ => Err(MewError::type_error(format!(
            "Cannot access property '{}' of {}",
            name,
            object_value.type_name()
          ))),
        }
      }
      Expr::Set(object, name, value_expr) => {
        let object_value = self.evaluate(&*object)?;
        let value = self.evaluate(&*value_expr)?;

        match object_value {
          Value::Object(mut obj) => {
            obj.insert(name.clone(), value.clone());
            Ok(value)
          }
          Value::Array(mut arr) => {
            if let Ok(index) = name.parse::<usize>() {
              if index < arr.len() {
                arr[index] = value.clone();
                Ok(value)
              } else {
                Err(MewError::runtime(format!("Index out of bounds: {}", index)))
              }
            } else {
              Err(MewError::type_error(format!(
                "Cannot set property '{}' of array",
                name
              )))
            }
          }
          _ => Err(MewError::type_error(format!(
            "Cannot set property '{}' of {}",
            name,
            object_value.type_name()
          ))),
        }
      }
      Expr::ArrayLiteral(elements) => {
        let mut array = Vec::new();

        for element in elements {
          array.push(self.evaluate(element)?);
        }

        Ok(Value::Array(array))
      }
      Expr::ObjectLiteral(properties) => {
        let mut object = HashMap::new();

        for (key, expr) in properties {
          object.insert(key.clone(), self.evaluate(expr)?);
        }

        Ok(Value::Object(object))
      }
      Expr::Function(name, params, body) => {
        let function = Rc::new(Function {
          name: name.clone(),
          parameters: params.clone(),
          body: body.clone(),
          closure: self.environment.clone(),
        });

        Ok(Value::Function(function))
      }
      Expr::Increment(target, is_prefix) => {
        // Handle both prefix (++x) and postfix (x++) increment
        match &**target {
          Expr::Variable(name) => {
            let current = self.environment.borrow().get(name)?;

            if let Value::Number(n) = current {
              let new_value = Value::Number(n + 1.0);
              self
                .environment
                .borrow_mut()
                .assign(name, new_value.clone())?;

              if *is_prefix {
                // For prefix (++x), return the new value
                Ok(new_value)
              } else {
                // For postfix (x++), return the original value
                Ok(Value::Number(n))
              }
            } else {
              Err(MewError::type_error(format!(
                "Cannot increment a non-number value: {}",
                current.type_name()
              )))
            }
          }
          Expr::Get(object, property_name) => {
            let object_value = self.evaluate(object)?;

            match object_value {
              Value::Object(mut obj) => {
                if let Some(value) = obj.get(property_name) {
                  if let Value::Number(n) = value {
                    let new_value = Value::Number(n + 1.0);
                    let old_value = Value::Number(*n);
                    obj.insert(property_name.clone(), new_value.clone());

                    if *is_prefix {
                      // For prefix (++obj.prop), return the new value
                      Ok(new_value)
                    } else {
                      // For postfix (obj.prop++), return the original value
                      Ok(old_value)
                    }
                  } else {
                    Err(MewError::type_error(format!(
                      "Cannot increment a non-number property: {}",
                      property_name
                    )))
                  }
                } else {
                  Err(MewError::name(format!(
                    "Property not found: {}",
                    property_name
                  )))
                }
              }
              Value::Array(mut arr) => {
                if property_name == "length" {
                  return Err(MewError::type_error("Cannot increment array length"));
                }

                if let Ok(index) = property_name.parse::<usize>() {
                  if index < arr.len() {
                    if let Value::Number(n) = arr[index] {
                      let new_value = Value::Number(n + 1.0);
                      arr[index] = new_value.clone();

                      if *is_prefix {
                        Ok(new_value)
                      } else {
                        Ok(Value::Number(n))
                      }
                    } else {
                      Err(MewError::type_error(format!(
                        "Cannot increment a non-number array element"
                      )))
                    }
                  } else {
                    Err(MewError::runtime(format!("Index out of bounds: {}", index)))
                  }
                } else if property_name.starts_with('[') && property_name.ends_with(']') {
                  let index_str = &property_name[1..property_name.len() - 1];
                  if let Ok(expr_value) = self.evaluate(&Expr::Variable(index_str.to_string())) {
                    if let Value::Number(n) = expr_value {
                      let index = n as usize;
                      if index < arr.len() {
                        if let Value::Number(element_val) = arr[index] {
                          let new_value = Value::Number(element_val + 1.0);
                          arr[index] = new_value.clone();

                          if *is_prefix {
                            Ok(new_value)
                          } else {
                            Ok(Value::Number(element_val))
                          }
                        } else {
                          Err(MewError::type_error(
                            "Cannot increment a non-number array element",
                          ))
                        }
                      } else {
                        Err(MewError::runtime(format!("Index out of bounds: {}", index)))
                      }
                    } else {
                      Err(MewError::type_error(format!(
                        "Array index must be a number, got: {}",
                        expr_value.type_name()
                      )))
                    }
                  } else {
                    Err(MewError::type_error(format!(
                      "Invalid array index: {}",
                      index_str
                    )))
                  }
                } else {
                  Err(MewError::type_error(format!(
                    "Cannot access property '{}' of array",
                    property_name
                  )))
                }
              }
              _ => Err(MewError::type_error(format!(
                "Cannot access property of {}",
                object_value.type_name()
              ))),
            }
          }
          _ => Err(MewError::syntax("Invalid increment target")),
        }
      }
      Expr::Decrement(target, is_prefix) => {
        match &**target {
          Expr::Variable(name) => {
            let current = self.environment.borrow().get(name)?;

            if let Value::Number(n) = current {
              let new_value = Value::Number(n - 1.0);
              self
                .environment
                .borrow_mut()
                .assign(name, new_value.clone())?;

              if *is_prefix {
                Ok(new_value)
              } else {
                Ok(Value::Number(n))
              }
            } else {
              Err(MewError::type_error(format!(
                "Cannot decrement a non-number value: {}",
                current.type_name()
              )))
            }
          }
          Expr::Get(object, property_name) => {
            let object_value = self.evaluate(object)?;

            match object_value {
              Value::Object(mut obj) => {
                if let Some(value) = obj.get(property_name) {
                  if let Value::Number(n) = value {
                    let new_value = Value::Number(n - 1.0);
                    let old_value = Value::Number(*n);
                    obj.insert(property_name.clone(), new_value.clone());

                    if *is_prefix {
                      Ok(new_value)
                    } else {
                      Ok(old_value)
                    }
                  } else {
                    Err(MewError::type_error(format!(
                      "Cannot decrement a non-number property: {}",
                      property_name
                    )))
                  }
                } else {
                  Err(MewError::name(format!(
                    "Property not found: {}",
                    property_name
                  )))
                }
              }
              Value::Array(mut arr) => {
                if property_name == "length" {
                  return Err(MewError::type_error("Cannot decrement array length"));
                }

                if let Ok(index) = property_name.parse::<usize>() {
                  if index < arr.len() {
                    if let Value::Number(n) = arr[index] {
                      let new_value = Value::Number(n - 1.0);
                      arr[index] = new_value.clone();

                      if *is_prefix {
                        Ok(new_value)
                      } else {
                        Ok(Value::Number(n))
                      }
                    } else {
                      Err(MewError::type_error(format!(
                        "Cannot decrement a non-number array element"
                      )))
                    }
                  } else {
                    Err(MewError::runtime(format!("Index out of bounds: {}", index)))
                  }
                } else if property_name.starts_with('[') && property_name.ends_with(']') {
                  // Handle the dynamic array indexing case (similar to Get handling)
                  let index_str = &property_name[1..property_name.len() - 1];
                  if let Ok(expr_value) = self.evaluate(&Expr::Variable(index_str.to_string())) {
                    if let Value::Number(n) = expr_value {
                      let index = n as usize;
                      if index < arr.len() {
                        if let Value::Number(element_val) = arr[index] {
                          let new_value = Value::Number(element_val - 1.0);
                          arr[index] = new_value.clone();

                          if *is_prefix {
                            Ok(new_value)
                          } else {
                            Ok(Value::Number(element_val))
                          }
                        } else {
                          Err(MewError::type_error(
                            "Cannot decrement a non-number array element",
                          ))
                        }
                      } else {
                        Err(MewError::runtime(format!("Index out of bounds: {}", index)))
                      }
                    } else {
                      Err(MewError::type_error(format!(
                        "Array index must be a number, got: {}",
                        expr_value.type_name()
                      )))
                    }
                  } else {
                    Err(MewError::type_error(format!(
                      "Invalid array index: {}",
                      index_str
                    )))
                  }
                } else {
                  Err(MewError::type_error(format!(
                    "Cannot access property '{}' of array",
                    property_name
                  )))
                }
              }
              _ => Err(MewError::type_error(format!(
                "Cannot access property of {}",
                object_value.type_name()
              ))),
            }
          }
          _ => Err(MewError::syntax("Invalid decrement target")),
        }
      }
    }
  }

  fn call_function(&mut self, callee: Value, arguments: Vec<Value>) -> MewResult<Value> {
    match callee {
      Value::Function(function) => {
        if arguments.len() != function.parameters.len() {
          return Err(MewError::runtime(format!(
            "Expected {} arguments but got {}",
            function.parameters.len(),
            arguments.len()
          )));
        }

        let mut environment = Environment::with_enclosing(function.closure.clone());

        for (i, param) in function.parameters.iter().enumerate() {
          environment.define(param, arguments[i].clone(), false);
        }

        match self.execute_block(&function.body, environment) {
          Ok(value) => Ok(value),
          Err(MewError::Runtime(msg, _)) if msg.starts_with("return:") => {
            let value_str = msg.trim_start_matches("return:");

            if value_str == "undefined" {
              Ok(Value::Undefined)
            } else if value_str == "null" {
              Ok(Value::Null)
            } else if value_str == "true" {
              Ok(Value::Bool(true))
            } else if value_str == "false" {
              Ok(Value::Bool(false))
            } else if let Ok(num) = value_str.parse::<f64>() {
              Ok(Value::Number(num))
            } else {
              let s = value_str
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string();
              Ok(Value::String(s))
            }
          }
          Err(e) => Err(e),
        }
      }
      Value::NativeFunction(native) => (native.function)(arguments),
      _ => Err(MewError::type_error(format!(
        "Can only call functions and classes, got {}",
        callee.type_name()
      ))),
    }
  }

  fn is_equal(&self, a: &Value, b: &Value) -> bool {
    match (a, b) {
      (Value::Null, Value::Null) => true,
      (Value::Undefined, Value::Undefined) => true,
      (Value::Number(a), Value::Number(b)) => {
        if a.is_nan() || b.is_nan() {
          false
        } else {
          (a - b).abs() < f64::EPSILON
        }
      }
      (Value::Bool(a), Value::Bool(b)) => a == b,
      (Value::String(a), Value::String(b)) => a == b,
      _ => std::ptr::eq(a, b),
    }
  }

  fn native_mewth_pounce(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Mewth.pounce requires exactly one number argument",
      ));
    }

    match &args[0] {
      Value::Number(n) => Ok(Value::Number(n.floor())),
      _ => Err(MewError::runtime(format!(
        "Mewth.pounce requires a number, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_mewth_leap(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Mewth.leap requires exactly one number argument",
      ));
    }

    match &args[0] {
      Value::Number(n) => Ok(Value::Number(n.ceil())),
      _ => Err(MewError::runtime(format!(
        "Mewth.leap requires a number, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_mewth_curl(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Mewth.curl requires exactly one number argument",
      ));
    }

    match &args[0] {
      Value::Number(n) => Ok(Value::Number(n.round())),
      _ => Err(MewError::runtime(format!(
        "Mewth.curl requires a number, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_mewth_lick(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Mewth.lick requires exactly one number argument",
      ));
    }

    match &args[0] {
      Value::Number(n) => Ok(Value::Number(n.abs())),
      _ => Err(MewError::runtime(format!(
        "Mewth.lick requires a number, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_mewth_alpha(args: Vec<Value>) -> MewResult<Value> {
    if args.is_empty() {
      return Err(MewError::runtime(
        "Mewth.alpha requires at least one number argument",
      ));
    }

    let mut max = f64::NEG_INFINITY;

    for arg in args {
      match arg {
        Value::Number(n) => {
          if n > max {
            max = n;
          }
        }
        _ => {
          return Err(MewError::runtime(format!(
            "Mewth.alpha requires numbers, got {}",
            arg.type_name()
          )))
        }
      }
    }

    Ok(Value::Number(max))
  }

  fn native_mewth_kitten(args: Vec<Value>) -> MewResult<Value> {
    if args.is_empty() {
      return Err(MewError::runtime(
        "Mewth.kitten requires at least one number argument",
      ));
    }

    let mut min = f64::INFINITY;

    for arg in args {
      match arg {
        Value::Number(n) => {
          if n < min {
            min = n;
          }
        }
        _ => {
          return Err(MewError::runtime(format!(
            "Mewth.kitten requires numbers, got {}",
            arg.type_name()
          )))
        }
      }
    }

    Ok(Value::Number(min))
  }

  fn native_mewth_chase(args: Vec<Value>) -> MewResult<Value> {
    if !args.is_empty() {
      return Err(MewError::runtime("Mewth.chase doesn't take any arguments"));
    }

    use rand::Rng;
    let mut rng = rand::rng();
    Ok(Value::Number(rng.random::<f64>()))
  }

  fn native_mewth_dig(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Mewth.dig requires exactly one number argument",
      ));
    }

    match &args[0] {
      Value::Number(n) => {
        if *n < 0.0 {
          return Err(MewError::runtime(
            "Cannot compute square root of negative number",
          ));
        }
        Ok(Value::Number(n.sqrt()))
      }
      _ => Err(MewError::runtime(format!(
        "Mewth.dig requires a number, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_mewth_scratch(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 2 {
      return Err(MewError::runtime(
        "Mewth.scratch requires exactly two number arguments",
      ));
    }

    match (&args[0], &args[1]) {
      (Value::Number(base), Value::Number(exponent)) => Ok(Value::Number(base.powf(*exponent))),
      _ => Err(MewError::runtime(format!(
        "Mewth.scratch requires two numbers, got {} and {}",
        args[0].type_name(),
        args[1].type_name()
      ))),
    }
  }

  fn native_mewth_tail_direction(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Mewth.tailDirection requires exactly one number argument",
      ));
    }

    match &args[0] {
      Value::Number(n) => {
        let sign = if *n > 0.0 {
          1.0
        } else if *n < 0.0 {
          -1.0
        } else {
          0.0
        };
        Ok(Value::Number(sign))
      }
      _ => Err(MewError::runtime(format!(
        "Mewth.tailDirection requires a number, got {}",
        args[0].type_name()
      ))),
    }
  }

  // CatTime native functions
  fn native_cat_time_now(_args: Vec<Value>) -> MewResult<Value> {
    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_millis();
    Ok(Value::Number(now as f64))
  }

  fn native_cat_time_wake_up(_args: Vec<Value>) -> MewResult<Value> {
    // Create a date object with current timestamp
    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_millis();

    // Create a date object
    let mut date_obj = HashMap::new();
    date_obj.insert("_timestamp".to_string(), Value::Number(now as f64));

    Ok(Value::Object(date_obj))
  }

  fn native_cat_time_full_year(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.fullYear requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.fullYear",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.fullYear requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    Ok(Value::Number(datetime.year() as f64))
  }

  fn native_cat_time_month(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.month requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.month",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.month requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    // Note: JavaScript months are 0-indexed (0-11), matching this behavior
    Ok(Value::Number((datetime.month() - 1) as f64))
  }

  fn native_cat_time_day(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.day requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.day",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.day requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    Ok(Value::Number(datetime.day() as f64))
  }

  fn native_cat_time_weekday(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.weekday requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.weekday",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.weekday requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    // Convert to 0-indexed weekday (Sunday = 0, Monday = 1, etc.)
    let weekday = datetime.weekday().num_days_from_sunday();

    Ok(Value::Number(weekday as f64))
  }

  fn native_cat_time_hours(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.hours requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.hours",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.hours requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    Ok(Value::Number(datetime.hour() as f64))
  }

  fn native_cat_time_minutes(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.minutes requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.minutes",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.minutes requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    Ok(Value::Number(datetime.minute() as f64))
  }

  fn native_cat_time_seconds(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.seconds requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.seconds",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.seconds requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    Ok(Value::Number(datetime.second() as f64))
  }

  fn native_cat_time_milliseconds(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.milliseconds requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.milliseconds",
          ))
        }
      },
      _ => {
        return Err(MewError::runtime(
          "CatTime.milliseconds requires a date object",
        ))
      }
    };

    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;
    let millis = nanos / 1_000_000;

    Ok(Value::Number(millis as f64))
  }

  fn native_cat_time_to_meow(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "CatTime.toMeow requires exactly one argument (date object)",
      ));
    }

    // Extract timestamp from date object
    let timestamp = match &args[0] {
      Value::Object(obj) => match obj.get("_timestamp") {
        Some(Value::Number(ts)) => *ts,
        _ => {
          return Err(MewError::runtime(
            "Invalid date object passed to CatTime.toMeow",
          ))
        }
      },
      _ => return Err(MewError::runtime("CatTime.toMeow requires a date object")),
    };

    // Convert timestamp to UTC date
    let seconds = (timestamp / 1000.0) as i64;
    let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;

    // Create a UTC datetime from the timestamp
    let datetime = DateTime::<Utc>::from_timestamp(seconds, nanos)
      .ok_or_else(|| MewError::runtime("Invalid timestamp in date object"))?;

    // Format date string similar to JavaScript's toString()
    let formatted = datetime.format("%a %b %d %Y %H:%M:%S GMT%z").to_string();

    Ok(Value::String(formatted))
  }

  // MewJ native functions
  fn native_mewj_sniff(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "MewJ.sniff requires exactly one argument",
      ));
    }

    match &args[0] {
      Value::String(json_str) => match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(json_value) => Ok(Interpreter::json_to_mew_value(json_value)),
        Err(e) => Err(MewError::runtime(format!("Invalid JSON syntax: {}", e))),
      },
      _ => Err(MewError::type_error(format!(
        "MewJ.sniff requires a string argument, got {}",
        args[0].type_name()
      ))),
    }
  }

  fn native_mewj_mewify(args: Vec<Value>) -> MewResult<Value> {
    if args.len() < 1 || args.len() > 2 {
      return Err(MewError::runtime(
        "MewJ.mewify requires one or two arguments",
      ));
    }

    let value = &args[0];
    let indent = if args.len() == 2 {
      match &args[1] {
        Value::Number(n) => {
          if *n >= 0.0 && *n <= 10.0 {
            Some(*n as usize)
          } else {
            None
          }
        }
        _ => None,
      }
    } else {
      None
    };

    match Interpreter::mew_value_to_json(value) {
      Ok(json_value) => {
        let result = if let Some(_spaces) = indent {
          match serde_json::to_string_pretty(&json_value) {
            Ok(s) => s,
            Err(e) => return Err(MewError::runtime(format!("Serialization error: {}", e))),
          }
        } else {
          match serde_json::to_string(&json_value) {
            Ok(s) => s,
            Err(e) => return Err(MewError::runtime(format!("Serialization error: {}", e))),
          }
        };

        Ok(Value::String(result))
      }
      Err(e) => Err(e),
    }
  }

  // Helper function to convert serde_json::Value to Mew Value
  fn json_to_mew_value(json_value: serde_json::Value) -> Value {
    match json_value {
      serde_json::Value::Null => Value::Null,
      serde_json::Value::Bool(b) => Value::Bool(b),
      serde_json::Value::Number(n) => {
        if let Some(f) = n.as_f64() {
          Value::Number(f)
        } else {
          // Handle integers that don't fit in f64
          Value::Number(n.as_i64().unwrap_or(0) as f64)
        }
      }
      serde_json::Value::String(s) => Value::String(s),
      serde_json::Value::Array(arr) => {
        let mew_arr = arr.into_iter().map(Self::json_to_mew_value).collect();
        Value::Array(mew_arr)
      }
      serde_json::Value::Object(obj) => {
        let mut mew_obj = HashMap::new();
        for (k, v) in obj {
          mew_obj.insert(k, Self::json_to_mew_value(v));
        }
        Value::Object(mew_obj)
      }
    }
  }

  // Helper function to convert Mew Value to serde_json::Value
  fn mew_value_to_json(value: &Value) -> MewResult<serde_json::Value> {
    match value {
      Value::Null => Ok(serde_json::Value::Null),
      Value::Bool(b) => Ok(serde_json::Value::Bool(*b)),
      Value::Number(n) => {
        if n.is_finite() {
          Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(*n).unwrap(),
          ))
        } else if n.is_nan() {
          Err(MewError::runtime("Cannot convert NaN to JSON"))
        } else {
          Err(MewError::runtime("Cannot convert Infinity to JSON"))
        }
      }
      Value::String(s) => Ok(serde_json::Value::String(s.clone())),
      Value::Array(arr) => {
        let mut json_arr = Vec::new();
        for item in arr {
          json_arr.push(Self::mew_value_to_json(item)?);
        }
        Ok(serde_json::Value::Array(json_arr))
      }
      Value::Object(obj) => {
        let mut json_obj = serde_json::Map::new();
        for (k, v) in obj {
          json_obj.insert(k.clone(), Self::mew_value_to_json(v)?);
        }
        Ok(serde_json::Value::Object(json_obj))
      }
      Value::Undefined => Ok(serde_json::Value::Null), // Convert undefined to null in JSON
      Value::Function(_) | Value::NativeFunction(_) => {
        Err(MewError::runtime("Functions cannot be converted to JSON"))
      }
    }
  }

  // Helper method to evaluate simple expressions that might be array indices
  #[allow(dead_code)]
  fn try_evaluate_as_expression(&mut self, expr_str: &str) -> Option<usize> {
    // Try to handle simple expressions like i+1, i-1, etc.

    // First, check for binary operations
    if let Some(plus_pos) = expr_str.find('+') {
      let left = &expr_str[0..plus_pos].trim();
      let right = &expr_str[plus_pos + 1..].trim();

      // Evaluate the left-hand side
      let left_val = if let Ok(val) = self.evaluate(&Expr::Variable(left.to_string())) {
        if let Value::Number(num) = val {
          Some(num)
        } else {
          None
        }
      } else {
        // Try parsing it as a number
        left.parse::<f64>().ok()
      };

      // Evaluate the right-hand side
      let right_val = if let Ok(val) = self.evaluate(&Expr::Variable(right.to_string())) {
        if let Value::Number(num) = val {
          Some(num)
        } else {
          None
        }
      } else {
        // Try parsing it as a number
        right.parse::<f64>().ok()
      };

      // If both sides are valid numbers, return the result
      if let (Some(left_num), Some(right_num)) = (left_val, right_val) {
        return Some((left_num + right_num) as usize);
      }
    } else if let Some(minus_pos) = expr_str.find('-') {
      let left = &expr_str[0..minus_pos].trim();
      let right = &expr_str[minus_pos + 1..].trim();

      // Evaluate the left-hand side
      let left_val = if let Ok(val) = self.evaluate(&Expr::Variable(left.to_string())) {
        if let Value::Number(num) = val {
          Some(num)
        } else {
          None
        }
      } else {
        // Try parsing it as a number
        left.parse::<f64>().ok()
      };

      // Evaluate the right-hand side
      let right_val = if let Ok(val) = self.evaluate(&Expr::Variable(right.to_string())) {
        if let Value::Number(num) = val {
          Some(num)
        } else {
          None
        }
      } else {
        // Try parsing it as a number
        right.parse::<f64>().ok()
      };

      // If both sides are valid numbers and left >= right, return the result
      if let (Some(left_num), Some(right_num)) = (left_val, right_val) {
        if left_num >= right_num {
          return Some((left_num - right_num) as usize);
        }
      }
    }

    // If we can't evaluate it as an expression, try to evaluate it as a variable
    if let Ok(val) = self.evaluate(&Expr::Variable(expr_str.to_string())) {
      if let Value::Number(num) = val {
        return Some(num as usize);
      }
    }

    None
  }

  // Native toString implementation for all value types
  fn native_to_string(args: Vec<Value>) -> MewResult<Value> {
    if args.is_empty() {
      return Err(MewError::runtime("toString requires at least one argument"));
    }

    let arg = &args[0];
    let string_repr = match arg {
      Value::Null => "null".to_string(),
      Value::Undefined => "undefined".to_string(),
      Value::Number(n) => {
        if n.is_nan() {
          "NaN".to_string()
        } else if n.is_infinite() {
          if n.is_sign_positive() {
            "Infinity".to_string()
          } else {
            "-Infinity".to_string()
          }
        } else if n.fract() == 0.0 {
          format!("{}", *n as i64)
        } else {
          n.to_string()
        }
      }
      Value::Bool(b) => b.to_string(),
      Value::String(s) => s.clone(),
      Value::Array(arr) => {
        let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
        format!("[{}]", elements.join(","))
      }
      Value::Object(_) => "[object Object]".to_string(),
      Value::Function(func) => {
        if let Some(name) = &func.name {
          format!("function {}(...)", name)
        } else {
          "function(...)".to_string()
        }
      }
      Value::NativeFunction(func) => {
        format!("function {}(...) [native]", func.name)
      }
    };

    Ok(Value::String(string_repr))
  }
}

pub fn interpret(source: &str) -> MewResult<Value> {
  use crate::lexer::MewLexer;
  use crate::parser::Parser;

  let mut lexer = MewLexer::new(source);
  let tokens = match lexer.scan_tokens() {
    Ok(t) => t,
    Err(e) => return Err(e),
  };

  let mut parser = Parser::new(tokens);
  let statements = match parser.parse() {
    Ok(s) => s,
    Err(e) => return Err(e),
  };

  let mut interpreter = Interpreter::new();
  interpreter.interpret(&statements)
}
