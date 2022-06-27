use crate::prelude::*;
use crate::interpreter::VM;

/// Represent a user-defined function.
///
/// It accepts the current VM as argument, to interact with the AirScript
/// program.
/// It returns the number of cells on the stack that should be returned.
pub type RustFunction<UD> = fn(&mut VM<UD>) -> Result<usize>;

/// Represent the expected number of return value of a function.
#[derive(Clone)]
pub enum FuncReturns {
  /// Everything that is on the stack are the return values
  MultipleResults,
  /// The top N cells on the stack are the return values
  Exactly(usize),
}
