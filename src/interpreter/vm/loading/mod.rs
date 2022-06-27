use crate::prelude::*;
use crate::interpreter::{ByteCode, IRet};
use crate::parser::Parser;
use super::VM;

/// # Source code loading API
impl<UD> VM<UD> where UD: Clone + 'static {
  pub(crate) fn load_bytecode(&mut self, bytecode: &ByteCode<UD>) -> Result<IRet> {
    for opcode in bytecode.iter() {
      match opcode.eval(self)? {
        IRet::Continue => {},
        IRet::Returns(n) => {
          return Ok(IRet::Returns(n));
        }
      }
    }

    Ok(IRet::Continue)
  }

  /// Parse and execute code from a string, returns the number of return values
  /// at the top of the stack.
  pub fn load_string(&mut self, source_code: &str) -> Result<usize> {
    let mut parser: Parser<UD> = Parser::new();
    let bytecode = parser.parse(
      source_code,
      self.interner.clone()
    )?;

    self.load_bytecode(&bytecode).map(|ret| {
      match ret {
        IRet::Continue => 0,
        IRet::Returns(n) => n,
      }
    })
  }

  /// Parse and execute code from a file, returns the number of return values
  /// at the top of the stack.
  pub fn load_file<P: AsRef<std::path::Path>>(&mut self, filepath: P) -> Result<usize> {
    let source_code = match std::fs::read_to_string(filepath) {
      Ok(code) => code,
      Err(reason) => {
        return Err(AirError::ParseError(format!("{}", reason)));
      },
    };

    self.load_string(&source_code)
  }
}
