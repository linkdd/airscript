use crate::prelude::*;
use crate::interpreter::{VM, IRet};
use std::cmp::Ordering;

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_lt(vm: &mut VM<UD>) -> Result<IRet> {
    let top = vm.get_top();
    let lhs_index = top - 2;
    let rhs_index = top - 1;

    if vm.is_string(lhs_index)? {
      let rhs_val = vm.pop_string()?;
      let lhs_val = vm.pop_string()?;
      let result = match lhs_val.cmp(&rhs_val) {
        Ordering::Less => true,
        _ => false,
      };
      vm.push_boolean(result);
      Ok(IRet::Continue)
    }
    else if vm.is_integer(lhs_index)? {
      if vm.is_integer(rhs_index)? {
        let rhs_val = vm.pop_integer()?;
        let lhs_val = vm.pop_integer()?;
        vm.push_boolean(lhs_val < rhs_val);
        Ok(IRet::Continue)
      }
      else {
        let rhs_val = vm.pop_float()?;
        let lhs_val = vm.pop_integer()? as f64;
        vm.push_boolean(lhs_val < rhs_val);
        Ok(IRet::Continue)
      }
    }
    else if vm.is_float(lhs_index)? {
      if vm.is_integer(rhs_index)? {
        let rhs_val = vm.pop_integer()? as f64;
        let lhs_val = vm.pop_float()?;
        vm.push_boolean(lhs_val < rhs_val);
        Ok(IRet::Continue)
      }
      else {
        let rhs_val = vm.pop_float()?;
        let lhs_val = vm.pop_float()?;
        vm.push_boolean(lhs_val < rhs_val);
        Ok(IRet::Continue)
      }
    }
    else {
      Err(AirError::TypeError(TypeErrorInfo::Other(
        "comparison are only allowed for (string, string) and (number, number)".to_string()
      )))
    }
  }
}
