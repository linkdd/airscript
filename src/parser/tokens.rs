use std::fmt;
use logos::Logos;
use snailquote::unescape;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
  // keywords
  #[token("let")]
  KeywordLet,

  #[token("func")]
  KeywordFunc,

  #[token("cond")]
  KeywordCond,

  #[token("else")]
  KeywordElse,

  #[token("while")]
  KeywordWhile,

  #[token("return")]
  KeywordReturn,

  #[token("delete")]
  KeywordDelete,

  #[token("do")]
  KeywordDo,

  #[token("nil")]
  KeywordNil,

  // symbols
  #[token("{")]
  CurlyBracketBegin,

  #[token("}")]
  CurlyBracketEnd,

  #[token("[")]
  BracketBegin,

  #[token("]")]
  BracketEnd,

  #[token("(")]
  ParenthesisBegin,

  #[token(")")]
  ParenthesisEnd,

  #[token("@")]
  At,

  #[token(",")]
  Comma,

  #[token(";")]
  SemiColon,

  #[token(":")]
  Colon,

  #[token("->")]
  Arrow,

  // operators
  #[token(":=")]
  OperatorAssign,

  #[token(".")]
  OperatorAccess,

  #[token("#")]
  OperaotrLen,

  #[token("-")]
  OperatorMinus,

  #[token("+")]
  OperatorPlus,

  #[token("*")]
  OperatorMul,

  #[token("/")]
  OperatorDiv,

  #[token("%")]
  OperatorMod,

  #[token("<>")]
  OperatorConcat,

  #[token("<")]
  OperatorLT,

  #[token("<=")]
  OperatorLTE,

  #[token(">")]
  OperatorGT,

  #[token(">=")]
  OperatorGTE,

  #[token("=")]
  OperatorEQ,

  #[token("!=")]
  OperatorNE,

  #[token("not")]
  OperatorNot,

  #[token("and")]
  OperatorAnd,

  #[token("or")]
  OperatorOr,

  // literals
  #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
  Identifier(String),

  #[token("true")]
  True,

  #[token("false")]
  False,

  #[regex(r"0b_*[01][_01]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  })]
  IntegerBase2(i64),

  #[regex(r"0o_*[0-7][_0-7]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  })]
  IntegerBase8(i64),

  #[regex(r"[0-9][_0-9]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  }, priority = 2)]
  IntegerBase10(i64),

  #[regex(r"0x_*[0-9a-fA-F][_0-9a-fA-F]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  })]
  IntegerBase16(i64),

  #[regex(r"((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?", |lex| {
    lex.slice().parse()
  })]
  Float(f64),

  #[regex("\"(?:[^\"]|\\\\\")*\"", |lex| {
    unescape(lex.slice())
  })]
  String(String),

  // comments and whitespaces
  #[regex(r"//.*\n?", logos::skip)]
	#[regex(r"[ \t\r\n\f]+", logos::skip)]
	#[error]
	Error,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
