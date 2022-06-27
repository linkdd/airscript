//! # AirScript interpreter
//!
//! The interpreter provides:
//!
//!  - a stack used to coordinate the program's execution
//!  - a tree of scope used to keep track of variables

mod data;
mod bytecode;
mod vm;

pub(crate) use self::data::*;
pub(crate) use self::bytecode::*;

pub use self::data::FuncReturns;
pub use self::vm::VM;
