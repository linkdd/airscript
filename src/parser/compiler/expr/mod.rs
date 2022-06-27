use string_interner::Symbol;

use crate::prelude::*;
use crate::interpreter::{ByteCode, OpCode, StackPosition, FuncReturns};
use crate::parser::ast::*;

use super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn expression(&self, node: Node<Expression>) -> Result<ByteCode<UD>> {
    match *node.data {
      Expression::BinaryOperation(op) => {
        self.expression_binop(op)
      },
      Expression::UnaryOperation(op) => {
        self.expression_unop(op)
      },
      Expression::FunctionCall {
        func: func_node,
        params: param_nodes,
      } => {
        let mut bytecode = ByteCode::new();

        let nargs = param_nodes.len();
        for param_node in param_nodes {
          let mut param_code = self.expression(param_node)?;
          bytecode.append(&mut param_code);
        }

        let mut func = self.expression(func_node)?;
        bytecode.append(&mut func);

        bytecode.push(OpCode::FunctionCall(nargs, FuncReturns::MultipleResults));

        Ok(bytecode)
      },
      Expression::Literal(term) => {
        self.expression_literal(term)
      },
      Expression::Variable(name) => {
        let sym = self.interner.lock().unwrap().get_or_intern(name);
        Ok(vec![OpCode::GetVariable(sym.to_usize())])
      },
      Expression::Dictionary(entry_nodes) => {
        let mut bytecode = ByteCode::new();

        bytecode.push(OpCode::NewDict);

        for (key, val_node) in entry_nodes {
          bytecode.push(OpCode::PushString(key));

          let mut val = self.expression(val_node)?;
          bytecode.append(&mut val);

          bytecode.push(OpCode::DictSetField(StackPosition::FromTop(3)));
        }

        Ok(bytecode)
      },
      Expression::Array(item_nodes) => {
        let mut bytecode = ByteCode::new();

        bytecode.push(OpCode::NewArray);

        for (idx, val_node) in item_nodes.into_iter().enumerate() {
          bytecode.push(OpCode::PushInteger(idx as i64));

          let mut val = self.expression(val_node)?;
          bytecode.append(&mut val);

          bytecode.push(OpCode::ArraySetIndex(StackPosition::FromTop(3)));
        }

        Ok(bytecode)
      },
    }
  }
}

mod binop;
mod unop;
mod literal;
