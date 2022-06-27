use crate::prelude::*;
use crate::interpreter::{VM, IRet};

pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn empty(vm: &mut VM<UD>) -> Result<IRet> {
    while vm.get_top() > 0 {
      vm.drop()?;
    }

    Ok(IRet::Continue)
  }

  pub fn push_nil(vm: &mut VM<UD>) -> Result<IRet> {
    vm.push_nil();
    Ok(IRet::Continue)
  }

  pub fn push_boolean(vm: &mut VM<UD>, val: bool) -> Result<IRet> {
    vm.push_boolean(val);
    Ok(IRet::Continue)
  }

  pub fn push_integer(vm: &mut VM<UD>, val: i64) -> Result<IRet> {
    vm.push_integer(val);
    Ok(IRet::Continue)
  }

  pub fn push_float(vm: &mut VM<UD>, val: f64) -> Result<IRet> {
    vm.push_float(val);
    Ok(IRet::Continue)
  }

  pub fn push_str(vm: &mut VM<UD>, val: &str) -> Result<IRet> {
    vm.push_str(val);
    Ok(IRet::Continue)
  }
}
