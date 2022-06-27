use string_interner::StringInterner;
use std::sync::{Arc, Mutex};

pub(crate) struct Compiler<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
  interner: Arc<Mutex<StringInterner>>,
}

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn new(interner: Arc<Mutex<StringInterner>>) -> Self {
    Self {
      phantom: std::marker::PhantomData,
      interner,
    }
  }
}

mod chunk;
mod statement;
mod expr;
