use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_concat(vm: &mut VM<UD>) -> Result<IRet> {
    let rhs_val = vm.pop_string()?;
    let lhs_val = vm.pop_string()?;
    vm.push_string(format!("{lhs_val}{rhs_val}"));

    Ok(IRet::Continue)
  }
}
