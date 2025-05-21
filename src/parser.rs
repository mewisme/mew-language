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
use crate::lexer::{Token, TokenKind};
use crate::value::{BinaryOp, Expr, Stmt, UnaryOp, Value};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// Add a Display implementation for Expr
impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Literal(val) => write!(f, "{}", val),
      Expr::Variable(name) => write!(f, "{}", name),
      Expr::Assignment(name, _) => write!(f, "{} = ...", name),
      Expr::Binary(_, _, _) => write!(f, "<binary-expr>"),
      Expr::Unary(_, _) => write!(f, "<unary-expr>"),
      Expr::Call(_, _) => write!(f, "<call-expr>"),
      Expr::Get(_, name) => write!(f, "<get-expr>.{}", name),
      Expr::Set(_, name, _) => write!(f, "<set-expr>.{} = ...", name),
      Expr::ArrayLiteral(_) => write!(f, "[...]"),
      Expr::ObjectLiteral(_) => write!(f, "{{...}}"),
      Expr::Function(name, _, _) => {
        if let Some(n) = name {
          write!(f, "function {}(...)", n)
        } else {
          write!(f, "function(...)")
        }
      }
      Expr::Increment(expr, is_prefix) => {
        if *is_prefix {
          write!(f, "++{}", expr)
        } else {
          write!(f, "{}++", expr)
        }
      }
      Expr::Decrement(expr, is_prefix) => {
        if *is_prefix {
          write!(f, "--{}", expr)
        } else {
          write!(f, "{}--", expr)
        }
      }
    }
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, current: 0 }
  }

  pub fn parse(&mut self) -> MewResult<Vec<Rc<RefCell<Stmt>>>> {
    let mut statements = Vec::new();

    while !self.is_at_end() {
      match self.declaration() {
        Ok(stmt) => statements.push(Rc::new(RefCell::new(stmt))),
        Err(e) => {
          // Attempt to synchronize to recover from errors
          self.synchronize();
          return Err(e);
        }
      }
    }

    Ok(statements)
  }

  fn declaration(&mut self) -> MewResult<Stmt> {
    if self.match_tokens(&[TokenKind::Var, TokenKind::Let, TokenKind::Const]) {
      return self.var_declaration();
    }

    if self.match_tokens(&[TokenKind::Function]) {
      return self.function_declaration("function");
    }

    self.statement()
  }

  fn var_declaration(&mut self) -> MewResult<Stmt> {
    let token = self.previous();
    let is_const = match token.kind {
      TokenKind::Const => true,
      _ => false,
    };

    let name = self.consume_identifier("Expected variable name.")?;

    let initializer = if self.match_tokens(&[TokenKind::Equal]) {
      Some(self.expression()?)
    } else {
      None
    };

    self.consume(
      TokenKind::Semicolon,
      "Expected ';' after variable declaration.",
    )?;
    Ok(Stmt::VarDeclaration(name, initializer, is_const))
  }

  fn function_declaration(&mut self, kind: &str) -> MewResult<Stmt> {
    let name = self.consume_identifier(&format!("Expected {} name.", kind))?;
    self.consume(
      TokenKind::LeftParen,
      &format!("Expected '(' after {} name.", kind),
    )?;

    let mut parameters = Vec::new();
    if !self.check(TokenKind::RightParen) {
      loop {
        if parameters.len() >= 255 {
          return Err(MewError::syntax_at(
            "Cannot have more than 255 parameters.",
            self.peek().location,
          ));
        }

        parameters.push(self.consume_identifier("Expected parameter name.")?);

        if !self.match_tokens(&[TokenKind::Comma]) {
          break;
        }
      }
    }

    self.consume(TokenKind::RightParen, "Expected ')' after parameters.")?;

    self.consume(
      TokenKind::LeftBrace,
      &format!("Expected '{{' before {} body.", kind),
    )?;
    let body = self.block()?;

    Ok(Stmt::Function(name, parameters, body))
  }

  fn statement(&mut self) -> MewResult<Stmt> {
    if self.match_tokens(&[TokenKind::Print]) {
      return self.print_statement();
    }

    if self.match_tokens(&[TokenKind::LeftBrace]) {
      let statements = self.block()?;
      return Ok(Stmt::Block(statements));
    }

    if self.match_tokens(&[TokenKind::If]) {
      return self.if_statement();
    }

    if self.match_tokens(&[TokenKind::While]) {
      return self.while_statement();
    }

    if self.match_tokens(&[TokenKind::Do]) {
      return self.do_while_statement();
    }

    if self.match_tokens(&[TokenKind::For]) {
      return self.for_statement();
    }

    if self.match_tokens(&[TokenKind::Break]) {
      return self.break_statement();
    }

    if self.match_tokens(&[TokenKind::Continue]) {
      return self.continue_statement();
    }

    if self.match_tokens(&[TokenKind::Return]) {
      return self.return_statement();
    }

    if self.match_tokens(&[TokenKind::Switch]) {
      return self.switch_statement();
    }

    self.expression_statement()
  }

  fn print_statement(&mut self) -> MewResult<Stmt> {
    self.consume(TokenKind::LeftParen, "Expected '(' after 'purr'.")?;
    let value = self.expression()?;
    self.consume(TokenKind::RightParen, "Expected ')' after expression.")?;
    self.consume(TokenKind::Semicolon, "Expected ';' after value.")?;
    Ok(Stmt::Print(value))
  }

  fn block(&mut self) -> MewResult<Vec<Rc<RefCell<Stmt>>>> {
    let mut statements = Vec::new();

    while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
      match self.declaration() {
        Ok(stmt) => statements.push(Rc::new(RefCell::new(stmt))),
        Err(e) => return Err(e),
      }
    }

    self.consume(TokenKind::RightBrace, "Expected '}' after block.")?;
    Ok(statements)
  }

  fn if_statement(&mut self) -> MewResult<Stmt> {
    self.consume(TokenKind::LeftParen, "Expected '(' after 'meow?'.")?;
    let condition = self.expression()?;
    self.consume(TokenKind::RightParen, "Expected ')' after condition.")?;

    let then_branch = Rc::new(RefCell::new(self.statement()?));

    let mut else_branch = None;
    if self.match_tokens(&[TokenKind::ElseIf]) {
      else_branch = Some(Rc::new(RefCell::new(self.if_statement()?)));
    } else if self.match_tokens(&[TokenKind::Else]) {
      else_branch = Some(Rc::new(RefCell::new(self.statement()?)));
    }

    Ok(Stmt::If(condition, then_branch, else_branch))
  }

  fn while_statement(&mut self) -> MewResult<Stmt> {
    self.consume(TokenKind::LeftParen, "Expected '(' after 'mewhile'.")?;
    let condition = self.expression()?;
    self.consume(TokenKind::RightParen, "Expected ')' after condition.")?;

    let body = Rc::new(RefCell::new(self.statement()?));

    Ok(Stmt::While(condition, body))
  }

  fn do_while_statement(&mut self) -> MewResult<Stmt> {
    let body = Rc::new(RefCell::new(self.statement()?));

    self.consume(
      TokenKind::While,
      "Expected 'mewhile' after block in do-while statement.",
    )?;
    self.consume(TokenKind::LeftParen, "Expected '(' after 'mewhile'.")?;
    let condition = self.expression()?;
    self.consume(TokenKind::RightParen, "Expected ')' after condition.")?;
    self.consume(
      TokenKind::Semicolon,
      "Expected ';' after do-while statement.",
    )?;

    Ok(Stmt::Block(vec![
      body.clone(), // Clone before using it again
      Rc::new(RefCell::new(Stmt::While(condition, body))),
    ]))
  }

  fn for_statement(&mut self) -> MewResult<Stmt> {
    self.consume(TokenKind::LeftParen, "Expected '(' after 'fur'.")?;

    let initializer;
    if self.match_tokens(&[TokenKind::Var, TokenKind::Let, TokenKind::Const]) {
      let token = self.previous();
      let is_const = token.kind == TokenKind::Const;

      let var_name = self.consume_identifier("Expected variable name.")?;

      let var_initializer = if self.match_tokens(&[TokenKind::Equal]) {
        Some(self.expression()?)
      } else {
        None
      };

      if self.match_tokens(&[TokenKind::In]) || self.match_tokens(&[TokenKind::Of]) {
        let is_of = self.previous().kind == TokenKind::Of;
        return self.for_in_of_statement(
          Stmt::VarDeclaration(var_name, var_initializer, is_const),
          is_of,
        );
      }

      self.consume(
        TokenKind::Semicolon,
        "Expected ';' after variable declaration.",
      )?;

      initializer = Some(Stmt::VarDeclaration(var_name, var_initializer, is_const));
    } else if self.match_tokens(&[TokenKind::Semicolon]) {
      initializer = None;
    } else {
      initializer = Some(self.expression_statement()?);
    }

    let condition = if !self.check(TokenKind::Semicolon) {
      self.expression()?
    } else {
      Expr::Literal(Value::Bool(true))
    };
    self.consume(TokenKind::Semicolon, "Expected ';' after loop condition.")?;

    let increment = if !self.check(TokenKind::RightParen) {
      Some(self.expression()?)
    } else {
      None
    };

    self.consume(TokenKind::RightParen, "Expected ')' after for clauses.")?;

    let mut body = self.statement()?;

    if let Some(inc) = increment {
      body = Stmt::Block(vec![
        Rc::new(RefCell::new(body)),
        Rc::new(RefCell::new(Stmt::Expression(inc))),
      ]);
    }

    body = Stmt::While(condition, Rc::new(RefCell::new(body)));

    if let Some(init) = initializer {
      body = Stmt::Block(vec![
        Rc::new(RefCell::new(init)),
        Rc::new(RefCell::new(body)),
      ]);
    }

    Ok(body)
  }

  fn for_in_of_statement(&mut self, initializer: Stmt, is_of: bool) -> MewResult<Stmt> {
    let iterator = self.expression()?;
    self.consume(
      TokenKind::RightParen,
      "Expected ')' after for-in/of clauses.",
    )?;

    let body = self.statement()?;

    let (var_name, is_const) = match &initializer {
      Stmt::VarDeclaration(name, _, const_val) => (name.clone(), *const_val),
      _ => return Err(MewError::syntax("Invalid for-in/of loop initializer.")),
    };

    let iterator_var = format!("__iterator_{}", var_name);
    let index_var = format!("__index_{}", var_name);

    let iterator_decl = Stmt::VarDeclaration(iterator_var.clone(), Some(iterator.clone()), false);

    let index_decl = Stmt::VarDeclaration(
      index_var.clone(),
      Some(Expr::Literal(Value::Number(0.0))),
      false,
    );

    let keys_or_values = if is_of {
      Expr::Call(
        Box::new(Expr::Get(
          Box::new(Expr::Variable(String::from("Object"))),
          String::from("values"),
        )),
        vec![Expr::Variable(iterator_var.clone())],
      )
    } else {
      Expr::Call(
        Box::new(Expr::Get(
          Box::new(Expr::Variable(String::from("Object"))),
          String::from("keys"),
        )),
        vec![Expr::Variable(iterator_var.clone())],
      )
    };

    let collection_var = if is_of {
      format!("__values_{}", var_name)
    } else {
      format!("__keys_{}", var_name)
    };
    let collection_decl = Stmt::VarDeclaration(collection_var.clone(), Some(keys_or_values), false);

    let condition = Expr::Binary(
      Box::new(Expr::Variable(index_var.clone())),
      BinaryOp::Lt,
      Box::new(Expr::Get(
        Box::new(Expr::Variable(collection_var.clone())),
        String::from("length"),
      )),
    );

    let loop_body = if is_const {
      let const_decl = Stmt::VarDeclaration(
        var_name.clone(),
        Some(Expr::Get(
          Box::new(Expr::Variable(collection_var.clone())),
          String::from("[") + &index_var + "]",
        )),
        true, // is_const = true
      );

      Stmt::Block(vec![
        Rc::new(RefCell::new(const_decl)),
        Rc::new(RefCell::new(body)),
        Rc::new(RefCell::new(Stmt::Expression(Expr::Increment(
          Box::new(Expr::Variable(index_var.clone())),
          false, // postfix increment
        )))),
      ])
    } else {
      let var_assignment = Stmt::Expression(Expr::Assignment(
        var_name.clone(),
        Box::new(Expr::Get(
          Box::new(Expr::Variable(collection_var.clone())),
          String::from("[") + &index_var + "]",
        )),
      ));

      Stmt::Block(vec![
        Rc::new(RefCell::new(var_assignment)),
        Rc::new(RefCell::new(body)),
        Rc::new(RefCell::new(Stmt::Expression(Expr::Increment(
          Box::new(Expr::Variable(index_var.clone())),
          false, // postfix increment
        )))),
      ])
    };

    let while_loop = Stmt::While(condition, Rc::new(RefCell::new(loop_body)));

    let mut statements = vec![
      Rc::new(RefCell::new(iterator_decl)),
      Rc::new(RefCell::new(index_decl)),
      Rc::new(RefCell::new(collection_decl)),
    ];

    if !is_const {
      statements.push(Rc::new(RefCell::new(initializer)));
    }

    statements.push(Rc::new(RefCell::new(while_loop)));

    let full_block = Stmt::Block(statements);

    Ok(full_block)
  }

  fn break_statement(&mut self) -> MewResult<Stmt> {
    self.consume(TokenKind::Semicolon, "Expected ';' after break statement.")?;
    Ok(Stmt::Break)
  }

  fn continue_statement(&mut self) -> MewResult<Stmt> {
    self.consume(
      TokenKind::Semicolon,
      "Expected ';' after continue statement.",
    )?;
    Ok(Stmt::Continue)
  }

  fn return_statement(&mut self) -> MewResult<Stmt> {
    let _keyword = self.previous();

    let value = if !self.check(TokenKind::Semicolon) {
      Some(self.expression()?)
    } else {
      None
    };

    self.consume(TokenKind::Semicolon, "Expected ';' after return value.")?;
    Ok(Stmt::Return(value))
  }

  fn switch_statement(&mut self) -> MewResult<Stmt> {
    self.consume(TokenKind::LeftParen, "Expected '(' after 'catwalk'.")?;
    let value = self.expression()?;
    self.consume(TokenKind::RightParen, "Expected ')' after value.")?;

    self.consume(TokenKind::LeftBrace, "Expected '{' after switch value.")?;

    let mut cases = Vec::new();

    while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
      if self.match_tokens(&[TokenKind::Case]) {
        let case_value = self.expression()?;
        self.consume(TokenKind::Colon, "Expected ':' after case value.")?;

        let mut statements = Vec::new();
        while !self.check(TokenKind::Case)
          && !self.check(TokenKind::Default)
          && !self.check(TokenKind::RightBrace)
          && !self.is_at_end()
        {
          statements.push(Rc::new(RefCell::new(self.declaration()?)));
        }

        cases.push((Some(case_value), statements));
      } else if self.match_tokens(&[TokenKind::Default]) {
        self.consume(TokenKind::Colon, "Expected ':' after default.")?;

        let mut statements = Vec::new();
        while !self.check(TokenKind::Case)
          && !self.check(TokenKind::Default)
          && !self.check(TokenKind::RightBrace)
          && !self.is_at_end()
        {
          statements.push(Rc::new(RefCell::new(self.declaration()?)));
        }

        cases.push((None, statements));
      } else {
        return Err(MewError::syntax(
          "Expected 'claw' or 'default' in switch statement.",
        ));
      }
    }

    self.consume(TokenKind::RightBrace, "Expected '}' after switch cases.")?;

    Ok(Stmt::Switch(value, cases))
  }

  fn expression_statement(&mut self) -> MewResult<Stmt> {
    let expr = self.expression()?;
    self.consume(TokenKind::Semicolon, "Expected ';' after expression.")?;
    Ok(Stmt::Expression(expr))
  }

  fn expression(&mut self) -> MewResult<Expr> {
    self.assignment()
  }

  fn assignment(&mut self) -> MewResult<Expr> {
    let expr = self.or()?;

    if self.match_tokens(&[TokenKind::Equal]) {
      let _equals = self.previous();
      let value = self.assignment()?;

      if let Expr::Variable(name) = expr {
        return Ok(Expr::Assignment(name, Box::new(value)));
      } else if let Expr::Get(object, name) = expr {
        return Ok(Expr::Set(object, name, Box::new(value)));
      }

      return Err(MewError::syntax("Invalid assignment target."));
    }

    Ok(expr)
  }

  fn or(&mut self) -> MewResult<Expr> {
    let mut expr = self.and()?;

    while self.match_tokens(&[TokenKind::Or]) {
      let operator = BinaryOp::Or;
      let right = self.and()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
  }

  fn and(&mut self) -> MewResult<Expr> {
    let mut expr = self.equality()?;

    while self.match_tokens(&[TokenKind::And]) {
      let operator = BinaryOp::And;
      let right = self.equality()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
  }

  fn equality(&mut self) -> MewResult<Expr> {
    let mut expr = self.comparison()?;

    while self.match_tokens(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
      let operator = match self.previous().kind {
        TokenKind::BangEqual => BinaryOp::NotEq,
        TokenKind::EqualEqual => BinaryOp::Eq,
        _ => unreachable!(),
      };
      let right = self.comparison()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
  }

  fn comparison(&mut self) -> MewResult<Expr> {
    let mut expr = self.term()?;

    while self.match_tokens(&[
      TokenKind::Greater,
      TokenKind::GreaterEqual,
      TokenKind::Less,
      TokenKind::LessEqual,
    ]) {
      let operator = match self.previous().kind {
        TokenKind::Greater => BinaryOp::Gt,
        TokenKind::GreaterEqual => BinaryOp::Gte,
        TokenKind::Less => BinaryOp::Lt,
        TokenKind::LessEqual => BinaryOp::Lte,
        _ => unreachable!(),
      };
      let right = self.term()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
  }

  fn term(&mut self) -> MewResult<Expr> {
    let mut expr = self.factor()?;

    while self.match_tokens(&[TokenKind::Minus, TokenKind::Plus]) {
      let operator = match self.previous().kind {
        TokenKind::Minus => BinaryOp::Sub,
        TokenKind::Plus => BinaryOp::Add,
        _ => unreachable!(),
      };
      let right = self.factor()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
  }

  fn factor(&mut self) -> MewResult<Expr> {
    let mut expr = self.unary()?;

    while self.match_tokens(&[TokenKind::Slash, TokenKind::Star, TokenKind::Percent]) {
      let operator = match self.previous().kind {
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Percent => BinaryOp::Mod,
        _ => unreachable!(),
      };
      let right = self.unary()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
  }

  fn unary(&mut self) -> MewResult<Expr> {
    if self.match_tokens(&[TokenKind::Bang, TokenKind::Minus]) {
      let operator = match self.previous().kind {
        TokenKind::Bang => UnaryOp::Not,
        TokenKind::Minus => UnaryOp::Minus,
        _ => unreachable!(),
      };
      let right = self.unary()?;
      return Ok(Expr::Unary(operator, Box::new(right)));
    }

    if self.match_tokens(&[TokenKind::Increment, TokenKind::Decrement]) {
      let is_increment = match self.previous().kind {
        TokenKind::Increment => true,
        TokenKind::Decrement => false,
        _ => unreachable!(),
      };

      let right = self.unary()?;

      match &right {
        Expr::Variable(_) | Expr::Get(_, _) => {
          if is_increment {
            return Ok(Expr::Increment(Box::new(right), true)); // prefix
          } else {
            return Ok(Expr::Decrement(Box::new(right), true)); // prefix
          }
        }
        _ => return Err(MewError::syntax("Invalid increment/decrement target.")),
      }
    }

    self.call()
  }

  fn call(&mut self) -> MewResult<Expr> {
    let mut expr = self.primary()?;

    loop {
      if self.match_tokens(&[TokenKind::LeftParen]) {
        expr = self.finish_call(expr)?;
      } else if self.match_tokens(&[TokenKind::Dot]) {
        let name = self.consume_identifier("Expected property name after '.'.")?;
        expr = Expr::Get(Box::new(expr), name);
      } else if self.match_tokens(&[TokenKind::LeftBracket]) {
        let index = self.expression()?;
        self.consume(TokenKind::RightBracket, "Expected ']' after array index.")?;
        expr = Expr::Get(Box::new(expr), format!("[{}]", index));
      } else if self.match_tokens(&[TokenKind::Increment, TokenKind::Decrement]) {
        let is_increment = match self.previous().kind {
          TokenKind::Increment => true,
          TokenKind::Decrement => false,
          _ => unreachable!(),
        };

        match &expr {
          Expr::Variable(_) | Expr::Get(_, _) => {
            if is_increment {
              expr = Expr::Increment(Box::new(expr), false); // postfix
            } else {
              expr = Expr::Decrement(Box::new(expr), false); // postfix
            }
          }
          _ => return Err(MewError::syntax("Invalid increment/decrement target.")),
        }
      } else {
        break;
      }
    }

    Ok(expr)
  }

  fn finish_call(&mut self, callee: Expr) -> MewResult<Expr> {
    let mut arguments = Vec::new();

    if !self.check(TokenKind::RightParen) {
      loop {
        if arguments.len() >= 255 {
          return Err(MewError::syntax_at(
            "Cannot have more than 255 arguments.",
            self.peek().location,
          ));
        }

        arguments.push(self.expression()?);

        if !self.match_tokens(&[TokenKind::Comma]) {
          break;
        }
      }
    }

    self.consume(TokenKind::RightParen, "Expected ')' after arguments.")?;

    Ok(Expr::Call(Box::new(callee), arguments))
  }

  fn primary(&mut self) -> MewResult<Expr> {
    if self.match_tokens(&[TokenKind::Boolean(true)]) {
      return Ok(Expr::Literal(Value::Bool(true)));
    }

    if self.match_tokens(&[TokenKind::Boolean(false)]) {
      return Ok(Expr::Literal(Value::Bool(false)));
    }

    if self.match_tokens(&[TokenKind::Null]) {
      return Ok(Expr::Literal(Value::Null));
    }

    if self.match_tokens(&[TokenKind::Undefined]) {
      return Ok(Expr::Literal(Value::Undefined));
    }

    // Matching number
    if self.check_type_variant::<f64>(&TokenKind::Number(0.0)) {
      if self.match_tokens(&[TokenKind::Number(0.0)]) {
        if let TokenKind::Number(n) = self.previous().kind {
          return Ok(Expr::Literal(Value::Number(n)));
        }
      }
    }

    // Matching string
    if self.check_type_variant::<String>(&TokenKind::String(String::new())) {
      if self.match_tokens(&[TokenKind::String(String::new())]) {
        if let TokenKind::String(s) = &self.previous().kind {
          return Ok(Expr::Literal(Value::String(s.clone())));
        }
      }
    }

    if self.match_tokens(&[TokenKind::Infinity]) {
      return Ok(Expr::Literal(Value::Number(f64::INFINITY)));
    }

    if self.match_tokens(&[TokenKind::NaN]) {
      return Ok(Expr::Literal(Value::Number(f64::NAN)));
    }

    if self.match_tokens(&[TokenKind::LeftParen]) {
      let expr = self.expression()?;
      self.consume(TokenKind::RightParen, "Expected ')' after expression.")?;
      return Ok(expr);
    }

    if self.match_tokens(&[TokenKind::LeftBracket]) {
      return self.array_literal();
    }

    if self.match_tokens(&[TokenKind::LeftBrace]) {
      return self.object_literal();
    }

    if self.match_tokens(&[TokenKind::Function]) {
      return self.function_expression();
    }

    // Matching identifier
    if self.check_type_variant::<String>(&TokenKind::Identifier(String::new())) {
      if self.match_tokens(&[TokenKind::Identifier(String::new())]) {
        if let TokenKind::Identifier(name) = &self.previous().kind {
          return Ok(Expr::Variable(name.clone()));
        }
      }
    }

    Err(MewError::syntax(format!(
      "Expected expression, got {:?}.",
      self.peek().kind
    )))
  }

  fn array_literal(&mut self) -> MewResult<Expr> {
    let mut elements = Vec::new();

    if !self.check(TokenKind::RightBracket) {
      loop {
        elements.push(self.expression()?);

        if !self.match_tokens(&[TokenKind::Comma]) {
          break;
        }

        if self.check(TokenKind::RightBracket) {
          break;
        }
      }
    }

    self.consume(
      TokenKind::RightBracket,
      "Expected ']' after array elements.",
    )?;

    Ok(Expr::ArrayLiteral(elements))
  }

  fn object_literal(&mut self) -> MewResult<Expr> {
    let mut properties = Vec::new();

    if !self.check(TokenKind::RightBrace) {
      loop {
        let key = if self.check_type_variant::<String>(&TokenKind::Identifier(String::new())) {
          if self.check_next(TokenKind::Colon) {
            if let TokenKind::Identifier(name) = &self.advance().kind {
              name.clone()
            } else {
              unreachable!()
            }
          } else {
            return Err(MewError::syntax("Expected ':' after property name."));
          }
        } else if self.check_type_variant::<String>(&TokenKind::String(String::new())) {
          if self.match_tokens(&[TokenKind::String(String::new())]) {
            if let TokenKind::String(s) = &self.previous().kind {
              s.clone()
            } else {
              unreachable!()
            }
          } else {
            unreachable!()
          }
        } else {
          return Err(MewError::syntax("Expected property name or string."));
        };

        self.consume(TokenKind::Colon, "Expected ':' after property name.")?;

        let value = self.expression()?;

        properties.push((key, value));

        if !self.match_tokens(&[TokenKind::Comma]) {
          break;
        }

        if self.check(TokenKind::RightBrace) {
          break;
        }
      }
    }

    self.consume(
      TokenKind::RightBrace,
      "Expected '}' after object properties.",
    )?;

    Ok(Expr::ObjectLiteral(properties))
  }

  fn function_expression(&mut self) -> MewResult<Expr> {
    let name = if self.check_type_variant::<String>(&TokenKind::Identifier(String::new())) {
      if let TokenKind::Identifier(name) = &self.advance().kind {
        Some(name.clone())
      } else {
        None
      }
    } else {
      None
    };

    self.consume(TokenKind::LeftParen, "Expected '(' after function name.")?;

    let mut parameters = Vec::new();
    if !self.check(TokenKind::RightParen) {
      loop {
        if parameters.len() >= 255 {
          return Err(MewError::syntax_at(
            "Cannot have more than 255 parameters.",
            self.peek().location,
          ));
        }

        parameters.push(self.consume_identifier("Expected parameter name.")?);

        if !self.match_tokens(&[TokenKind::Comma]) {
          break;
        }
      }
    }

    self.consume(TokenKind::RightParen, "Expected ')' after parameters.")?;

    if self.match_tokens(&[TokenKind::Arrow]) {
      if self.match_tokens(&[TokenKind::LeftBrace]) {
        let body = self.block()?;
        return Ok(Expr::Function(name, parameters, body));
      } else {
        let expr = self.expression()?;
        let body = vec![Rc::new(RefCell::new(Stmt::Return(Some(expr))))];
        return Ok(Expr::Function(name, parameters, body));
      }
    }

    self.consume(TokenKind::LeftBrace, "Expected '{' before function body.")?;
    let body = self.block()?;

    Ok(Expr::Function(name, parameters, body))
  }

  fn match_tokens(&mut self, types: &[TokenKind]) -> bool {
    for t in types {
      if self.check(t.clone()) {
        self.advance();
        return true;
      }
    }

    false
  }

  fn consume(&mut self, kind: TokenKind, message: &str) -> MewResult<Token> {
    if self.check(kind) {
      Ok(self.advance())
    } else {
      let token = self.peek();
      Err(MewError::syntax_at(
        format!("{} Got {:?}", message, token.kind),
        token.location,
      ))
    }
  }

  fn consume_identifier(&mut self, message: &str) -> MewResult<String> {
    if self.check_type_variant::<String>(&TokenKind::Identifier(String::new())) {
      if self.match_tokens(&[TokenKind::Identifier(String::new())]) {
        if let TokenKind::Identifier(name) = &self.previous().kind {
          return Ok(name.clone());
        }
      }
    }

    let token = self.peek();
    Err(MewError::syntax_at(message, token.location))
  }

  fn check(&self, kind: TokenKind) -> bool {
    if self.is_at_end() {
      return false;
    }

    match (&self.peek().kind, &kind) {
      (TokenKind::Number(_), TokenKind::Number(_)) => true,
      (TokenKind::String(_), TokenKind::String(_)) => true,
      (TokenKind::Boolean(a), TokenKind::Boolean(b)) => a == b,
      (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
      (a, b) if a == b => true,
      _ => false,
    }
  }

  fn check_type_variant<T: 'static>(&self, _pattern: &TokenKind) -> bool {
    if self.is_at_end() {
      return false;
    }

    match &self.peek().kind {
      TokenKind::Number(_) if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() => true,
      TokenKind::String(_) if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() => {
        true
      }
      TokenKind::Boolean(_) if std::any::TypeId::of::<T>() == std::any::TypeId::of::<bool>() => {
        true
      }
      TokenKind::Identifier(_)
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() =>
      {
        true
      }
      _ => false,
    }
  }

  fn check_next(&self, kind: TokenKind) -> bool {
    if self.current + 1 >= self.tokens.len() {
      return false;
    }

    match (&self.tokens[self.current + 1].kind, &kind) {
      (TokenKind::Number(_), TokenKind::Number(_)) => true,
      (TokenKind::String(_), TokenKind::String(_)) => true,
      (TokenKind::Boolean(_), TokenKind::Boolean(_)) => true,
      (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
      (a, b) if a == b => true,
      _ => false,
    }
  }

  fn advance(&mut self) -> Token {
    if !self.is_at_end() {
      self.current += 1;
    }

    self.previous()
  }

  fn is_at_end(&self) -> bool {
    matches!(self.peek().kind, TokenKind::Eof)
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.current]
  }

  fn previous(&self) -> Token {
    self.tokens[self.current - 1].clone()
  }

  fn synchronize(&mut self) {
    self.advance();

    while !self.is_at_end() {
      if matches!(self.previous().kind, TokenKind::Semicolon) {
        return;
      }

      match self.peek().kind {
        TokenKind::Function
        | TokenKind::Var
        | TokenKind::Let
        | TokenKind::Const
        | TokenKind::For
        | TokenKind::If
        | TokenKind::While
        | TokenKind::Print
        | TokenKind::Return => return,
        _ => {}
      }

      self.advance();
    }
  }
}
