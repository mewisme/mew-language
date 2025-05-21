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

  // Static native functions for Object methods
  fn native_object_keys(args: Vec<Value>) -> MewResult<Value> {
    if args.len() != 1 {
      return Err(MewError::runtime(
        "Object.keys requires exactly one argument",
      ));
    }

    match &args[0] {
      Value::Object(obj) => {
        // Extract keys from object
        let keys: Vec<Value> = obj.keys().map(|k| Value::String(k.clone())).collect();

        Ok(Value::Array(keys))
      }
      Value::Array(arr) => {
        // For arrays, return the indices as numbers (not strings)
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
        // Extract values from object
        let values: Vec<Value> = obj.values().cloned().collect();

        Ok(Value::Array(values))
      }
      Value::Array(arr) => {
        // For arrays, return a clone of the array
        Ok(Value::Array(arr.clone()))
      }
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
            Err(MewError::Runtime(msg)) if msg.contains("break") => break,
            Err(MewError::Runtime(msg)) if msg.contains("continue") => continue,
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
        Err(MewError::Runtime(msg)) if msg.starts_with("return:") => {
          let value_str = msg.trim_start_matches("return:");

          // Parse the return value (simplified - in a real implementation, this would be more robust)
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
            // Assume it's a string (remove quotes)
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
          // Arithmetic
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

          // String concatenation
          (BinaryOp::Add, Value::String(a), _) => Ok(Value::String(format!("{}{}", a, right))),
          (BinaryOp::Add, _, Value::String(b)) => Ok(Value::String(format!("{}{}", left, b))),

          // Comparisons
          (BinaryOp::Eq, _, _) => Ok(Value::Bool(self.is_equal(&left, &right))),
          (BinaryOp::NotEq, _, _) => Ok(Value::Bool(!self.is_equal(&left, &right))),
          (BinaryOp::Lt, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
          (BinaryOp::Lte, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
          (BinaryOp::Gt, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
          (BinaryOp::Gte, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),

          // Logical operators
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
      Expr::Unary(op, expr) => {
        let right = self.evaluate(&*expr)?;

        match (op, &right) {
          (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
          (UnaryOp::Not, _) => Ok(Value::Bool(!right.is_truthy())),
          _ => Err(MewError::type_error(format!(
            "Cannot apply operator {:?} to {}",
            op,
            right.type_name()
          ))),
        }
      }
      Expr::Call(callee, arguments) => {
        let callee = self.evaluate(&*callee)?;

        let mut args = Vec::new();
        for arg in arguments {
          args.push(self.evaluate(arg)?);
        }

        self.call_function(callee, args)
      }
      Expr::Get(object, name) => {
        let object = self.evaluate(&*object)?;

        match object {
          Value::Object(obj) => {
            if name.starts_with('[') && name.ends_with(']') {
              // Handle dynamic property access: obj[expr]
              let key_str = &name[1..name.len() - 1];

              if let Ok(expr_value) = self.evaluate(&Expr::Variable(key_str.to_string())) {
                if let Value::String(key) = expr_value {
                  if let Some(value) = obj.get(&key) {
                    Ok(value.clone())
                  } else {
                    Ok(Value::Undefined)
                  }
                } else if let Value::Number(num) = expr_value {
                  // Convert number to string for object key lookup
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
                // Try evaluating the index as a direct expression
                let parsed_result = key_str.parse::<usize>();

                if let Ok(index) = parsed_result {
                  let key = index.to_string();
                  if let Some(value) = obj.get(&key) {
                    return Ok(value.clone());
                  } else {
                    return Ok(Value::Undefined);
                  }
                }

                // If it's not a variable, try evaluating it as a direct expression
                let expr_value = self.evaluate(&Expr::Variable(key_str.to_string()))?;

                if let Value::String(key) = expr_value {
                  if let Some(value) = obj.get(&key) {
                    Ok(value.clone())
                  } else {
                    Ok(Value::Undefined)
                  }
                } else if let Value::Number(num) = expr_value {
                  // Convert number to string for object key lookup
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
              }
            } else if let Some(value) = obj.get(name) {
              Ok(value.clone())
            } else {
              Ok(Value::Undefined)
            }
          }
          Value::Array(arr) => {
            if name == "length" {
              // Special property: length
              Ok(Value::Number(arr.len() as f64))
            } else if name.starts_with('[') && name.ends_with(']') {
              // Handle dynamic indexing: arr[expr]
              let index_str = &name[1..name.len() - 1];

              if let Ok(expr_value) = self.evaluate(&Expr::Variable(index_str.to_string())) {
                if let Value::Number(n) = expr_value {
                  let index = n as usize;
                  if index < arr.len() {
                    Ok(arr[index].clone())
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
                // Try parsing the index directly
                if let Ok(index) = index_str.parse::<usize>() {
                  if index < arr.len() {
                    Ok(arr[index].clone())
                  } else {
                    Err(MewError::runtime(format!("Index out of bounds: {}", index)))
                  }
                } else {
                  Err(MewError::type_error(format!(
                    "Invalid array index: {}",
                    index_str
                  )))
                }
              }
            } else {
              // Try to parse the name as a number for array indices
              if let Ok(index) = name.parse::<usize>() {
                if index < arr.len() {
                  Ok(arr[index].clone())
                } else {
                  Err(MewError::runtime(format!("Index out of bounds: {}", index)))
                }
              } else {
                // Special case for our internal variable names
                if name.starts_with("__index_")
                  || name.starts_with("__key_")
                  || name.starts_with("__keys_")
                  || name.starts_with("__values_")
                  || name.starts_with("__iterator_")
                {
                  Err(MewError::runtime(format!(
                    "Internal variable '{}' not available on this array",
                    name
                  )))
                } else {
                  Err(MewError::type_error(format!(
                    "Cannot access property '{}' of array",
                    name
                  )))
                }
              }
            }
          }
          _ => Err(MewError::type_error(format!(
            "Cannot access property '{}' of {}",
            name,
            object.type_name()
          ))),
        }
      }
      Expr::Set(object, name, value) => {
        let object = self.evaluate(&*object)?;
        let value = self.evaluate(&*value)?;

        match object {
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
            object.type_name()
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
                        // For prefix (++arr[i]), return the new value
                        Ok(new_value)
                      } else {
                        // For postfix (arr[i]++), return the original value
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
                  // Handle the dynamic array indexing case (similar to Get handling)
                  let index_str = &property_name[1..property_name.len() - 1];
                  if let Ok(expr_value) = self.evaluate(&Expr::Variable(index_str.to_string())) {
                    if let Value::Number(n) = expr_value {
                      let index = n as usize;
                      if index < arr.len() {
                        if let Value::Number(element_val) = arr[index] {
                          let new_value = Value::Number(element_val + 1.0);
                          arr[index] = new_value.clone();

                          if *is_prefix {
                            // For prefix (++arr[i]), return the new value
                            Ok(new_value)
                          } else {
                            // For postfix (arr[i]++), return the original value
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
        // Handle both prefix (--x) and postfix (x--) decrement
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
                // For prefix (--x), return the new value
                Ok(new_value)
              } else {
                // For postfix (x--), return the original value
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
                      // For prefix (--obj.prop), return the new value
                      Ok(new_value)
                    } else {
                      // For postfix (obj.prop--), return the original value
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
                        // For prefix (--arr[i]), return the new value
                        Ok(new_value)
                      } else {
                        // For postfix (arr[i]--), return the original value
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
                            // For prefix (--arr[i]), return the new value
                            Ok(new_value)
                          } else {
                            // For postfix (arr[i]--), return the original value
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
          Err(MewError::Runtime(msg)) if msg.starts_with("return:") => {
            let value_str = msg.trim_start_matches("return:");

            // Parse the return value (simplified)
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
              // Assume it's a string (remove quotes)
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
      // For other types, reference equality
      _ => std::ptr::eq(a, b),
    }
  }
}

pub fn interpret(source: &str) -> MewResult<Value> {
  use crate::lexer::MewLexer;
  use crate::parser::Parser;

  let mut lexer = MewLexer::new(source);
  let tokens = lexer.scan_tokens()?;

  let mut parser = Parser::new(tokens);
  let statements = parser.parse()?;

  let mut interpreter = Interpreter::new();
  interpreter.interpret(&statements)
}
