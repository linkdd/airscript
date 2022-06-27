use crate::interpreter::Cell;

/// Represent a stack of `Cell`
pub(crate) type Stack<UD> = Vec<Cell<UD>>;
