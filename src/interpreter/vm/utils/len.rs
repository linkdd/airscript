use crate::prelude::*;
use crate::interpreter::Cell;
use super::super::VM;

/// # Length API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Push to the stack the length of the string or table at the top of the
  /// stack.
  ///
  /// Stack `[-0, +1]`
  pub fn len(&mut self, pos: usize) -> Result<()> {
    let size = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(Cell::String(val)) => {
        val.len()
      },
      Some(Cell::Dict(dict)) => {
        dict.lock().unwrap().len()
      },
      Some(Cell::Array(array)) => {
        array.lock().unwrap().len()
      },
      _ => {
        return Err(AirError::TypeError(TypeErrorInfo::Iterable));
      }
    };

    self.push_integer(size as i64);

    Ok(())
  }
}
