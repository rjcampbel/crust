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
   Copy {
      src: Val,
      dest: Val,
   },
   Jump(String),
   JumpIfZero {
      condition: Val,
      target: String,
   },
   JumpIfNotZero {
      condition: Val,
      target: String,
   },
   Label(String),
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
   LogicalAnd,
   LogicalOr,
}