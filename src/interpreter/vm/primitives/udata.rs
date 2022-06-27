use std::sync::{Arc, Mutex};
use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # UserData API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push a custom Rust type on the stack.
  ///
  /// Ownership to the value is given to the VM and will be stored on the heap.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_userdata(&mut self, val: UD) {
    self.push_cell(Cell::UserData(Arc::new(Mutex::new(val))));
  }

  /// Pop and return a custom Rust type from the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn pop_userdata(&mut self) -> Result<Arc<Mutex<UD>>> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::UserData(val) => Ok(val),
      _ => Err(AirError::TypeError(TypeErrorInfo::UserData)),
    }
  }

  /// Check if the cell at position `pos` in the stack is a custom Rust type.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_userdata(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::UserData(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Get a non-mutable reference to the custom Rust type at position `pos` in
  /// the stack.
  ///
  /// Stack: `[-0, +0]`
  pub fn get_userdata(&mut self, pos: usize) -> Result<Arc<Mutex<UD>>> {
    match self.get_cell(pos)? {
      Cell::UserData(val) => Ok(val.clone()),
      _ => Err(AirError::TypeError(TypeErrorInfo::UserData)),
    }
  }
}
