use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_mod(vm: &mut VM<UD>) -> Result<IRet> {
    let rhs_val = vm.pop_integer()?;
    let lhs_val = vm.pop_integer()?;
    vm.push_integer(lhs_val % rhs_val);

    Ok(IRet::Continue)
  }
}
