use super::stack_allocator::StackAllocator;

pub struct AssemblyAST {
   pub program: AssemblyProgram,
}

pub enum AssemblyProgram {
   Function {
      name: String,
      instructions: Vec<Instruction>,
      stack_allocator: StackAllocator
   },
}

#[derive(Clone)]
pub enum Instruction {
   Mov(Operand, Operand),
   Unary(UnaryOp, Operand),
   AllocateStack(i64),
   Return
}

#[derive(Clone)]
pub enum UnaryOp {
   Neg,
   Not,
}

#[derive(Debug,Clone)]
pub enum Operand {
   Immediate(i64),
   Register(Register),
   Pseudo(String),
   Stack(i64),
}

#[derive(Debug,Clone)]
pub enum Register {
   AX,
   R10D,
}