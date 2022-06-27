use string_interner::Symbol;

use crate::prelude::*;
use super::super::VM;

/// # Variable API
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Pop the value at the top of the stack and assign it to a new variable.
  ///
  /// This potentially shadows a variable defined in a parent scope.
  ///
  /// Stack: `[-1, +0]`
  pub fn new_var(&mut self, var_name: String) -> Result<()> {
    let sym = self.interner.lock().unwrap().get_or_intern(var_name);
    self.new_interned_var(sym.to_usize())
  }

  pub(crate) fn new_interned_var(&mut self, var_name: usize) -> Result<()> {
    let cell = self.pop_cell()?;
    self.scope_tree.set_symbol(var_name, cell);
    Ok(())
  }

  /// Pop the value at the top the stack and assign it to an existing variable.
  ///
  /// The function will fail if the variable does not exist.
  ///
  /// Stack: `[-1, +0]`
  pub fn set_var(&mut self, var_name: String) -> Result<()> {
    let sym = self.interner.lock().unwrap().get_or_intern(var_name);
    self.set_interned_var(sym.to_usize())
  }

  pub(crate) fn set_interned_var(&mut self, var_name: usize) -> Result<()> {
    let cell = self.pop_cell()?;
    self.scope_tree.replace_symbol(var_name, cell)?;
    Ok(())
  }

  /// Push to the stack the value of the variable, or `nil` if the variable is
  /// not defined.
  ///
  /// Stack: `[-0, +1]`
  pub fn get_var(&mut self, var_name: String) {
    let sym = self.interner.lock().unwrap().get_or_intern(var_name);
    self.get_interned_var(sym.to_usize())
  }

  pub(crate) fn get_interned_var(&mut self, var_name: usize) {
    match self.scope_tree.lookup_symbol(var_name) {
      None => self.push_nil(),
      Some(cell) => self.push_cell(cell),
    }
  }

  pub(crate) fn enter_scope(&mut self) {
    self.scope_tree.enter_scope();
  }

  pub(crate) fn leave_scope(&mut self) {
    self.scope_tree.leave_scope();
  }
}
