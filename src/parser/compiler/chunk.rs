use crate::prelude::*;
use crate::interpreter::ByteCode;
use crate::parser::ast::*;

use super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn chunk(&self, node: Node<Statement>) -> Result<ByteCode<UD>> {
    match *node.data {
      Statement::Chunk(..) => {
        self.statement(node)
      },
      _ => {
        let (l, r) = node.location;
        Err(AirError::SyntaxError(format!("{}:{}: expected a chunk", l, r)))
      }
    }
  }
}
