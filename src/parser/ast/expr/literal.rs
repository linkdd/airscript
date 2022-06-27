#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
}

impl Literal {
  pub fn nil() -> Self {
    Self::Nil
  }

  pub fn boolean(val: bool) -> Self {
    Self::Boolean(val)
  }

  pub fn int(val: i64) -> Self {
    Self::Integer(val)
  }

  pub fn float(val: f64) -> Self {
    Self::Float(val)
  }

  pub fn string(val: String) -> Self {
    Self::String(val)
  }
}
