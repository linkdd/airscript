use crate::prelude::*;
use super::super::VM;

/// # Stack API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Get the size of the stack
  pub fn get_top(&self) -> usize {
    self.stack.len()
  }

  /// Push to the stack the value at position `pos` in the stack.
  ///
  /// Stack: `[-0, +1]`
  pub fn copy(&mut self, pos: usize) -> Result<()> {
    let new_cell = match self.peek_cell(pos) {
      None => {
        return Err(AirError::StackIndexOutOfBounds);
      },
      Some(cell) => cell.clone(),
    };

    self.push_cell(new_cell);

    Ok(())
  }

  /// Pop the value at the top of the stack and discard it.
  ///
  /// Stack `[-1, +0]`
  pub fn drop(&mut self) -> Result<()> {
    match self.stack.pop() {
      None => Err(AirError::EmptyStack),
      Some(..) => Ok(()),
    }
  }
}

