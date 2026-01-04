mod assembly;
mod assembly_printer;

use crate::parser::ast;
use anyhow::Result;
use assembly::*;
use assembly_printer::print_assembly_ast;

pub fn codegen(program: &ast::Program) -> Result<()> {
   let assembly_program = generate_assembly(&program)?;
   print_assembly_ast(&assembly_program);
   Ok(())
}

fn generate_assembly(program: &ast::Program) -> Result<Program> {
   match program {
      ast::Program::Function(func) => {
         let function = generate_function(&func)?;
         Ok(Program::Function(function))
      }
   }
}

fn generate_function(func: &ast::Function) -> Result<(Function)> {
   let instructions = generate_instructions(&func.stmt)?;
   let assembly_function = Function {
      name: func.name.clone(),
      instructions,
   };
   Ok(assembly_function)
}

fn generate_instructions(stmt: &ast::Stmt) -> Result<Vec<Instruction>> {
   let mut instructions = Vec::new();
   match stmt {
      ast::Stmt::Return(expr) => {
         instructions.push(generate_move_instruction(&expr)?);
         instructions.push(Instruction::Return);
         Ok(instructions)
      }
   }
}

fn generate_move_instruction(expr: &ast::Expr) -> Result<Instruction> {
   match expr {
      ast::Expr::Integer(value) => {
         Ok(Instruction::Mov(Operand::Immediate(*value), Operand::Register))
      }
   }
}
