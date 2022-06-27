use crate::prelude::*;
use crate::interpreter::{VM, FuncReturns};

pub type ByteCode<UD> = Vec<OpCode<UD>>;

pub enum IRet {
  Continue,
  Returns(usize),
}

#[derive(Clone)]
pub enum StackPosition {
  FromTop(usize),
  FromBase(usize),
}

#[derive(Clone)]
pub enum OpCode<UD> where UD: Clone + 'static {
  NoOp(std::marker::PhantomData<UD>),

  EmptyStack,
  PushNil,
  PushBoolean(bool),
  PushInteger(i64),
  PushFloat(f64),
  PushString(String),

  UnaryOpNegation,
  UnaryOpNot,
  UnaryOpLen,

  BinaryOpAccess,
  BinaryOpIndex,
  BinaryOpMul,
  BinaryOpDiv,
  BinaryOpMod,
  BinaryOpAdd,
  BinaryOpSub,
  BinaryOpConcat,
  BinaryOpLt,
  BinaryOpLte,
  BinaryOpGt,
  BinaryOpGte,
  BinaryOpEq,
  BinaryOpNe,
  BinaryOpAnd,
  BinaryOpOr,

  NewVariable(usize),
  GetVariable(usize),
  SetVariable(usize),

  SetTable,
  DelTable,

  FunctionCall(usize, FuncReturns),
  NewFunction(String, ByteCode<UD>),
  Return(usize),

  While(ByteCode<UD>, ByteCode<UD>),
  Cond(Vec<(ByteCode<UD>, ByteCode<UD>)>, Option<ByteCode<UD>>),

  NewDict,
  DictSetField(StackPosition),

  NewArray,
  ArraySetIndex(StackPosition),
}

mod stack;
mod var;
mod func;
mod controlflow;
mod table;
mod dict;
mod array;
mod expr;

impl<UD> OpCode<UD> where UD: Clone + 'static {
  pub fn eval(&self, vm: &mut VM<UD>) -> Result<IRet> {
    match self {
      Self::NoOp(..) => {
        let ctx = vm.profiler.start_profiling("NoOp");
        let res = Ok(IRet::Continue);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::EmptyStack => {
        let ctx = vm.profiler.start_profiling("EmptyStack");
        let res = stack::Operation::empty(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::PushNil => {
        let ctx = vm.profiler.start_profiling("PushNil");
        let res = stack::Operation::push_nil(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::PushBoolean(val) => {
        let ctx = vm.profiler.start_profiling("PushBoolean");
        let res = stack::Operation::push_boolean(vm, *val);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::PushInteger(val) => {
        let ctx = vm.profiler.start_profiling("PushInteger");
        let res = stack::Operation::push_integer(vm, *val);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::PushFloat(val) => {
        let ctx = vm.profiler.start_profiling("PushFloat");
        let res = stack::Operation::push_float(vm, *val);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::PushString(val) => {
        let ctx = vm.profiler.start_profiling("PushString");
        let res = stack::Operation::push_str(vm, val);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::UnaryOpNegation => {
        let ctx = vm.profiler.start_profiling("UnaryOpNegation");
        let res = expr::Operation::unop_neg(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::UnaryOpNot => {
        let ctx = vm.profiler.start_profiling("UnaryOpNot");
        let res = expr::Operation::unop_not(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::UnaryOpLen => {
        let ctx = vm.profiler.start_profiling("UnaryOpLen");
        let res = expr::Operation::unop_len(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpAccess => {
        let ctx = vm.profiler.start_profiling("BinaryOpAccess");
        let res = expr::Operation::binop_access(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpIndex => {
        let ctx = vm.profiler.start_profiling("BinaryOpIndex");
        let res = expr::Operation::binop_index(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpMul => {
        let ctx = vm.profiler.start_profiling("BinaryOpMul");
        let res = expr::Operation::binop_mul(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpDiv => {
        let ctx = vm.profiler.start_profiling("BinaryOpDiv");
        let res = expr::Operation::binop_div(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpMod => {
        let ctx = vm.profiler.start_profiling("BinaryOpMod");
        let res = expr::Operation::binop_mod(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpAdd => {
        let ctx = vm.profiler.start_profiling("BinaryOpAdd");
        let res = expr::Operation::binop_add(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpSub => {
        let ctx = vm.profiler.start_profiling("BinaryOpSub");
        let res = expr::Operation::binop_sub(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpConcat => {
        let ctx = vm.profiler.start_profiling("BinaryOpConcat");
        let res = expr::Operation::binop_concat(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpLt => {
        let ctx = vm.profiler.start_profiling("BinaryOpLt");
        let res = expr::Operation::binop_lt(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpLte => {
        let ctx = vm.profiler.start_profiling("BinaryOpLte");
        let res = expr::Operation::binop_lte(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpGt => {
        let ctx = vm.profiler.start_profiling("BinaryOpGt");
        let res = expr::Operation::binop_gt(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpGte => {
        let ctx = vm.profiler.start_profiling("BinaryOpGte");
        let res = expr::Operation::binop_gte(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpEq => {
        let ctx = vm.profiler.start_profiling("BinaryOpEq");
        let res = expr::Operation::binop_eq(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpNe => {
        let ctx = vm.profiler.start_profiling("BinaryOpNe");
        let res = expr::Operation::binop_ne(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpAnd => {
        let ctx = vm.profiler.start_profiling("BinaryOpAnd");
        let res = expr::Operation::binop_and(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::BinaryOpOr => {
        let ctx = vm.profiler.start_profiling("BinaryOpOr");
        let res = expr::Operation::binop_or(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::NewVariable(var_name) => {
        let ctx = vm.profiler.start_profiling("NewVariable");
        let res = var::Operation::new(vm, *var_name);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::GetVariable(var_name) => {
        let ctx = vm.profiler.start_profiling("GetVariable");
        let res = var::Operation::get(vm, *var_name);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::SetVariable(var_name) => {
        let ctx = vm.profiler.start_profiling("SetVariable");
        let res = var::Operation::set(vm, *var_name);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::SetTable => {
        let ctx = vm.profiler.start_profiling("SetTable");
        let res = table::Operation::set(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::DelTable => {
        let ctx = vm.profiler.start_profiling("DelTable");
        let res = table::Operation::del(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::FunctionCall(nargs, nreturns) => {
        let ctx = vm.profiler.start_profiling("FunctionCall");
        let res = func::Operation::call(vm, *nargs, nreturns.clone());
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::NewFunction(name, bytecode) => {
        let ctx = vm.profiler.start_profiling("NewFunction");
        let res = func::Operation::new(vm, name.clone(), bytecode.clone());
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::Return(nreturns) => {
        let ctx = vm.profiler.start_profiling("Return");
        let res = Ok(IRet::Returns(*nreturns));
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::While(cond, iteration) => {
        let ctx = vm.profiler.start_profiling("While");
        let res = controlflow::Operation::loop_while(vm, cond, iteration);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::Cond(cases, else_case) => {
        let ctx = vm.profiler.start_profiling("Cond");
        let res = controlflow::Operation::cond(vm, cases, else_case);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::NewDict => {
        let ctx = vm.profiler.start_profiling("NewDict");
        let res = dict::Operation::new(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::DictSetField(pos) => {
        let ctx = vm.profiler.start_profiling("DictSetField");
        let res = dict::Operation::setfield(vm, pos.clone());
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::NewArray => {
        let ctx = vm.profiler.start_profiling("NewArray");
        let res = array::Operation::new(vm);
        vm.profiler.stop_profiling(ctx);
        res
      },
      Self::ArraySetIndex(pos) => {
        let ctx = vm.profiler.start_profiling("ArraySetIndex");
        let res = array::Operation::setindex(vm, pos.clone());
        vm.profiler.stop_profiling(ctx);
        res
      },
    }
  }
}
