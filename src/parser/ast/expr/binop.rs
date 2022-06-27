use super::super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
  Access { lhs: Node<Expression>, rhs: String },
  Index { table: Node<Expression>, index: Node<Expression> },
  Multiply { lhs: Node<Expression>, rhs: Node<Expression> },
  Divide { lhs: Node<Expression>, rhs: Node<Expression> },
  Modulo { lhs: Node<Expression>, rhs: Node<Expression> },
  Add { lhs: Node<Expression>, rhs: Node<Expression> },
  Substract { lhs: Node<Expression>, rhs: Node<Expression> },
  Concat { lhs: Node<Expression>, rhs: Node<Expression> },
  LessThan { lhs: Node<Expression>, rhs: Node<Expression> },
  LessThanOrEqual { lhs: Node<Expression>, rhs: Node<Expression> },
  GreaterThan { lhs: Node<Expression>, rhs: Node<Expression> },
  GreaterThanOrEqual { lhs: Node<Expression>, rhs: Node<Expression> },
  Equal { lhs: Node<Expression>, rhs: Node<Expression> },
  NotEqual { lhs: Node<Expression>, rhs: Node<Expression> },
  And { lhs: Node<Expression>, rhs: Node<Expression> },
  Or { lhs: Node<Expression>, rhs: Node<Expression> },
}

impl BinaryOperation {
  pub fn access(lhs: Node<Expression>, rhs: String) -> Self {
    Self::Access { lhs, rhs }
  }

  pub fn index(table: Node<Expression>, index: Node<Expression>) -> Self {
    Self::Index { table, index }
  }

  pub fn mul(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Multiply { lhs, rhs }
  }

  pub fn div(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Divide { lhs, rhs }
  }

  pub fn modulo(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Modulo { lhs, rhs }
  }

  pub fn add(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Add { lhs, rhs }
  }

  pub fn sub(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Substract { lhs, rhs }
  }

  pub fn concat(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Concat { lhs, rhs }
  }

  pub fn lt(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::LessThan { lhs, rhs }
  }

  pub fn lte(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::LessThanOrEqual { lhs, rhs }
  }

  pub fn gt(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::GreaterThan { lhs, rhs }
  }

  pub fn gte(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::GreaterThanOrEqual { lhs, rhs }
  }

  pub fn eq(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Equal { lhs, rhs }
  }

  pub fn ne(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::NotEqual { lhs, rhs }
  }

  pub fn and(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::And { lhs, rhs }
  }

  pub fn or(lhs: Node<Expression>, rhs: Node<Expression>) -> Self {
    Self::Or { lhs, rhs }
  }
}
