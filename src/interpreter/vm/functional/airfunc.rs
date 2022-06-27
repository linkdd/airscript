use crate::prelude::*;
use crate::interpreter::{Cell, ByteCode, ScopeTree};
use super::super::VM;

/// # AirFunction API
impl<UD> VM<UD> where UD: Clone + 'static {
  pub(crate) fn push_air_function(
    &mut self,
    scope_tree: ScopeTree<UD>,
    code: ByteCode<UD>,
    name: String,
  ) {
    self.push_cell(Cell::AirFunction(Box::new((scope_tree, code, name))));
  }

  /// Check if the Cell at position `pos` on the stack is an AirScript function.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_air_function(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::AirFunction(..)) => Ok(true),
      _ => Ok(false),
    }
  }
}
