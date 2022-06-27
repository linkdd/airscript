use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn binop_ne(vm: &mut VM<UD>) -> Result<IRet> {
    let top = vm.get_top();
    let lhs_index = top - 2;
    let rhs_index = top - 1;

    if vm.is_string(lhs_index)? {
      let rhs_val = vm.pop_string()?;
      let lhs_val = vm.pop_string()?;
      vm.push_boolean(lhs_val != rhs_val);
      Ok(IRet::Continue)
    }
    else if vm.is_integer(lhs_index)? {
      if vm.is_integer(rhs_index)? {
        let rhs_val = vm.pop_integer()?;
        let lhs_val = vm.pop_integer()?;
        vm.push_boolean(lhs_val != rhs_val);
        Ok(IRet::Continue)
      }
      else {
        let rhs_val = vm.pop_float()?;
        let lhs_val = vm.pop_integer()? as f64;
        vm.push_boolean(lhs_val != rhs_val);
        Ok(IRet::Continue)
      }
    }
    else if vm.is_float(lhs_index)? {
      if vm.is_integer(rhs_index)? {
        let rhs_val = vm.pop_integer()? as f64;
        let lhs_val = vm.pop_float()?;
        vm.push_boolean(lhs_val != rhs_val);
        Ok(IRet::Continue)
      }
      else {
        let rhs_val = vm.pop_float()?;
        let lhs_val = vm.pop_float()?;
        vm.push_boolean(lhs_val != rhs_val);
        Ok(IRet::Continue)
      }
    }
    else if vm.is_nil(lhs_index)? {
      let isnil = vm.is_nil(rhs_index)?;
      vm.push_boolean(!isnil);
      Ok(IRet::Continue)
    }
    else {
      vm.push_boolean(true);
      Ok(IRet::Continue)
    }
  }
}
