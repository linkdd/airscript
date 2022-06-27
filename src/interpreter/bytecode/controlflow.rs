use crate::prelude::*;
use crate::interpreter::{VM, ByteCode, IRet};

pub struct Operation<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Operation<UD> where UD: Clone + 'static {
  pub fn loop_while(vm: &mut VM<UD>, cond: &ByteCode<UD>, iteration: &ByteCode<UD>) -> Result<IRet> {
    loop {
      let ret = vm.load_bytecode(&cond)?;
      match ret {
        IRet::Continue => {},
        IRet::Returns(n) => {
          return Ok(IRet::Returns(n));
        },
      }

      let cond_res = vm.pop_boolean()?;
      if !cond_res {
        break;
      }

      vm.enter_scope();
      let ret = vm.load_bytecode(&iteration)?;
      vm.leave_scope();

      match ret {
        IRet::Continue => {},
        IRet::Returns(n) => {
          return Ok(IRet::Returns(n));
        },
      }
    }

    Ok(IRet::Continue)
  }

  pub fn cond(
    vm: &mut VM<UD>,
    cases: &Vec<(ByteCode<UD>, ByteCode<UD>)>,
    else_case: &Option<ByteCode<UD>>,
  ) -> Result<IRet> {
    let mut matched = false;

    for (cond, branch) in cases.iter() {
      let ret = vm.load_bytecode(cond)?;
      match ret {
        IRet::Continue => {},
        IRet::Returns(n) => {
          return Ok(IRet::Returns(n));
        },
      }

      let cond_res = vm.pop_boolean()?;
      if cond_res {
        vm.enter_scope();
        let ret = vm.load_bytecode(branch)?;
        vm.leave_scope();

        match ret {
          IRet::Continue => {},
          IRet::Returns(n) => {
            return Ok(IRet::Returns(n));
          },
        }

        matched = true;
        break;
      }
    }

    if !matched {
      if let Some(branch) = else_case {
        vm.enter_scope();
        let ret = vm.load_bytecode(branch)?;
        vm.leave_scope();

        match ret {
          IRet::Continue => {},
          IRet::Returns(n) => {
            return Ok(IRet::Returns(n));
          },
        }
      }
    }

    Ok(IRet::Continue)
  }
}
