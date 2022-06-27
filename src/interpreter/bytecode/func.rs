use crate::prelude::*;
use crate::interpreter::{VM, ByteCode, FuncReturns, IRet};

pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn new(vm: &mut VM<UD>, func_name: String, bytecode: ByteCode<UD>) -> Result<IRet> {
    vm.push_air_function(
      vm.scope_tree.clone(),
      bytecode,
      func_name.clone(),
    );
    vm.new_var(func_name)?;

    Ok(IRet::Continue)
  }

  pub fn call(vm: &mut VM<UD>, nargs: usize, nreturns: FuncReturns) -> Result<IRet> {
    vm.call(nargs, nreturns)?;
    Ok(IRet::Continue)
  }
}
