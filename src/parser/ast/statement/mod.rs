use super::*;

mod assign;

pub use self::{
  assign::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
  Chunk(Vec<Node<Statement>>),
  Assignment {
    locations: Vec<Node<AssignLocation>>,
    values: Vec<Node<Expression>>
  },
  Function {
    func_name: String,
    params: Vec<String>,
    body: Vec<Node<Statement>>,
  },
  While {
    cond: Node<Expression>,
    iteration: Vec<Node<Statement>>,
  },
  Cond {
    cases: Vec<(Node<Expression>, Vec<Node<Statement>>)>,
    else_case: Option<Vec<Node<Statement>>>,
  },
  Return(Vec<Node<Expression>>),
  Evaluation(Node<Expression>),
  Delete(Node<Expression>),
}

impl Statement {
  pub fn chunk(statements: Vec<Node<Statement>>) -> Box<Self> {
    Box::new(Self::Chunk(statements))
  }

  pub fn assign(
    locations: Vec<Node<AssignLocation>>,
    values: Vec<Node<Expression>>,
  ) -> Box<Self> {
    Box::new(Self::Assignment { locations, values })
  }

  pub fn function(
    func_name: String,
    params: Vec<String>,
    body: Vec<Node<Statement>>,
  ) -> Box<Self> {
    Box::new(Self::Function { func_name, params, body })
  }

  pub fn control_flow_while(
    cond: Node<Expression>,
    iteration: Vec<Node<Statement>>,
  ) -> Box<Self> {
    Box::new(Self::While { cond, iteration })
  }

  pub fn control_flow_cond(
    cases: Vec<(Node<Expression>, Vec<Node<Statement>>)>,
    else_case: Option<Vec<Node<Statement>>>,
  ) -> Box<Self> {
    Box::new(Self::Cond { cases, else_case })
  }

  pub fn return_vals(vals: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Return(vals))
  }

  pub fn eval(expression: Node<Expression>) -> Box<Self> {
    Box::new(Self::Evaluation(expression))
  }

  pub fn delete(table_index: Node<Expression>) -> Box<Self> {
    Box::new(Self::Delete(table_index))
  }
}
