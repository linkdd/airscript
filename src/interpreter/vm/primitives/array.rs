use std::sync::{Arc, Mutex};
use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # Array API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Create a new array and push it on the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn new_array(&mut self) {
    self.push_cell(Cell::Array(Arc::new(Mutex::new(vec![]))));
  }

  /// Check if the cell at position `pos` in the stack is an array.
  ///
  /// Stack: `[-0, +0]`
  pub fn is_array(&mut self, pos: usize) -> Result<bool> {
    match self.peek_cell(pos) {
      None => Err(AirError::StackIndexOutOfBounds),
      Some(&Cell::Array(..)) => Ok(true),
      _ => Ok(false),
    }
  }

  /// Push `a[k]` on the stack where `a` is the array at position `pos` and `k`
  /// an integer on top of the stack.
  ///
  /// Stack: `[-1, +1]`
  pub fn array_geti(&mut self, pos: usize) -> Result<()> {
    let i = self.pop_integer()?;
    let key = if i < 0 {
      return Err(AirError::NegativeArrayIndex);
    }
    else {
      i as usize
    };

    let new_cell = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(Cell::Array(array)) => {
        let new_cell = match array.lock().unwrap().get(key) {
          None => Cell::Nil,
          Some(cell) => cell.clone()
        };

        new_cell
      },
      _ => {
        return Err(AirError::TypeError(TypeErrorInfo::Array));
      },
    };

    self.push_cell(new_cell);
    Ok(())
  }

  /// Equivalent of `a[k] = v` where `v` is the value at the top of the stack,
  /// `k` is an integer bellow the top of the stack, and `a` the array at
  /// position `pos`.
  ///
  /// If the position exceed the array's size, the array is resized and new
  /// cells are set to `nil`.
  ///
  /// Stack: [-2, +0]
  pub fn array_seti(&mut self, pos: usize) -> Result<()> {
    let val = self.pop_cell()?;
    let i = self.pop_integer()?;
    let key = if i < 0 {
      return Err(AirError::NegativeArrayIndex);
    }
    else {
      i as usize
    };

    let mut array = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(Cell::Array(array)) => {
        array.lock().unwrap()
      },
      _ => {
        return Err(AirError::TypeError(TypeErrorInfo::Array));
      }
    };

    while key >= array.len() {
      array.push(Cell::Nil);
    }

    let cell = array.get_mut(key).expect("array should have correct size");
    *cell = val;

    Ok(())
  }

  /// Delete `a[k]` where `a` is the array at position `pos` and `k` an integer
  /// on top of the stack.
  ///
  /// Stack: `[-1, +0]`
  pub fn array_deli(&mut self, pos: usize) -> Result<()> {
    let i = self.pop_integer()?;
    let key = if i < 0 {
      return Err(AirError::NegativeArrayIndex);
    }
    else {
      i as usize
    };

    let mut array = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(Cell::Array(array)) => {
        array.lock().unwrap()
      },
      _ => {
        return Err(AirError::TypeError(TypeErrorInfo::Array));
      }
    };

    array.remove(key);

    Ok(())
  }
}
