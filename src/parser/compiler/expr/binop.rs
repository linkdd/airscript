use crate::prelude::*;
use crate::interpreter::{ByteCode, OpCode};
use crate::parser::ast::*;

use super::super::Compiler;

impl<UD> Compiler<UD> where UD: Clone + 'static {
  pub fn expression_binop(&self, op: BinaryOperation) -> Result<ByteCode<UD>> {
    match op {
      BinaryOperation::Access {
        lhs: lhs_node,
        rhs,
      } => {
        let mut lhs = self.expression(lhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.push(OpCode::PushString(rhs));
        bytecode.push(OpCode::BinaryOpAccess);

        Ok(bytecode)
      },
      BinaryOperation::Index {
        table: table_node,
        index: index_node,
      } => {
        let mut table = self.expression(table_node)?;
        let mut index = self.expression(index_node)?;

        let mut bytecode = vec![];
        bytecode.append(&mut table);
        bytecode.append(&mut index);
        bytecode.push(OpCode::BinaryOpIndex);

        Ok(bytecode)
      },
      BinaryOperation::Multiply {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpMul);

        Ok(bytecode)
      },
      BinaryOperation::Divide {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpDiv);

        Ok(bytecode)
      },
      BinaryOperation::Modulo {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpMod);

        Ok(bytecode)
      },
      BinaryOperation::Add {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpAdd);

        Ok(bytecode)
      },
      BinaryOperation::Substract {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpSub);

        Ok(bytecode)
      },
      BinaryOperation::Concat {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpConcat);

        Ok(bytecode)
      },
      BinaryOperation::LessThan {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpLt);

        Ok(bytecode)
      },
      BinaryOperation::LessThanOrEqual {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpLte);

        Ok(bytecode)
      },
      BinaryOperation::GreaterThan {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpGt);

        Ok(bytecode)
      },
      BinaryOperation::GreaterThanOrEqual {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpGte);

        Ok(bytecode)
      },
      BinaryOperation::Equal {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpEq);

        Ok(bytecode)
      },
      BinaryOperation::NotEqual {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpNe);

        Ok(bytecode)
      },
      BinaryOperation::And {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpAnd);

        Ok(bytecode)
      },
      BinaryOperation::Or {
        lhs: lhs_node,
        rhs: rhs_node,
      } => {
        let mut lhs = self.expression(lhs_node)?;
        let mut rhs = self.expression(rhs_node)?;

        let mut bytecode = vec![];

        bytecode.append(&mut lhs);
        bytecode.append(&mut rhs);
        bytecode.push(OpCode::BinaryOpOr);

        Ok(bytecode)
      },
    }
  }
}
