use std::rc::Rc;

pub struct TackyAST {
   pub program: TackyProgram,
}

pub enum TackyProgram {
   Function(Rc<String>, Vec<Instr>)
}

pub enum Instr {
   Return(Val),
   Unary(UnaryOp, Val, Val),
   Binary(BinaryOp, Val, Val, Val),
   Copy(Val, Val),
   Jump(Rc<String>),
   JumpIfZero(Val, Rc<String>),
   JumpIfNotZero(Val, Rc<String>),
   Label(Rc<String>),
}

#[derive(Clone, Debug)]
pub enum Val {
   Integer(i64),
   Var(Rc<String>),
}

#[derive(Debug)]
pub enum UnaryOp {
   Negate,
   Complement,
   Not,
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