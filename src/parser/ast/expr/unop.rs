use super::super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
  Negation(Node<Expression>),
  Not(Node<Expression>),
  Length(Node<Expression>),
}

impl UnaryOperation {
  pub fn neg(expr: Node<Expression>) -> Self {
    Self::Negation(expr)
  }

  pub fn not(expr: Node<Expression>) -> Self {
    Self::Not(expr)
  }

  pub fn len(expr: Node<Expression>) -> Self {
    Self::Length(expr)
  }
}
