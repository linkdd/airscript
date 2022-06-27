use crate::interpreter::{Dict, Array, RustFunction, ScopeTree, ByteCode};
use std::sync::{Arc, Mutex};

/// Represent a value on the stack
pub(crate) enum Cell<UD> where UD: Clone + 'static {
  /// Null value
  Nil,
  /// `true` or `false`
  Boolean(bool),
  /// 64-bits signed integer
  Integer(i64),
  /// 64-bits floating point number
  Float(f64),
  /// UTF-8 encoded string
  String(String),
  /// User defined Rust type, allocated on the heap
  UserData(Arc<Mutex<UD>>),
  /// Dictionnary
  Dict(Arc<Mutex<Dict<UD>>>),
  /// Array
  Array(Arc<Mutex<Array<UD>>>),
  /// Rust concrete function
  RustFunction(RustFunction<UD>),
  /// AirScript function (defined by a scope and a ByteCode sequence)
  AirFunction(Box<(ScopeTree<UD>, ByteCode<UD>, String)>),
}

impl<UD> Default for Cell<UD> where UD: Clone + 'static {
  fn default() -> Self {
    Self::Nil
  }
}

impl<UD> Clone for Cell<UD> where UD: Clone + 'static {
  fn clone(&self) -> Self {
    match self {
      Self::Nil => {
        Self::Nil
      },
      Self::Boolean(val) => {
        Self::Boolean(*val)
      },
      Self::Integer(val) => {
        Self::Integer(*val)
      },
      Self::Float(val) => {
        Self::Float(*val)
      },
      Self::String(val) => {
        Self::String(val.clone())
      },
      Self::UserData(udata) => {
        Self::UserData(udata.clone())
      },
      Self::Dict(dict) => {
        Self::Dict(dict.clone())
      },
      Self::Array(array) => {
        Self::Array(array.clone())
      },
      Self::RustFunction(func) => {
        Self::RustFunction(*func)
      },
      Self::AirFunction(func) => {
        Self::AirFunction(func.clone())
      }
    }
  }
}
