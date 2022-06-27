use crate::prelude::*;
use crate::interpreter::{ByteCode, OpCode};
use crate::parser::ast::*;

use super::super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn expression_literal(&self, literal: Literal) -> Result<ByteCode<UD>> {
    match literal {
      Literal::Nil => {
        Ok(vec![OpCode::PushNil])
      },
      Literal::Boolean(val) => {
        Ok(vec![OpCode::PushBoolean(val)])
      },
      Literal::Integer(val) => {
        Ok(vec![OpCode::PushInteger(val)])
      },
      Literal::Float(val) => {
        Ok(vec![OpCode::PushFloat(val)])
      },
      Literal::String(val) => {
        Ok(vec![OpCode::PushString(val)])
      }
    }
  }
}
