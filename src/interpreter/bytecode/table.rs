use crate::prelude::*;
use crate::interpreter::{VM, IRet};

pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn set(vm: &mut VM<UD>) -> Result<IRet> {
    let index = vm.pop_cell()?;
    let table = vm.pop_cell()?;
    let value = vm.pop_cell()?;

    vm.push_cell(table);
    vm.push_cell(index);
    vm.push_cell(value);

    let table_idx = vm.get_top() - 3;
    if vm.is_dict(table_idx)? {
      vm.dict_setfield(table_idx)?;
    }
    else if vm.is_array(table_idx)? {
      vm.array_seti(table_idx)?;
    }
    else {
      return Err(AirError::TypeError(TypeErrorInfo::Table));
    }

    vm.drop()?; // pop table

    Ok(IRet::Continue)
  }

  pub fn del(vm: &mut VM<UD>) -> Result<IRet> {
    let table_idx = vm.get_top() - 2;

    if vm.is_dict(table_idx)? {
      vm.push_nil();
      vm.dict_setfield(table_idx)?;
    }
    else if vm.is_array(table_idx)? {
      vm.array_deli(table_idx)?;
    }
    else {
      return Err(AirError::TypeError(TypeErrorInfo::Table));
    }

    vm.drop()?; // pop table

    Ok(IRet::Continue)
  }
}
