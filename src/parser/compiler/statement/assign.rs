use string_interner::Symbol;

use crate::prelude::*;
use crate::interpreter::{ByteCode, OpCode};
use crate::parser::ast::*;

use super::super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn assign_location(&self, node: Node<AssignLocation>) -> Result<ByteCode<UD>> {
    match *node.data {
      AssignLocation::NewVariable(name) => {
        let sym = self.interner.lock().unwrap().get_or_intern(name);
        Ok(vec![OpCode::NewVariable(sym.to_usize())])
      },
      AssignLocation::SetVariable(target) => {
        match *target.data {
          Expression::Variable(name) => {
            let sym = self.interner.lock().unwrap().get_or_intern(name);
            Ok(vec![OpCode::SetVariable(sym.to_usize())])
          },
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
                bytecode.push(OpCode::SetTable);

                Ok(bytecode)
              },
              _ => {
                let (l, r) = target.location;

                Err(AirError::ParseError(
                  format!("{}:{}: operation does not target a table cell", l, r)
                ))
              }
            }
          },
          _ => {
            let (l, r) = node.location;

            Err(AirError::ParseError(
              format!("{}:{}: expression does not evaluate to a table cell", l, r)
            ))
          }
        }
      }
    }
  }
}
