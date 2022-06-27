//! # AirScript
//!
//! Like [Lua](https://lua.org), but in [Rust](https://rust-lang.org), and
//! different.
//!
//! **AirScript** is a general purpose, interpreted, stack-based and dynamically
//! typed programming language.
//!
//! It is available either as a REPL/interpreter, or as a crate to embed
//! directly within your projects.
//!
//! ## Examples
//!
//! Like **Lua**, it offers a strong integration with your project:
//!
//! ```rust
//! use airscript::prelude::*;
//! use airscript::interpreter::{VM, FuncReturns};
//!
//! fn square(vm: &mut VM<()>) -> Result<usize> {
//!   let a = vm.pop_integer()?;
//!   vm.push_integer(a * a);
//!   Ok(1)
//! }
//!
//! let mut vm: VM<()> = VM::new();
//!
//! vm.push_integer(2);
//! vm.push_rust_function(square);
//! vm.call(1, FuncReturns::Exactly(1)).unwrap();
//!
//! let res = vm.pop_integer().unwrap();
//! assert_eq!(res, 4);
//! ```
//!
//! Or with a custom Rust type:
//!
//! ```rust
//! use airscript::prelude::*;
//! use airscript::interpreter::{VM, FuncReturns};
//!
//! #[derive(Debug, Clone)]
//! struct Vector {
//!   pub x: f64,
//!   pub y: f64,
//! }
//!
//! fn vec_mag(vm: &mut VM<Vector>) -> Result<usize> {
//!   let v_ref = vm.pop_userdata()?;
//!   let v = v_ref.lock().unwrap();
//!   let m = (v.x * v.x + v.y * v.y).sqrt();
//!   vm.push_float(m);
//!   Ok(1)
//! }
//!
//! let mut vm: VM<Vector> = VM::new();
//!
//! vm.push_userdata(Vector { x: 3f64, y: 4f64 });
//! vm.push_rust_function(vec_mag);
//! vm.call(1, FuncReturns::Exactly(1)).unwrap();
//!
//! let res = vm.pop_float().unwrap();
//! assert_eq!(res, 5f64);
//! ```
//!
//! But unlike **Lua**, its syntax is inspired by Rust and Go:
//!
//! ```airscript
//! func make_action(kind) {
//!   func action(fn) {
//!     print(kind <> ": " <> fn() <> "\n");
//!   }
//!
//!   return action;
//! }
//!
//! func greeter(name) {
//!   func greet() {
//!     return "hello " <> name;
//!   }
//!
//!   return greet;
//! }
//!
//! let say := make_action("say");
//! let greet := greeter("world");
//!
//! say(greet);
//! ```
//!
//! ## Data Types
//!
//! **AirScript** provides the following datatypes:
//!
//!  - `nil`
//!  - booleans
//!  - integers (`i64`)
//!  - floating numbers (`f64`)
//!  - strings (`String`)
//!  - tables (similar to a JavaScript object)
//!  - arrays (indexes starts at `0`)
//!  - higher-order functions (functions are first-class citizens)
//!  - userdata (`Box<UD> where UD: Clone + 'static`)
//!
//! ## Operators
//!
//! **AirScript** provides the following operators:
//!
//!  - arithmetic: `+`, `-`, `*`, `/`, `//` (integer division), `%`
//!  - bitwise: `|`, `&`, `^`, `~`
//!  - comparison: `<`, `<=`, `=`, `!=`, `>=`, `>`
//!  - logical: `and`, `or`, `not`, `==>` (implication), `<==>` (bicondition)
//!  - string concatenation: `<>`
//!  - string/table/array length: `#`
//!  - table member access: `.`
//!  - assignment: `:=`

#[macro_use] extern crate lalrpop_util;
extern crate logos;

pub mod prelude;
pub(crate) mod parser;
pub mod interpreter;
pub(crate) mod builtins;
pub(crate) mod repl;

pub fn do_file(filepath: &str) -> crate::prelude::Result<()> {
  let mut vm: crate::interpreter::VM<()> = crate::interpreter::VM::new_debug();

  let ctx = vm.profiler.start_profiling("main");
  vm.openlibs()?;
  vm.load_file(filepath)?;
  vm.profiler.stop_profiling(ctx);

  vm.show_profiler_data();

  Ok(())
}
