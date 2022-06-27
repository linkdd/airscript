use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # Boolean API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push a boolean value on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_boolean(&mut self, val: bool) {
    self.push_cell(Cell::Boolean(val));
  }

  /// Pop and return a boolean value from the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn pop_boolean(&mut self) -> Result<bool> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::Boolean(val) => Ok(val),
      _ => Err(AirError::TypeError(TypeErrorInfo::Boolean)),
    }
  }

  /// Check if the cell at position `pos` in the stack is a boolean.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_boolean(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::Boolean(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Get a boolean at position `pos` in the stack.
  ///
  /// Stack: `[-0, +0]`
  pub fn get_boolean(&mut self, pos: usize) -> Result<bool> {
    match self.get_cell(pos)? {
      Cell::Boolean(val) => Ok(*val),
      _ => Err(AirError::TypeError(TypeErrorInfo::Boolean)),
    }
  }
}
