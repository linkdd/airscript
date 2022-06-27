use string_interner::{StringInterner, Symbol, symbol::SymbolU32};
use std::sync::{Arc, Mutex};

use crate::prelude::*;
use crate::interpreter::Cell;

use std::collections::HashMap;

#[derive(Clone)]
struct Scope<UD> where UD: Clone + 'static {
  symbols: HashMap<usize, Cell<UD>>,
}


impl<UD> Scope<UD> where UD: Clone + 'static {
  pub fn new() -> Self {
    Self { symbols: HashMap::new() }
  }
}


/// Tree of scope. Each scope is a HashMap associating a name to a `Cell`.
#[derive(Clone)]
pub(crate) struct ScopeTree<UD> where UD: Clone + 'static {
  tree: Vec<Scope<UD>>,
  interner: Arc<Mutex<StringInterner>>,
}

impl<UD> ScopeTree<UD> where UD: Clone + 'static {
  /// Construct a new tree
  pub fn new(interner: Arc<Mutex<StringInterner>>) -> Self {
    Self { tree: vec![Scope::new()], interner }
  }

  /// Set the value for a symbol in the current scope.
  ///
  /// This potentially shadows a symbol defined in a parent scope.
  pub fn set_symbol(&mut self, symbol_name: usize, value: Cell<UD>) {
    if let Some(current) = self.tree.last_mut() {
      current.symbols.insert(symbol_name, value);
    }
    else {
      unreachable!();
    }
  }

  /// Get the value for a symbol if it exists in the tree.
  pub fn lookup_symbol(&self, symbol_name: usize) -> Option<Cell<UD>> {
    for scope in self.tree.iter().rev() {
      match scope.symbols.get(&symbol_name) {
        Some(value) => {
          return Some(value.clone());
        },
        None => {}
      }
    }

    None
  }

  /// Sets the value of a symbol without shadowing it in the current scope.
  ///
  /// This function fails if the symbol was not defined.
  pub fn replace_symbol(&mut self, symbol_name: usize, value: Cell<UD>) -> Result<()> {
    let mut symbol_scope = None;

    for scope in self.tree.iter_mut().rev() {
      match scope.symbols.get(&symbol_name) {
        Some(..) => {
          symbol_scope = Some(scope);
          break;
        },
        None => {}
      }
    }

    match symbol_scope {
      None => {
        let interner = self.interner.lock().unwrap();
        let human_name = interner.resolve(
          SymbolU32::try_from_usize(symbol_name).unwrap()
        ).unwrap();
        Err(AirError::NameNotFound(human_name.to_string()))
      },
      Some(scope) => {
        scope.symbols.insert(symbol_name, value.clone());
        Ok(())
      },
    }
  }

  /// Enters a new scope.
  ///
  /// The current scope becomes the parent of the new scope.
  pub fn enter_scope(&mut self) {
    self.tree.push(Scope::new());
  }

  /// Leaves the current scope.
  ///
  /// The parent scope becomes the current scope.
  pub fn leave_scope(&mut self) {
    self.tree.pop();
  }
}
