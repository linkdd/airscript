use crate::prelude::*;
use crate::builtins;
use super::super::VM;

/// # Library API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Register builtin functions into scope
  pub fn openlibs(&mut self) -> Result<()> {
    self.push_rust_function(builtins::to_string);
    self.new_var("str".to_string())?;

    self.push_rust_function(builtins::print);
    self.new_var("print".to_string())?;

    Ok(())
  }
}
