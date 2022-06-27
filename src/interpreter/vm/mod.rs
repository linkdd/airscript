use string_interner::StringInterner;
use std::sync::{Arc, Mutex};

use crate::interpreter::{Stack, ScopeTree};

mod profiler;
use self::profiler::*;

/// AirScript virtual machine, used to execute code and interface with your Rust
/// program.
///
/// The VM takes a type parameter identify which Rust types can be managed by
/// the VM. The type must implement the `Clone` trait and have a `'static`
/// lifetime.
pub struct VM<UD> where UD: Clone + 'static {
  debug: bool,
  stack: Stack<UD>,
  pub(crate) scope_tree: ScopeTree<UD>,
  pub(crate) profiler: Profiler,
  pub(crate) interner: Arc<Mutex<StringInterner>>,
}

/// # Constructors
impl<UD> VM<UD> where UD: Clone + 'static {
  /// Create a new VM
  pub fn new() -> Self {
    let interner = Arc::new(Mutex::new(StringInterner::default()));

    Self {
      debug: false,
      stack: Stack::new(),
      scope_tree: ScopeTree::new(interner.clone()),
      interner,
      profiler: Profiler::new(),
    }
  }

  /// Create a new VM with debug mode
  pub fn new_debug() -> Self {
    let interner = Arc::new(Mutex::new(StringInterner::default()));

    Self {
      debug: true,
      stack: Stack::new(),
      scope_tree: ScopeTree::new(interner.clone()),
      interner,
      profiler: Profiler::new(),
    }
  }

  fn new_with_parent(
    debug: bool,
    stack: Stack<UD>,
    scope_tree: ScopeTree<UD>,
    interner: Arc<Mutex<StringInterner>>,
  ) -> Self {
    Self {
      debug,
      stack,
      scope_tree,
      interner,
      profiler: Profiler::new(),
    }
  }

  pub fn show_profiler_data(&self) {
    self.profiler.show();
  }
}

mod core;
mod loading;
mod primitives;
mod functional;
mod utils;
