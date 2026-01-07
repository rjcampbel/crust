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
}

#[derive(Clone, Debug)]
pub enum Val {
   Integer(u64),
   Var(String),
}

#[derive(Debug)]
pub enum UnaryOp {
   Negate,
   Complement,
}