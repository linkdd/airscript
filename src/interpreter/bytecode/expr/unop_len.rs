use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn unop_len(vm: &mut VM<UD>) -> Result<IRet> {
    let item_idx = vm.get_top() - 1;
    vm.len(item_idx)?;

    let cell = vm.pop_cell()?;
    vm.drop()?; // pop iterable
    vm.push_cell(cell);

    Ok(IRet::Continue)
  }
}
