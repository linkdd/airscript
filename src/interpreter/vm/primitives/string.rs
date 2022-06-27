use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # String API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push a string value on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_string(&mut self, val: String) {
    self.push_cell(Cell::String(val));
  }

  /// Push a string value on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_str(&mut self, val: &str) {
    self.push_cell(Cell::String(val.to_string()));
  }

  /// Pop and return a string value from the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn pop_string(&mut self) -> Result<String> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::String(val) => Ok(val),
      _ => Err(AirError::TypeError(TypeErrorInfo::String)),
    }
  }

  /// Check if the cell at position `pos` in the stack is a string.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_string(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::String(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Get a string at position `pos` in the stack.
  ///
  /// Stack: `[-0, +0]`
  pub fn get_string(&mut self, pos: usize) -> Result<String> {
    match self.get_cell(pos)? {
      Cell::String(val) => Ok(val.clone()),
      _ => Err(AirError::TypeError(TypeErrorInfo::String)),
    }
  }
}
