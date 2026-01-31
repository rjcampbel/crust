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
   Function(String, Vec<Instruction>, StackAllocator),
}

impl fmt::Display for AssemblyProgram {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         AssemblyProgram::Function(name, instructions, ..) => {
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
   Movb(Operand, Operand),
   Unary(UnaryOp, Operand),
   Binary(BinaryOp, Operand, Operand),
   Cmp(Operand, Operand),
   Shl(Operand, Operand),
   Shr(Operand, Operand),
   Idiv(Operand),
   Cdq,
   Jmp(String),
   JmpCC(ConditionCode, String),
   SetCC(ConditionCode, Operand),
   Label(String),
   AllocateStack(i64),
   Return
}

impl fmt::Display for Instruction {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Instruction::Mov(src, dest) => write!(f, "\tmovl {}, {}", src, dest),
         Instruction::Movb(src, dest) => write!(f, "\tmovb {}, {}", src, dest),
         Instruction::Unary(op, operand) => write!(f, "\t{} {}", op, operand),
         Instruction::Binary(op, left, right) => write!(f, "\t{} {}, {}", op, left, right),
         Instruction::Shl(dst, count) => write!(f, "\tshll {}, {}", dst, count),
         Instruction::Shr(dst, count) => write!(f, "\tsarl {}, {}", dst, count),
         Instruction::Cmp(left, right) => write!(f, "\tcmpl {}, {}", left, right),
         Instruction::Cdq => write!(f, "\tcdq"),
         Instruction::Jmp(label) => write!(f, "\tjmp L{}", label),
         Instruction::JmpCC(condition, label) => write!(f, "\tj{} L{}", condition, label),
         Instruction::SetCC(condition, operand) => write!(f, "\tset{} {}", condition, operand),
         Instruction::Label(label) => write!(f, "L{}:", label),
         Instruction::Idiv(operand) => write!(f, "\tidivl {}", operand),
         Instruction::AllocateStack(i) => write!(f, "\tsubq ${}, %rsp", i),
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
   Sub,
   Mult,
   BitwiseAnd,
   BitwiseOr,
   BitwiseXor,
}

impl fmt::Display for BinaryOp {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         BinaryOp::Add => write!(f, "addl"),
         BinaryOp::Sub => write!(f, "subl"),
         BinaryOp::Mult => write!(f, "imull"),
         BinaryOp::BitwiseAnd => write!(f, "andl"),
         BinaryOp::BitwiseOr => write!(f, "orl"),
         BinaryOp::BitwiseXor => write!(f, "xorl"),
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

#[derive(Debug, Clone)]
pub enum ConditionCode {
   E,
   NE,
   G,
   GE,
   L,
   LE
}

impl fmt::Display for ConditionCode {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         ConditionCode::E => write!(f, "e"),
         ConditionCode::NE=> write!(f, "ne"),
         ConditionCode::G => write!(f, "g"),
         ConditionCode::GE => write!(f, "ge"),
         ConditionCode::L => write!(f, "l"),
         ConditionCode::LE => write!(f, "le")
      }
   }
}

#[derive(Debug,Clone)]
pub enum Register {
   AX,
   DX,
   R10,
   R11,
   CL,
}

impl fmt::Display for Register {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Register::AX => write!(f, "%eax"),
         Register::DX => write!(f, "%edx"),
         Register::R10 => write!(f, "%r10d"),
         Register::R11 => write!(f, "%r11d"),
         Register::CL => write!(f, "%cl"),
      }
   }
}
