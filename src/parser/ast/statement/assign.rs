use super::super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AssignLocation {
  NewVariable(String),
  SetVariable(Node<Expression>),
}

impl AssignLocation {
  pub fn new_var(name: String) -> Box<Self> {
    Box::new(Self::NewVariable(name))
  }

  pub fn set_var(name: Node<Expression>) -> Box<Self> {
    Box::new(Self::SetVariable(name))
  }
}
