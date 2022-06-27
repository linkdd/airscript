use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_and(vm: &mut VM<UD>) -> Result<IRet> {
    let rhs_val = vm.pop_boolean()?;
    let lhs_val = vm.pop_boolean()?;
    vm.push_boolean(lhs_val && rhs_val);

    Ok(IRet::Continue)
  }
}
