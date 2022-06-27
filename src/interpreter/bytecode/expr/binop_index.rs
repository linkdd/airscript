use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_index(vm: &mut VM<UD>) -> Result<IRet> {
    let table_idx = vm.get_top() - 2;

    if vm.is_dict(table_idx)? {
      vm.dict_getfield(table_idx)?;
    }
    else if vm.is_array(table_idx)? {
      vm.array_geti(table_idx)?;
    }
    else {
      return Err(AirError::TypeError(TypeErrorInfo::Table));
    }

    let cell = vm.pop_cell()?;
    vm.drop()?; // pop table
    vm.push_cell(cell);

    Ok(IRet::Continue)
  }
}
