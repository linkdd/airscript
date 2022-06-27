use super::*;

mod literal;
mod binop;
mod unop;

pub use self::{
  literal::*,
  binop::*,
  unop::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Literal(Literal),
  BinaryOperation(BinaryOperation),
  UnaryOperation(UnaryOperation),
  FunctionCall {
    func: Node<Expression>,
    params: Vec<Node<Expression>>,
  },
  Variable(String),
  Dictionary(Vec<(String, Node<Expression>)>),
  Array(Vec<Node<Expression>>),
}

impl Expression {
  pub fn literal(val: Literal) -> Box<Self> {
    Box::new(Self::Literal(val))
  }

  pub fn binop(op: BinaryOperation) -> Box<Self> {
    Box::new(Self::BinaryOperation(op))
  }

  pub fn unop(op: UnaryOperation) -> Box<Self> {
    Box::new(Self::UnaryOperation(op))
  }

  pub fn func_call(
    func: Node<Expression>,
    params: Vec<Node<Expression>>,
  ) -> Box<Self> {
    Box::new(Self::FunctionCall { func, params })
  }

  pub fn var(name: String) -> Box<Self> {
    Box::new(Self::Variable(name))
  }

  pub fn dict(entries: Vec<(String, Node<Expression>)>) -> Box<Self> {
    Box::new(Self::Dictionary(entries))
  }

  pub fn array(items: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Array(items))
  }
}
