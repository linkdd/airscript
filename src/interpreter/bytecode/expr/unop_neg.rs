use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn unop_neg(vm: &mut VM<UD>) -> Result<IRet> {
    let number_idx = vm.get_top() - 1;

    if vm.is_integer(number_idx)? {
      let i = vm.pop_integer()?;
      vm.push_integer(-1 * i);
    }
    else {
      let f = vm.pop_float()?;
      vm.push_float(-1.0 * f);
    }

    Ok(IRet::Continue)
  }
}
