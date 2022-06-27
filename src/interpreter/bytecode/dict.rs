use crate::prelude::*;
use crate::interpreter::{VM, StackPosition, IRet};

pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn new(vm: &mut VM<UD>) -> Result<IRet> {
    vm.new_dict();
    Ok(IRet::Continue)
  }

  pub fn setfield(vm: &mut VM<UD>, pos: StackPosition) -> Result<IRet> {
    let spos = match pos {
      StackPosition::FromBase(n) => n,
      StackPosition::FromTop(n) => vm.get_top() - n,
    };

    vm.dict_setfield(spos)?;
    Ok(IRet::Continue)
  }
}
