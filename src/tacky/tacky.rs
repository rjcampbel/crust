pub struct TackyAST {
   pub program: TackyProgram,
}

pub enum TackyProgram {
   Function {
      identifier: String,
      body: Vec<Instr>,
   }
}

pub enum Instr {
   Return(Val),
   Unary {
      operator: UnaryOp,
      src: Val,
      dest: Val,
   },
   Binary {
      operator: BinaryOp,
      left: Val,
      right: Val,
      dest: Val,
   },
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
}

#[derive(Debug)]
pub enum BinaryOp {
   Add,
   Subtract,
   Multiply,
   Divide,
   Modulus,
}