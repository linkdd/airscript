use airscript::prelude::*;
use airscript::interpreter::{VM, FuncReturns};

#[derive(Debug, Clone)]
struct Vector {
  pub x: f64,
  pub y: f64,
}

fn vec_mag(vm: &mut VM<Vector>) -> Result<usize> {
  let v_ref = vm.pop_userdata()?;
  let v = v_ref.lock().unwrap();
  let m = (v.x * v.x + v.y * v.y).sqrt();
  vm.push_float(m);
  Ok(1)
}

#[test]
fn vm_works_with_udata() {
  let mut vm: VM<Vector> = VM::new();

  vm.push_userdata(Vector { x: 3f64, y: 4f64 });
  vm.push_rust_function(vec_mag);

  let call_res = vm.call(1, FuncReturns::Exactly(1));
  assert_eq!(call_res, Ok(()));

  let val_res = vm.pop_float();
  assert_eq!(val_res, Ok(5f64));
}
