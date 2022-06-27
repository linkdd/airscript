use airscript::interpreter::VM;


#[test]
fn vm_loads_code() {
  let mut vm: VM<()> = VM::new();

  vm.openlibs().unwrap();
  let load_res = vm.load_string("let x := true; print(str(x));");
  assert_eq!(load_res, Ok(0));
}
