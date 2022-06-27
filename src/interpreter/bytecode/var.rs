use crate::prelude::*;
use crate::interpreter::{VM, IRet};

pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn new(vm: &mut VM<UD>, var_name: usize) -> Result<IRet> {
    vm.new_interned_var(var_name)?;
    Ok(IRet::Continue)
  }

  pub fn get(vm: &mut VM<UD>, var_name: usize) -> Result<IRet> {
    vm.get_interned_var(var_name);
    Ok(IRet::Continue)
  }

  pub fn set(vm: &mut VM<UD>, var_name: usize) -> Result<IRet> {
    vm.set_interned_var(var_name)?;
    Ok(IRet::Continue)
  }
}
