use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # Null API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push a null value on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_nil(&mut self) {
    self.push_cell(Cell::Nil);
  }

  /// Pop a nil value from the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn pop_nil(&mut self) -> Result<()> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::Nil => Ok(()),
      _ => Err(AirError::TypeError(TypeErrorInfo::Nil)),
    }
  }

  /// Check if the cell at position `pos` in the stack is a null value.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_nil(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::Nil) => Ok(true),
      _ => Ok(false),
    }
  }
}
