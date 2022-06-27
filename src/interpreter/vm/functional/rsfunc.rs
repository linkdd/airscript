use crate::prelude::*;
use crate::interpreter::{Cell, RustFunction};
use super::super::VM;

/// # Rust functions API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push a Rust concrete function on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn push_rust_function(&mut self, func: RustFunction<UD>) {
    self.push_cell(Cell::RustFunction(func));
  }

  /// Check if the cell at position `pos` in the stack is a Rust concrete
  /// function.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_rust_function(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::RustFunction(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Get the Rust concrete function at position `pos` in the stack.
  ///
  /// Stack: `[-0, +0]`
  pub fn get_rust_function(&mut self, pos: usize) -> Result<RustFunction<UD>> {
    match self.get_cell(pos)? {
      Cell::RustFunction(func) => Ok(*func),
      _ => Err(AirError::TypeError(TypeErrorInfo::RustFunction)),
    }
  }
}
