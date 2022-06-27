use string_interner::Symbol;

use crate::prelude::*;
use crate::interpreter::{ByteCode, OpCode};
use crate::parser::ast::*;

use super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn statement(&self, node: Node<Statement>) -> Result<ByteCode<UD>> {
    match *node.data {
      Statement::Chunk(statements) => {
        let mut bytecode = vec![];

        for statement in statements {
          let mut code = self.statement(statement)?;
          bytecode.append(&mut code);
        }

        Ok(bytecode)
      },
      Statement::Assignment {
        locations,
        values,
      } => {
        let mut bytecode = vec![];

        for value in values.into_iter().rev() {
          let mut code = self.expression(value)?;
          bytecode.append(&mut code);
        }

        for location in locations {
          let mut code = self.assign_location(location)?;
          bytecode.append(&mut code);
        }

        Ok(bytecode)
      },
      Statement::Function {
        func_name,
        params,
        body,
      } => {
        let mut body_code = vec![];

        for param in params.into_iter().rev() {
          let sym = self.interner.lock().unwrap().get_or_intern(param);
          body_code.push(OpCode::NewVariable(sym.to_usize()));
        }

        for statement in body {
          let mut code = self.statement(statement)?;
          body_code.append(&mut code);
        }

        body_code.push(OpCode::PushNil);
        body_code.push(OpCode::Return(1));

        Ok(vec![OpCode::NewFunction(func_name, body_code)])
      },
      Statement::While {
        cond,
        iteration,
      } => {
        let cond_code = self.expression(cond)?;
        let mut iteration_code = vec![];

        for statement in iteration {
          let mut statement_code = self.statement(statement)?;
          iteration_code.append(&mut statement_code);
        }

        Ok(vec![OpCode::While(cond_code, iteration_code)])
      },
      Statement::Cond {
        cases,
        else_case,
      } => {
        let mut cases_code = vec![];

        for (cond, branch) in cases {
          let cond_code = self.expression(cond)?;
          let mut branch_code = vec![];

          for statement in branch {
            let mut statement_code = self.statement(statement)?;
            branch_code.append(&mut statement_code);
          }

          cases_code.push((cond_code, branch_code));
        }

        let else_case_code = match else_case {
          Some(branch) => {
            let mut else_case_code = vec![];

            for statement in branch {
              let mut statement_code = self.statement(statement)?;
              else_case_code.append(&mut statement_code);
            }

            Some(else_case_code)
          },
          None => {
            None
          }
        };

        Ok(vec![OpCode::Cond(cases_code, else_case_code)])
      },
      Statement::Return(expressions) => {
        let nreturns = expressions.len();
        let mut bytecode = vec![];

        for expression in expressions {
          let mut code = self.expression(expression)?;
          bytecode.append(&mut code);
        }

        bytecode.push(OpCode::Return(nreturns));

        Ok(bytecode)
      },
      Statement::Evaluation(expression) => {
        let mut bytecode = self.expression(expression)?;
        bytecode.push(OpCode::EmptyStack);

        Ok(bytecode)
      },
      Statement::Delete(table_index) => {
        match *table_index.data {
          Expression::BinaryOperation(op) => {
            match op {
              BinaryOperation::Index {
                table: table_node,
                index: index_node,
              } => {
                let mut table = self.expression(table_node)?;
                let mut index = self.expression(index_node)?;

                let mut bytecode = vec![];
                bytecode.append(&mut table);
                bytecode.append(&mut index);
                bytecode.push(OpCode::DelTable);

                Ok(bytecode)
              },
              _ => {
                let (l, r) = table_index.location;

                Err(AirError::SyntaxError(
                  format!("{}:{}: operation does not target a table cell", l, r)
                ))
              }
            }
          },
          _ => {
            let (l, r) = node.location;

            Err(AirError::SyntaxError(
              format!("{}:{}: expression does not evaluate to a table cell", l, r)
            ))
          }
        }
      },
    }
  }
}

mod assign;
