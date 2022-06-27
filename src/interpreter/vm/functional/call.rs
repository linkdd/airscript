use crate::prelude::*;
use crate::interpreter::{Cell, Stack, ScopeTree, FuncReturns, IRet};
use super::super::VM;

/// # Function Call API
impl<UD> VM<UD> where UD: Clone + 'static {
  fn run_function(
    &mut self,
    scope_tree: ScopeTree<UD>,
    func: Box<dyn FnOnce(&mut VM<UD>) -> Result<usize>>,
    nargs: usize,
    nresults: FuncReturns,
  ) -> Result<()> {
    let mut local_stack: Stack<UD> = Stack::new();

    for _ in 0..nargs {
      let cell = self.pop_cell()?;
      local_stack.insert(0, cell);
    }

    let mut local_vm = VM::new_with_parent(
      self.debug,
      local_stack,
      scope_tree,
      self.interner.clone(),
    );
    let result_count = func(&mut local_vm)?;

    if result_count > local_vm.get_top() {
      return Err(AirError::NotEnoughValues);
    }

    let drop_count = local_vm.get_top() - result_count;
    if drop_count > 0 {
      local_vm.stack.drain(0..drop_count);
    }

    let copy_nresults = match nresults {
      FuncReturns::MultipleResults => local_vm.get_top(),
      FuncReturns::Exactly(n) => n,
    };

    let top = self.get_top();
    let mut result_stack = vec![];

    for _ in 0..copy_nresults {
      let cell = local_vm.pop_cell()?;
      result_stack.push(cell);
    }

    for cell in result_stack.into_iter().rev() {
      self.stack.insert(top, cell);
    }

    self.profiler.merge(&local_vm.profiler);

    Ok(())
  }

  /// Call the function at the top of the stack, with the arguments being the
  /// next `nargs` cells, and expected `nresults` return values.
  ///
  /// A new VM with an empty stack is created to run the function. The arguments
  /// are copied onto the new local stack and the return values are copied back
  /// onto the current stack, droping any leftovers on the local stack.
  ///
  /// Stack: `[-(nargs + 1), +nresults]`
  pub fn call(&mut self, nargs: usize, nresults: FuncReturns) -> Result<()> {
    let cell = self.pop_cell()?;

    match cell {
      Cell::RustFunction(func) => {
        let runner = Box::new(
          move |vm: &mut VM<UD>| -> Result<usize> {
            vm.enter_scope();
            let res = func(vm);
            vm.leave_scope();
            res
          }
        );
        self.run_function(ScopeTree::new(self.interner.clone()), runner, nargs, nresults)
      },
      Cell::AirFunction(func) => {
        let (scope_tree, bytecode, name) = *func;
        let func_scope = scope_tree.clone();

        let runner = Box::new(
          move |vm: &mut VM<UD>| -> Result<usize> {
            vm.enter_scope();

            vm.push_air_function(scope_tree.clone(), bytecode.clone(), name.clone());
            vm.new_var(name.clone())?;

            let res = vm.load_bytecode(&bytecode);
            vm.leave_scope();
            res.map(|ret| {
              match ret {
                IRet::Continue => 0,
                IRet::Returns(n) => n,
              }
            })
          }
        );
        self.run_function(func_scope, runner, nargs, nresults)
      },
      _ => {
        Err(AirError::TypeError(TypeErrorInfo::Function))
      }
    }
  }

  /// Similar to `call()` but panics if an error was returned.
  pub fn calle(&mut self, nargs: usize, nresults: FuncReturns) {
    self.call(nargs, nresults).unwrap();
  }
}
