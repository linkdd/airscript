use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_access(vm: &mut VM<UD>) -> Result<IRet> {
    let dict_idx = vm.get_top() - 2;
    vm.dict_getfield(dict_idx)?;

    let cell = vm.pop_cell()?;
    vm.drop()?;
    vm.push_cell(cell);

    Ok(IRet::Continue)
  }
}
