use crate::prelude::*;
use crate::interpreter::{VM, IRet};

use super::Operation;

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn unop_not(vm: &mut VM<UD>) -> Result<IRet> {
    let b = vm.pop_boolean()?;
    vm.push_boolean(!b);

    Ok(IRet::Continue)
  }
}
