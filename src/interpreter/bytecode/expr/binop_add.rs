use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_add(vm: &mut VM<UD>) -> Result<IRet> {
    let top = vm.get_top();
    let rhs_int = vm.is_integer(top - 1)?;
    let lhs_int = vm.is_integer(top - 2)?;

    match (lhs_int, rhs_int) {
      (true, true) => {
        let rhs_val = vm.pop_integer()?;
        let lhs_val = vm.pop_integer()?;
        vm.push_integer(lhs_val + rhs_val);
      },
      (false, true) => {
        let rhs_val = vm.pop_integer()? as f64;
        let lhs_val = vm.pop_float()?;
        vm.push_float(lhs_val + rhs_val);
      },
      (true, false) => {
        let rhs_val = vm.pop_float()?;
        let lhs_val = vm.pop_integer()? as f64;
        vm.push_float(lhs_val + rhs_val);
      },
      (false, false) => {
        let rhs_val = vm.pop_float()?;
        let lhs_val = vm.pop_float()?;
        vm.push_float(lhs_val + rhs_val);
      }
    };

    Ok(IRet::Continue)
  }
}
