use crate::codegen::assembly::*;

use std::fmt;

impl fmt::Display for Assembly {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.program)
   }
}

impl fmt::Display for AssemblyProgram {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for top_level in &self.top_level {
         match top_level {
            TopLevel::Function(func) => writeln!(f, "{}", func)?,
            TopLevel::StaticVar(var) => writeln!(f, "{}", var)?,
         }
      }
      Ok(())
   }
}

impl fmt::Display for StaticVar {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      if self.global {
         writeln!(f, "\t.globl _{}", self.name)?;
      }
      if self.value != 0 {
         writeln!(f, "\t.data")?;
         writeln!(f, "\t.balign 4")?;
         writeln!(f, "_{}:", self.name)?;
         writeln!(f, ".long {}", self.value)?;
      } else {
         writeln!(f, "\t.bss")?;
         writeln!(f, "\t.balign 4")?;
         writeln!(f, "_{}:", self.name)?;
         writeln!(f, "\t.zero 4")?;
      }
      Ok(())
   }
}

impl fmt::Display for Function {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      if self.global {
         writeln!(f, "\t.globl _{}", self.name)?;
      }
      writeln!(f, "\t.text")?;
      writeln!(f, "_{}:", self.name)?;
      writeln!(f, "\tpushq\t%rbp")?;
      writeln!(f, "\tmovq\t%rsp, %rbp")?;
      for instr in &self.instructions {
         writeln!(f, "{}", instr)?;
      }
      Ok(())
   }
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
         Instruction::DeallocateStack(i) => write!(f, "\taddq ${}, %rsp", i),
         Instruction::Push(operand) => write!(f, "\tpushq {}", operand),
         Instruction::Call(label) => write!(f, "\tcall _{}", label),
         Instruction::Return => {
            writeln!(f, "\tmovq\t%rbp, %rsp")?;
            writeln!(f, "\tpopq\t%rbp")?;
            writeln!(f, "\tret")?;
            fmt::Result::Ok(())
         }
      }
   }
}

impl fmt::Display for UnaryOp {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         UnaryOp::Neg => write!(f, "negl"),
         UnaryOp::Not => write!(f, "notl"),
      }
   }
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

impl fmt::Display for Operand {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Operand::Immediate(value) => write!(f, "${}", value),
         Operand::Register(r) => write!(f, "{}", r),
         Operand::Pseudo(name) => write!(f, "{}", name),
         Operand::Stack(i) => write!(f, "{}(%rbp)", i),
         Operand::Data(name) => write!(f, "_{}(%rip)", name)
      }
   }
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

impl fmt::Display for Register {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Register::AX(size) => {
            match size {
               8 => write!(f, "%rax"),
               4 => write!(f, "%eax"),
               1 => write!(f, "%al"),
               _ => unreachable!()
            }
         },
         Register::DX(size) => {
            match size {
               8 => write!(f, "%rdx"),
               4 => write!(f, "%edx"),
               1 => write!(f, "%dl"),
               _ => unreachable!()
            }
         },
         Register::CX(size) => {
            match size {
               8 => write!(f, "%rcx"),
               4 => write!(f, "%ecx"),
               1 => write!(f, "%cl"),
               _ => unreachable!()
            }
         },
         Register::DI(size) => {
            match size {
               8 => write!(f, "%rdi"),
               4 => write!(f, "%edi"),
               1 => write!(f, "%dil"),
               _ => unreachable!()
            }
         },
         Register::SI(size) => {
            match size {
               8 => write!(f, "%rsi"),
               4 => write!(f, "%esi"),
               1 => write!(f, "%sil"),
               _ => unreachable!()
            }
         },
         Register::R8(size) => {
            match size {
               8 => write!(f, "%r8"),
               4 => write!(f, "%r8d"),
               1 => write!(f, "%r8b"),
               _ => unreachable!()
            }
         },
         Register::R9(size) => {
            match size {
               8 => write!(f, "%r9"),
               4 => write!(f, "%r9d"),
               1 => write!(f, "%r9b"),
               _ => unreachable!()
            }
         },
         Register::R10(size) => {
            match size {
               8 => write!(f, "%r10"),
               4 => write!(f, "%r10d"),
               1 => write!(f, "%r10b"),
               _ => unreachable!()
            }
         },
         Register::R11(size) => {
            match size {
               8 => write!(f, "%r11"),
               4 => write!(f, "%r11d"),
               1 => write!(f, "%r11b"),
               _ => unreachable!()
            }
         }
      }
   }
}
