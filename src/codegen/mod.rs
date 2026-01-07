pub mod assembly;
mod assembly_printer;

use crate::parser::ast::*;
use anyhow::{bail, Result};
use assembly::*;
use assembly_printer::print_assembly_ast;

pub fn codegen(ast: &AST, print_assembly: bool) -> Result<AssemblyAST> {
   let assembly_ast = generate_assembly(&ast)?;
   if print_assembly {
      print_assembly_ast(&assembly_ast);
   }
   Ok(assembly_ast)
}

fn generate_assembly(ast: &AST) -> Result<AssemblyAST> {
   match &ast.program {
      Program::Function { name, stmt }=> {
         let function = generate_function(&name, &stmt)?;
         Ok(AssemblyAST { program: function })
      }
   }
}

fn generate_function(name: &String, stmt: &Stmt) -> Result<AssemblyProgram> {
   let instructions = generate_instructions(stmt)?;
   let assembly_function = AssemblyProgram::Function {
      name: name.clone(),
      instructions,
   };
   Ok(assembly_function)
}

fn generate_instructions(stmt: &Stmt) -> Result<Vec<Instruction>> {
   let mut instructions = Vec::new();
   match stmt {
      Stmt::Return(expr) => {
         instructions.push(generate_move_instruction(&expr)?);
         instructions.push(Instruction::Return);
         Ok(instructions)
      }
   }
}

fn generate_move_instruction(expr: &Expr) -> Result<Instruction> {
   match expr {
      Expr::Integer(value) => {
         Ok(Instruction::Mov(Operand::Immediate(*value), Operand::Register))
      },
      Expr::UnaryOp { .. } => {
         bail!("unsupported")
      }
   }
}
