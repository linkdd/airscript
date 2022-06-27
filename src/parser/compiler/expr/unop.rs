use crate::prelude::*;
use crate::interpreter::{ByteCode, OpCode};
use crate::parser::ast::*;

use super::super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn expression_unop(&self, op: UnaryOperation) -> Result<ByteCode<UD>> {
    match op {
      UnaryOperation::Negation(term_node) => {
        let mut bytecode = vec![];

        let mut term = self.expression(term_node)?;
        bytecode.append(&mut term);

        bytecode.push(OpCode::UnaryOpNegation);

        Ok(bytecode)
      },
      UnaryOperation::Not(term_node) => {
        let mut bytecode = vec![];

        let mut term = self.expression(term_node)?;
        bytecode.append(&mut term);

        bytecode.push(OpCode::UnaryOpNot);

        Ok(bytecode)
      },
      UnaryOperation::Length(term_node) => {
        let mut bytecode = vec![];

        let mut term = self.expression(term_node)?;
        bytecode.append(&mut term);

        bytecode.push(OpCode::UnaryOpLen);

        Ok(bytecode)
      },
    }
  }
}
