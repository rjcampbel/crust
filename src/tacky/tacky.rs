pub struct TackyAST {
   pub program: TackyProgram,
}

pub struct TackyProgram {
   pub funcs: Vec<Function>,
}

pub struct Function {
   pub name: String,
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