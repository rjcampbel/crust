use super::stack_allocator::StackAllocator;
use std::fmt;

pub struct AssemblyAST {
   pub program: AssemblyProgram,
}

impl fmt::Display for AssemblyAST {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.program)
   }
}

pub enum AssemblyProgram {
   Function {
      name: String,
      instructions: Vec<Instruction>,
      stack_allocator: StackAllocator
   },
}

impl fmt::Display for AssemblyProgram {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         AssemblyProgram::Function { name, instructions, .. } => {
            writeln!(f, "\t.globl _{}", name)?;
            writeln!(f, "_{}:", name)?;
            writeln!(f, "\tpushq\t%rbp")?;
            writeln!(f, "\tmovq\t%rsp, %rbp")?;
            for instr in instructions {
               writeln!(f, "{}", instr)?;
            }
            Ok(())
         }
      }
   }
}

#[derive(Clone)]
pub enum Instruction {
   Mov(Operand, Operand),
   Unary(UnaryOp, Operand),
   Binary(BinaryOp, Operand, Operand),
   Idiv(Operand),
   Cdq,
   AllocateStack(i64),
   Return
}

impl fmt::Display for Instruction {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Instruction::Mov(src, dest) => write!(f, "\tmovl {}, {}", src, dest),
         Instruction::Unary(op, operand) => write!(f, "\t{} {}", op, operand),
         Instruction::AllocateStack(i) => write!(f, "\tsubq ${}, %rsp", i),
         Instruction::Binary(op, left, right) => write!(f, "\t{} {}, {}", op, left, right),
         Instruction::Idiv(operand) => write!(f, "\tidivl {}", operand),
         Instruction::Cdq => write!(f, "\tcdq"),
         Instruction::Return => {
            writeln!(f, "\tmovq\t%rbp, %rsp")?;
            writeln!(f, "\tpopq\t%rbp")?;
            writeln!(f, "\tret")?;
            fmt::Result::Ok(())
         }
      }
   }
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


#[derive(Clone)]
pub enum BinaryOp {
   Add,
   Subt,
   Mult,
}

impl fmt::Display for BinaryOp {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         BinaryOp::Add => write!(f, "addl"),
         BinaryOp::Subt => write!(f, "subl"),
         BinaryOp::Mult => write!(f, "imull"),
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

impl fmt::Display for Operand {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Operand::Immediate(value) => write!(f, "${}", value),
         Operand::Register(r) => write!(f, "{}", r),
         Operand::Pseudo(name) => write!(f, "{}", name),
         Operand::Stack(i) => write!(f, "{}(%rbp)", i),
      }
   }
}

#[derive(Debug,Clone)]
pub enum Register {
   AX,
   DX,
   R10,
   R11
}

impl fmt::Display for Register {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Register::AX => write!(f, "%eax"),
         Register::DX => write!(f, "%edx"),
         Register::R10 => write!(f, "%r10d"),
         Register::R11 => write!(f, "%r11d"),
      }
   }
}
