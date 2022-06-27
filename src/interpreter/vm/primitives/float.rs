use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # Floating point numbers API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push a float value on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_float(&mut self, val: f64) {
    self.push_cell(Cell::Float(val));
  }

  /// Pop and return a float value from the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn pop_float(&mut self) -> Result<f64> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::Float(val) => Ok(val),
      _ => Err(AirError::TypeError(TypeErrorInfo::Float)),
    }
  }

  /// Check if the cell at position `pos` in the stack is a float.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_float(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::Float(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Get a float at position `pos` in the stack.
  ///
  /// Stack: `[-0, +0]`
  pub fn get_float(&mut self, pos: usize) -> Result<f64> {
    match self.get_cell(pos)? {
      Cell::Float(val) => Ok(*val),
      _ => Err(AirError::TypeError(TypeErrorInfo::Float)),
    }
  }
}
