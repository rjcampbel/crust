pub mod assembly;
mod assembly_printer;
mod stack_allocator;

use crate::codegen::assembly::*;
use crate::tacky::tacky::{BinaryOp, Instr, UnaryOp, TackyIR, Val};

use anyhow::Result;
use assembly_printer::print_assembly;
use stack_allocator::StackAllocator;

pub fn codegen(tacky: TackyIR, print: bool) -> Result<Assembly> {
   let assembly = generate_assembly(tacky)?;
   if print {
      print_assembly(&assembly);
   }
   Ok(assembly)
}

fn generate_assembly(tacky: TackyIR) -> Result<Assembly> {
   let mut functions = Vec::new();
   for func in &tacky.program.funcs {
      functions.push(generate_function(func.name.clone(), &func.params, &func.instrs)?);
   }
   let mut assembly = Assembly{ program: AssemblyProgram {functions} };
   replace_pseudoregisters(&mut assembly);
   fixup_instructions(&mut assembly);
   Ok(assembly)
}

fn generate_function(name: String, params: &Vec<String>, ir_instrs: &Vec<Instr>) -> Result<Function> {
   let mut instructions = Vec::new();
   for (i, param) in params.iter().enumerate() {
      let operand = Operand::Pseudo(param.clone());
      match i {
         0 => instructions.push(Instruction::Mov(Operand::Register(Register::DI(4)), operand)),
         1 => instructions.push(Instruction::Mov(Operand::Register(Register::SI(4)), operand)),
         2 => instructions.push(Instruction::Mov(Operand::Register(Register::DX(4)), operand)),
         3 => instructions.push(Instruction::Mov(Operand::Register(Register::CX(4)), operand)),
         4 => instructions.push(Instruction::Mov(Operand::Register(Register::R8(4)), operand)),
         5 => instructions.push(Instruction::Mov(Operand::Register(Register::R9(4)), operand)),
         pos @ _ => {
            instructions.push(Instruction::Mov(Operand::Stack(16 + 8 * ((pos - 6 ) as i64)), operand));
         }
      }
   }
   generate_function_instructions(ir_instrs, &mut instructions)?;
   let assembly_function = Function{ name, instructions, stack_allocator: StackAllocator::new() };
   Ok(assembly_function)
}

fn generate_function_instructions(ir_instrs: &Vec<Instr>, instructions: &mut Vec<Instruction>) -> Result<()> {
   for instr in ir_instrs {
      match instr {
         Instr::Return(val) => {
            let ret = generate_operand(val.clone());
            instructions.push(Instruction::Mov(ret, Operand::Register(Register::AX(4))));
            instructions.push(Instruction::Return);
         },
         Instr::Unary(UnaryOp::Not, src, dest) => {
            instructions.push(Instruction::Cmp(Operand::Immediate(0), generate_operand(src.clone())));
            instructions.push(Instruction::Mov(Operand::Immediate(0), generate_operand(dest.clone())));
            instructions.push(Instruction::SetCC(ConditionCode::E, generate_operand(dest.clone())));
         },
         Instr::Unary(operator, src, dest) => {
            let src = generate_operand(src.clone());
            let dst = generate_operand(dest.clone());
            let op = match operator {
               UnaryOp::Negate => {
                  assembly::UnaryOp::Neg
               },
               UnaryOp::Complement => {
                  assembly::UnaryOp::Not
               },
               _ => unreachable!()
            };
            instructions.push(Instruction::Mov(src, dst.clone()));
            instructions.push(Instruction::Unary(op, dst));
         },
         Instr::Binary(operator @ (BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::LessThan | BinaryOp::LessOrEqual | BinaryOp::GreaterThan | BinaryOp::GreaterOrEqual), left, right, dest) => {
            instructions.push(Instruction::Cmp(generate_operand(right.clone()), generate_operand(left.clone())));
            instructions.push(Instruction::Mov(Operand::Immediate(0), generate_operand(dest.clone())));
            let code = match operator {
               BinaryOp::Equal => ConditionCode::E,
               BinaryOp::NotEqual => ConditionCode::NE,
               BinaryOp::LessThan => ConditionCode::L,
               BinaryOp::LessOrEqual => ConditionCode::LE,
               BinaryOp::GreaterThan => ConditionCode::G,
               BinaryOp::GreaterOrEqual => ConditionCode::GE,
               _ => unreachable!()
            };
            instructions.push(Instruction::SetCC(code, generate_operand(dest.clone())));
         },
         Instr::Binary(operator, left, right, dest) => {
            let left = generate_operand(left.clone());
            let right = generate_operand(right.clone());
            let dst = generate_operand(dest.clone());
            match operator {
               BinaryOp::Add => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Add, right, dst));
               },
               BinaryOp::Subtract => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Sub, right, dst));
               },
               BinaryOp::Multiply => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Mult, right, dst));
               },
               BinaryOp::Divide => {
                  instructions.push(Instruction::Mov(left, Operand::Register(Register::AX(4))));
                  instructions.push(Instruction::Cdq);
                  instructions.push(Instruction::Idiv(right));
                  instructions.push(Instruction::Mov(Operand::Register(Register::AX(4)), dst));
               },
               BinaryOp::Modulus => {
                  instructions.push(Instruction::Mov(left, Operand::Register(Register::AX(4))));
                  instructions.push(Instruction::Cdq);
                  instructions.push(Instruction::Idiv(right));
                  instructions.push(Instruction::Mov(Operand::Register(Register::DX(4)), dst));
               },
               BinaryOp::BitwiseAnd => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseAnd, right, dst));
               },
               BinaryOp::BitwiseOr => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseOr, right, dst));
               },
               BinaryOp::BitwiseXor => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseXor, right, dst));
               },
               BinaryOp::LeftShift => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Shl(right, dst));
               },
               BinaryOp::RightShift => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Shr(right, dst));
               },
               _ => unreachable!()
            };
         },
         Instr::Jump(label) => {
            instructions.push(Instruction::Jmp(label.clone()));
         },
         Instr::JumpIfNotZero(condition, target) => {
            instructions.push(Instruction::Cmp(Operand::Immediate(0), generate_operand(condition.clone())));
            instructions.push(Instruction::JmpCC(ConditionCode::NE, target.clone()));
         },
         Instr::JumpIfZero(condition, target) => {
            instructions.push(Instruction::Cmp(Operand::Immediate(0), generate_operand(condition.clone())));
            instructions.push(Instruction::JmpCC(ConditionCode::E, target.clone()));
         },
         Instr::Copy(src, dest) => {
            instructions.push(Instruction::Mov(generate_operand(src.clone()), generate_operand(dest.clone())));
         },
         Instr::Label(label) => {
            instructions.push(Instruction::Label(label.clone()));
         },
         Instr::FuncCall(name, args, dest) => {
            let stack_args = args.iter().skip(6).rev();
            let stack_args_size = stack_args.len() as i64;

            let stack_padding =
               if stack_args_size % 2 == 1 {
                  instructions.push(Instruction::AllocateStack(8));
                  8
               } else {
                  0
               };

            for (i, arg) in args.iter().take(6).enumerate() {
               let operand = generate_operand(arg.clone());
               match i {
                  0 => instructions.push(Instruction::Mov(operand, Operand::Register(Register::DI(4)))),
                  1 => instructions.push(Instruction::Mov(operand, Operand::Register(Register::SI(4)))),
                  2 => instructions.push(Instruction::Mov(operand, Operand::Register(Register::DX(4)))),
                  3 => instructions.push(Instruction::Mov(operand, Operand::Register(Register::CX(4)))),
                  4 => instructions.push(Instruction::Mov(operand, Operand::Register(Register::R8(4)))),
                  5 => instructions.push(Instruction::Mov(operand, Operand::Register(Register::R9(4)))),
                  _ => unreachable!()
               }
            }

            for arg in stack_args {
               let operand = generate_operand(arg.clone());
               match operand {
                  Operand::Immediate(_) | Operand::Register(_) => instructions.push(Instruction::Push(operand)),
                  _ => {
                     instructions.push(Instruction::Mov(operand, Operand::Register(Register::AX(4))));
                     instructions.push(Instruction::Push(Operand::Register(Register::AX(8))));
                  }
               }
            }

            instructions.push(Instruction::Call(name.clone()));

            let dealloc_bytes = stack_args_size * 8 + stack_padding;
            if dealloc_bytes > 0 {
               instructions.push(Instruction::DeallocateStack(dealloc_bytes));
            }

            let dest = generate_operand(dest.clone());
            instructions.push(Instruction::Mov(Operand::Register(Register::AX(4)), dest));
         }
      }
   }
   Ok(())
}

fn generate_operand(val: Val) -> Operand {
   match val {
      Val::Integer(i) => Operand::Immediate(i),
      Val::Var(name) => Operand::Pseudo(name),
   }
}

fn replace_pseudoregisters(assembly: &mut Assembly) {
   for func in &mut assembly.program.functions {
      for instr in &mut func.instructions {
         match instr {
            Instruction::Mov(src, dst) => {
               convert_pseudo_stack(src, 4, &mut func.stack_allocator);
               convert_pseudo_stack(dst, 4, &mut func.stack_allocator);
            },
            Instruction::Unary(_, operand) => {
               convert_pseudo_stack(operand, 4, &mut func.stack_allocator);
            },
            Instruction::Binary(_, left, right) => {
               convert_pseudo_stack(left, 4, &mut func.stack_allocator);
               convert_pseudo_stack(right, 4, &mut func.stack_allocator);
            },
            Instruction::Idiv(operand) => {
               convert_pseudo_stack(operand, 4, &mut func.stack_allocator);
            },
            Instruction::Shl(dest, count) => {
               convert_pseudo_stack(dest, 4, &mut func.stack_allocator);
               convert_pseudo_stack(count, 4, &mut func.stack_allocator);
            },
            Instruction::Shr(dest, count) => {
               convert_pseudo_stack(dest, 4, &mut func.stack_allocator);
               convert_pseudo_stack(count, 4, &mut func.stack_allocator);
            },
            Instruction::Cmp(left, right) => {
               convert_pseudo_stack(left, 4, &mut func.stack_allocator);
               convert_pseudo_stack(right, 4, &mut func.stack_allocator);
            },
            Instruction::SetCC(_, operand) => {
               convert_pseudo_stack(operand, 4, &mut func.stack_allocator);
            },
            Instruction::Push(operand) => {
               convert_pseudo_stack(operand, 4, &mut func.stack_allocator);
            },
            _ => {}
         }
      }
   }
}

fn fixup_instructions(assembly: &mut Assembly) {
   for func in &mut assembly.program.functions {
      let stack_size = func.stack_allocator.get();
      let stack_size = ((stack_size + 15) / 16) * 16;
      let mut new_instructions = Vec::new();
      new_instructions.push(Instruction::AllocateStack(stack_size));

      for instr in &func.instructions {
         match instr {
            Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
               new_instructions.push(Instruction::Mov(Operand::Stack(*src), Operand::Register(Register::R10(4))));
               new_instructions.push(Instruction::Mov(Operand::Register(Register::R10(4)), Operand::Stack(*dst)));
            },
            Instruction::Movb(Operand::Stack(src), Operand::Stack(dst)) => {
               new_instructions.push(Instruction::Mov(Operand::Stack(*src), Operand::Register(Register::R10(4))));
               new_instructions.push(Instruction::Mov(Operand::Register(Register::R10(4)), Operand::Stack(*dst)));
            },
            Instruction::Idiv(Operand::Immediate(i)) => {
               new_instructions.push(Instruction::Mov(Operand::Immediate(*i), Operand::Register(Register::R10(4))));
               new_instructions.push(Instruction::Idiv(Operand::Register(Register::R10(4))));
            }
            Instruction::Binary(op @ (assembly::BinaryOp::Add | assembly::BinaryOp::Sub | assembly::BinaryOp::BitwiseAnd | assembly::BinaryOp::BitwiseOr | assembly::BinaryOp::BitwiseXor), Operand::Stack(src), Operand::Stack(dst)) => {
               new_instructions.push(Instruction::Mov(Operand::Stack(*src), Operand::Register(Register::R10(4))));
               new_instructions.push(Instruction::Binary(op.clone(), Operand::Register(Register::R10(4)), Operand::Stack(*dst)));
            },
            Instruction::Binary(assembly::BinaryOp::Mult, src @ _, dst @ Operand::Stack(_)) => {
               new_instructions.push(Instruction::Mov(dst.clone(), Operand::Register(Register::R11(4))));
               new_instructions.push(Instruction::Binary(assembly::BinaryOp::Mult, src.clone(), Operand::Register(Register::R11(4))));
               new_instructions.push(Instruction::Mov(Operand::Register(Register::R11(4)), dst.clone()));
            },
            Instruction::Shl(count @ Operand::Stack(_), dest @ _) => {
               new_instructions.push(Instruction::Movb(count.clone(), Operand::Register(Register::CX(1))));
               new_instructions.push(Instruction::Shl(Operand::Register(Register::CX(1)), dest.clone()));
            },
            Instruction::Shr(count @ Operand::Stack(_), dest @ _) => {
               new_instructions.push(Instruction::Movb(count.clone(), Operand::Register(Register::CX(1))));
               new_instructions.push(Instruction::Shr(Operand::Register(Register::CX(1)), dest.clone()));
            },
            Instruction::Cmp(Operand::Stack(left), Operand::Stack(right)) => {
               new_instructions.push(Instruction::Mov(Operand::Stack(*left), Operand::Register(Register::R10(4))));
               new_instructions.push(Instruction::Cmp(Operand::Register(Register::R10(4)), Operand::Stack(*right)));
            },
            Instruction::Cmp(left @ _, right @ Operand::Immediate(_)) => {
               new_instructions.push(Instruction::Mov(right.clone(), Operand::Register(Register::R10(4))));
               new_instructions.push(Instruction::Cmp(left.clone(), Operand::Register(Register::R10(4))));
            },
            i @ _ => {
               new_instructions.push(i.clone());
            }
         }
      }

      func.instructions = new_instructions;
   }
}

fn convert_pseudo_stack(pseudo: &mut Operand, size: i64, stack_allocator: &mut StackAllocator) {
   if let Operand::Pseudo(name) = pseudo {
      *pseudo = Operand::Stack(-stack_allocator.allocate(name.to_string(), size));
   };
}