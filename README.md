# AirScript

It's like [Lua](https://lua.org), but in [Rust](https://rust-lang.org), and
different.

## Introduction

AirScript is a dynamically typed, interpreted language inspired by **Lua** and
written in **Rust**.

Just like **Lua**, the interpreter is a stack-based virtual machine. The VM
provides an API to manipulate this stack, allowing you to:

 - push/pop primitive values
 - push/pop custom Rust types
 - push/pop (and call) Rust functions
 - ...

Example:

```rust
use airscript::prelude::*;
use airscript::interpreter::{VM, FuncReturns};

fn square(vm: &mut VM<()>) -> Result<usize> {
  let a = vm.pop_integer()?;
  vm.push_integer(a * a);
  Ok(1)
}

fn main() {
  let mut vm: VM<()> = VM::new();

  vm.push_integer(2);
  vm.push_rust_function(square);
  vm.call(1, FuncReturns::Exactly(1)).unwrap();

  let res = vm.pop_integer().unwrap();
  println!("{}", res); // prints: 4
}
```

Unlike **Lua**, the syntax is inspired by **Rust** and **Go**.

Example:

```
func make_action(kind) {
  func action(fn) {
    print(kind <> ": " <> fn() <> "\n");
  }

  return action;
}

func greeter(name) {
  func greet() {
    return "hello " <> name;
  }

  return greet;
}

let say := make_action("say");
let greet := greeter("world");

say(greet); // prints: say: hello world
```

To provide your own Rust types to the VM:

```rust
use airscript::prelude::*;
use airscript::interpreter::{VM, FuncReturns};

#[derive(Debug, Clone)]
struct Vector {
  pub x: f64,
  pub y: f64,
}

fn vec_mag(vm: &mut VM<Vector>) -> Result<usize> {
  let v_ref = vm.pop_userdata()?;
  let v = v.lock().unwrap();
  let m = (v.x * v.x + v.y * v.y).sqrt();
  vm.push_float(m);
  Ok(1)
}

fn main() {
  let mut vm: VM<Vector> = VM::new();

  vm.push_userdata(Vector { x: 3f64, y: 4f64 });
  vm.push_rust_function(vec_mag);
  vm.call(1, FuncReturns::Exactly(1)).unwrap();

  let res = vm.pop_float().unwrap();
  println!("{}", res); // prints: 5
}
```

## Disclaimer

This is still a **Work In Progress**:

 - the VM's API is not complete nor definitive
 - the syntax is not complete nor definitive
 - the semantics are not specified

## License

This project is released under the terms of the [MIT License](./LICENSE.txt).
