//! Utility types

#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorInfo {
  Nil,
  Boolean,
  Integer,
  Float,
  /// Integer or Float
  Number,
  String,
  Array,
  Dict,
  /// Array or Dict
  Table,
  /// String or Table
  Iterable,
  AirFunction,
  RustFunction,
  /// AirFunction or RustFunction
  Function,
  UserData,
  Other(String),
}

/// Errors produced by the AirScript interpreter
#[derive(Debug, Clone, PartialEq)]
pub enum AirError {
  /// AirScript was unable to parse the source code
  ParseError(String),
  /// AirScript met an unknown token while parsing
  SyntaxError(String),
  /// The VM tried to pop from an empty stack
  EmptyStack,
  /// The VM tried to access a cell with a position outside of the stack bounds
  StackIndexOutOfBounds,
  /// The VM tried to access an array with a negative position
  NegativeArrayIndex,
  /// The VM expected a function to return more values that what was on the stack
  NotEnoughValues,
  /// The VM tried to access a variable that was not in scope
  NameNotFound(String),
  /// The VM expected a cell of another type
  TypeError(TypeErrorInfo),
  /// A user defined error
  UserError(String),
}

pub type Result<T> = std::result::Result<T, AirError>;
