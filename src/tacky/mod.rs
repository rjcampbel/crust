pub mod tacky;
mod tacky_printer;

use crate::parser::ast::{AST, Program, Stmt, Expr};
use crate::parser::ast;
use tacky::*;
use anyhow::Result;
use std::sync::atomic::{Ordering, AtomicUsize};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gen_tacky(ast: &AST, print_tacky: bool) -> Result<TackyAST> {
    let tacky_ast = gen_tacky_program(ast)?;

    if print_tacky {
        tacky_printer::print_tacky_ast(&tacky_ast);
    }

    Ok(tacky_ast)
}

fn gen_tacky_program(ast: &AST) -> Result<TackyAST> {
    match &ast.program {
        Program::Function { name, stmt } => {
            let program = gen_tacky_function(name, stmt)?;
            Ok(TackyAST { program })
        }
    }
}

fn gen_tacky_function(name: &String, stmt: &Stmt) -> Result<TackyProgram> {
    let mut instrs = Vec::new();
    match &stmt {
        Stmt::Return(expr) => {
            gen_return_instrs(expr, &mut instrs)?;
        }
    }
    Ok(TackyProgram::Function {
        identifier: name.clone(),
        body: instrs,
    })
}

fn gen_return_instrs(expr: &Expr, instrs: &mut Vec<Instr>) -> Result<()> {
    let val = gen_expr_instrs(expr, instrs)?;
    instrs.push(Instr::Return(val));
    Ok(())
}

fn gen_expr_instrs(expr: &Expr, instrs: &mut Vec<Instr>) -> Result<Val> {
    match &expr {
        Expr::Integer(i) => {
            Ok(Val::Integer(*i))
        },
        Expr::UnaryOp { operator, expr } => {
            let src = gen_expr_instrs(expr, instrs)?;
            let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
            let dest = Val::Var(format!("tmp.{}", counter));
            let unary_op = match operator {
                ast::UnaryOp::Negate => UnaryOp::Negate,
                ast::UnaryOp::Complement => UnaryOp::Complement,
            };
            instrs.push(Instr::Unary {
                operator: unary_op,
                src,
                dest: dest.clone(),
            });
            Ok(dest)
        },
        Expr::BinaryOp { operator, left, right } => {
            let left = gen_expr_instrs(left, instrs)?;
            let right = gen_expr_instrs(right, instrs)?;
            let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
            let dest = Val::Var(format!("tmp.{}", counter));
            let binary_op = match operator {
                ast::BinaryOp::Add => BinaryOp::Add,
                ast::BinaryOp::Subtract => BinaryOp::Subtract,
                ast::BinaryOp::Multiply => BinaryOp::Multiply,
                ast::BinaryOp::Divide => BinaryOp::Divide,
                ast::BinaryOp::Modulus => BinaryOp::Modulus,
                _ => unimplemented!(),
            };
            instrs.push(Instr::Binary {
                operator: binary_op,
                left,
                right,
                dest: dest.clone(),
            });
            Ok(dest)
        }
    }
}
