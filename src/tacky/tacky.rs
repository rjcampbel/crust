use crate::validator::symbol_table::*;

pub struct TackyIR {
   pub program: TackyProgram,
   pub symbol_table: SymbolTable
}

pub struct TackyProgram {
   pub top_level: Vec<TopLevel>,
}

pub enum TopLevel {
   Function(Function),
   StaticVar(StaticVar),
}

pub struct StaticVar {
   pub name: String,
   pub global: bool,
   pub value: i64,
}

pub struct Function {
   pub name: String,
   pub global: bool,
   pub params: Vec<String>,
   pub instrs: Vec<Instr>,
}

pub enum Instr {
   Return(Val),
   Unary(UnaryOp, Val, Val),
   Binary(BinaryOp, Val, Val, Val),
   Copy(Val, Val),
   Jump(String),
   JumpIfZero(Val, String),
   JumpIfNotZero(Val, String),
   Label(String),
   FuncCall(String, Vec<Val>, Val),
}

#[derive(Clone, Debug)]
pub enum Val {
   Integer(i64),
   Var(String),
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
   Negate,
   Complement,
   Not,
   PreIncrement,
   PreDecrement,
   PostIncrement,
   PostDecrement,
}

#[derive(Debug)]
pub enum BinaryOp {
   Add,
   Subtract,
   Multiply,
   Divide,
   Modulus,
   BitwiseAnd,
   BitwiseOr,
   BitwiseXor,
   LeftShift,
   RightShift,
   Equal,
   NotEqual,
   LessThan,
   LessOrEqual,
   GreaterThan,
   GreaterOrEqual,
}