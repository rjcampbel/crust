use super::stack_allocator::StackAllocator;
use std::fmt;

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

impl fmt::Display for UnaryOp {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         UnaryOp::Neg => write!(f, "negl"),
         UnaryOp::Not => write!(f, "notl"),
      }
   }
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

impl fmt::Display for Register {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Register::AX => write!(f, "%eax"),
         Register::R10D => write!(f, "%r10d"),
      }
   }
}
