use logos::{Logos, SpannedIter};

use crate::prelude::*;
use crate::parser::tokens::Token;

pub type Spanned<Tok, Loc> = Result<(Loc, Tok, Loc)>;

pub struct Lexer<'input> {
  input: &'input str,
  token_stream: SpannedIter<'input, Token>
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    Self {
      input,
      token_stream: Token::lexer(input).spanned(),
    }
  }
}

impl<'input> Iterator for Lexer<'input> {
  type Item = Spanned<Token, usize>;

  fn next(&mut self) -> Option<Self::Item> {
    self.token_stream.next().map(|(token, span)| {
      match token {
        Token::Error => {
          Err(AirError::SyntaxError(
            format!(
              "{}:{}: unexpected token {}",
              span.start, span.end,
              self.input[span.clone()].to_string()
            )
          ))
        },
        _ => Ok((span.start, token, span.end))
      }
    })
  }
}
