use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # Integer numbers API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push an integer value on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_integer(&mut self, val: i64) {
    self.push_cell(Cell::Integer(val));
  }

  /// Pop and return an integer value from the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn pop_integer(&mut self) -> Result<i64> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::Integer(val) => Ok(val),
      _ => Err(AirError::TypeError(TypeErrorInfo::Integer)),
    }
  }

  /// Check if the cell at position `pos` in the stack is an integer.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_integer(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::Integer(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Get an integer at position `pos` in the stack.
  ///
  /// Stack: `[-0, +0]`
  pub fn get_integer(&mut self, pos: usize) -> Result<i64> {
    match self.get_cell(pos)? {
      Cell::Integer(val) => Ok(*val),
      _ => Err(AirError::TypeError(TypeErrorInfo::Integer)),
    }
  }
}
