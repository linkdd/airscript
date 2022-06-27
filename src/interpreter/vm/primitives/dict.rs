use std::sync::{Arc, Mutex};
use crate::prelude::*;
use crate::interpreter::{Cell, Dict};
use super::super::VM;

/// # Dict API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Create a new dict and push it on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn new_dict(&mut self) {
    self.push_cell(Cell::Dict(Arc::new(Mutex::new(Dict::new()))));
  }

  /// Check if the cell at position `pos` in the stack is a dict.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_dict(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::Dict(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Push `d[k]` on the stack where `d` is the dict at position `pos` and `k`
  /// a string on top of the stack.
  ///
  /// Stack: `[-1, +1]`
  pub fn dict_getfield(&mut self, pos: usize) -> Result<()> {
    let key = self.pop_string()?;

    let new_cell = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(Cell::Dict(dict)) => {
        let new_cell = match dict.lock().unwrap().get(&key) {
          None => Cell::Nil,
          Some(cell) => cell.clone()
        };

        new_cell
      },
      _ => {
        return Err(AirError::TypeError(TypeErrorInfo::Dict));
      },
    };

    self.push_cell(new_cell);
    Ok(())
  }

  /// Equivalent of `d[k] = v` where `v` is the value at the top of the stack,
  /// `k` is a string bellow the top of the stack, and `d` the dict at position
  /// `pos`.
  ///
  /// If the value `v` is a `nil` value, then the key `k` is removed from the
  /// dict.
  ///
  /// Stack: [-2, +0]
  pub fn dict_setfield(&mut self, pos: usize) -> Result<()> {
    let val = self.pop_cell()?;
    let key = self.pop_string()?;

    let mut dict = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(Cell::Dict(dict)) => {
        dict.lock().unwrap()
      },
      _ => {
        return Err(AirError::TypeError(TypeErrorInfo::Dict));
      }
    };

    match val {
      Cell::Nil => dict.remove(&key),
      _ => dict.insert(key, val),
    };

    Ok(())
  }
}
