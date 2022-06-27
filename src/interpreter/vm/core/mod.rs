use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

impl<UD> VM<UD> where UD: Clone + 'static {
  pub(crate) fn push_cell(&mut self, cell: Cell<UD>) {
    self.stack.push(cell);
  }

  pub(crate) fn pop_cell(&mut self) -> Result<Cell<UD>> {
    match self.stack.pop() {
      None => Err(AirError::EmptyStack),
      Some(cell) => Ok(cell),
    }
  }

  pub(crate) fn peek_cell(&self, pos: usize) -> Option<&Cell<UD>> {
    self.stack.get(pos)
  }

  pub(crate) fn get_cell(&self, pos: usize) -> Result<&Cell<UD>> {
    match self.stack.get(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(cell) => Ok(cell),
    }
  }
}
