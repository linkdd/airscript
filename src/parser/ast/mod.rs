#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
  pub location: (usize, usize),
  pub data: Box<T>,
}

impl<T> Node<T> {
  pub fn new(location: (usize, usize), data: Box<T>) -> Self {
    Self { location, data }
  }
}

pub mod statement;
pub mod expr;

pub use self::{
  statement::*,
  expr::*,
};
