use crate::prelude::*;
use crate::interpreter::ByteCode;

use string_interner::StringInterner;
use std::sync::{Arc, Mutex};

mod tokens;
mod lexer;
lalrpop_mod!(grammar, "/parser/grammar.rs");

pub mod ast;
pub mod compiler;

pub use self::{
  tokens::*,
  lexer::*,
  grammar::*,
};


pub struct Parser<UD> where UD: Clone + 'static {
  phantom: std::marker::PhantomData<UD>,
}

impl<UD> Parser<UD> where UD: Clone + 'static {
  pub fn new() -> Self {
    Self { phantom: std::marker::PhantomData }
  }

  pub fn parse(
    &mut self,
    input: &str,
    interner: Arc<Mutex<StringInterner>>,
  ) -> Result<ByteCode<UD>> {
    let lexer = Lexer::new(input);
    let parser: ChunkParser = ChunkParser::new();

    match parser.parse(self, lexer) {
      Ok(ast) => {
        let compiler = compiler::Compiler::new(interner);
        let bytecode = compiler.chunk(ast)?;
        Ok(bytecode)
      },
      Err(reason) => {
        match reason {
          lalrpop_util::ParseError::User { error } => Err(error),
          _ => Err(AirError::ParseError(format!("{:?}", reason)))
        }
      },
    }
  }
}
