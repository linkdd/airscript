use airscript::prelude::*;
use airscript::interpreter::{VM, FuncReturns};

fn sub(vm: &mut VM<()>) -> Result<usize> {
  if vm.get_top() < 2 {
    return Err(AirError::UserError("not enough arguments".to_string()));
  }

  let a = vm.get_integer(0)?;
  let b = vm.get_integer(1)?;

  vm.push_integer(a - b);

  Ok(1)
}

#[test]
fn vm_runs_rust_functions() {
  let mut vm = VM::new();

  vm.push_integer(1);
  vm.push_integer(2);
  vm.push_rust_function(sub);

  let call_res = vm.call(2, FuncReturns::Exactly(1));
  assert_eq!(call_res, Ok(()));

  let val_res = vm.pop_integer();
  assert_eq!(val_res, Ok(-1));

  assert_eq!(vm.get_top(), 0);
}

#[test]
fn call_fails_with_not_enough_arguments() {
  let mut vm = VM::new();

  vm.push_rust_function(sub);

  let call_noarg_res = vm.call(0, FuncReturns::Exactly(1));
  assert_eq!(call_noarg_res, Err(AirError::UserError("not enough arguments".to_string())));

  let call_arg_res = vm.call(2, FuncReturns::Exactly(1));
  assert_eq!(call_arg_res, Err(AirError::EmptyStack));
}

#[test]
#[should_panic]
fn calle_panics_on_errors() {
  let mut vm = VM::new();

  vm.push_rust_function(sub);
  vm.calle(2, FuncReturns::Exactly(1));
}
