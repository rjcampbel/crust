pub mod tacky;
mod tacky_printer;

use crate::parser::ast::{AST, Expr, FunctionDefinition, Program, Stmt};
use crate::parser::ast;
use tacky::*;
use anyhow::Result;
use anyhow::bail;
use std::sync::atomic::{Ordering, AtomicUsize};
use crate::name_generator;

static LBL_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn gen_label(name: &str) -> String {
    let counter = LBL_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}.{}", name, counter)
}

pub fn gen_tacky(ast: &AST, print_tacky: bool) -> Result<TackyAST> {
    let tacky_ast = gen_tacky_program(ast)?;

    if print_tacky {
        tacky_printer::print_tacky_ast(&tacky_ast);
    }

    Ok(tacky_ast)
}

fn gen_tacky_program(ast: &AST) -> Result<TackyAST> {
    match &ast.program {
        Program::FunctionDefinition(FunctionDefinition::Function(_name, _body)) => {
            // let program = gen_tacky_function(name, body)?;
            // Ok(TackyAST { program })
            todo!()
        }
    }
}

fn gen_tacky_function(name: &String, stmt: &Stmt) -> Result<TackyProgram> {
    let mut instrs = Vec::new();
    match &stmt {
        Stmt::Return(expr) => {
            gen_return_instrs(expr, &mut instrs)?;
        },
        _ => todo!()
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
        Expr::Integer(i, _) => {
            Ok(Val::Integer(*i))
        },
        Expr::UnaryOp { operator, expr, ..} => {
            let src = gen_expr_instrs(expr, instrs)?;
            let dest = Val::Var(name_generator::gen_tmp_name());
            let unary_op = match operator {
                ast::UnaryOp::Negate => UnaryOp::Negate,
                ast::UnaryOp::Complement => UnaryOp::Complement,
                ast::UnaryOp::Not => UnaryOp::Not,
            };
            instrs.push(Instr::Unary {
                operator: unary_op,
                src,
                dest: dest.clone(),
            });
            Ok(dest)
        },
        Expr::BinaryOp { operator: ast::BinaryOp::LogicalAnd, left, right, .. } => {
            gen_logical_and(left, right, instrs)
        },
        Expr::BinaryOp { operator: ast::BinaryOp::LogicalOr, left, right, .. } => {
            gen_logical_or(left, right, instrs)
        },
        Expr::BinaryOp { operator, left, right, .. } => {
            let left = gen_expr_instrs(left, instrs)?;
            let right = gen_expr_instrs(right, instrs)?;
            let dest = Val::Var(name_generator::gen_tmp_name());
            let binary_op = match operator {
                ast::BinaryOp::Add => BinaryOp::Add,
                ast::BinaryOp::Subtract => BinaryOp::Subtract,
                ast::BinaryOp::Multiply => BinaryOp::Multiply,
                ast::BinaryOp::Divide => BinaryOp::Divide,
                ast::BinaryOp::Modulus => BinaryOp::Modulus,
                ast::BinaryOp::BitwiseAnd => BinaryOp::BitwiseAnd,
                ast::BinaryOp::BitwiseOr => BinaryOp::BitwiseOr,
                ast::BinaryOp::BitwiseXor => BinaryOp::BitwiseXor,
                ast::BinaryOp::LeftShift => BinaryOp::LeftShift,
                ast::BinaryOp::RightShift => BinaryOp::RightShift,
                ast::BinaryOp::Equal => BinaryOp::Equal,
                ast::BinaryOp::NotEqual => BinaryOp::NotEqual,
                ast::BinaryOp::LessThan => BinaryOp::LessThan,
                ast::BinaryOp::LessOrEqual => BinaryOp::LessOrEqual,
                ast::BinaryOp::GreaterThan => BinaryOp::GreaterThan,
                ast::BinaryOp::GreaterOrEqual => BinaryOp::GreaterOrEqual,
                _ => bail!("Unsupported binary op")
            };
            instrs.push(Instr::Binary {
                operator: binary_op,
                left,
                right,
                dest: dest.clone(),
            });
            Ok(dest)
        },
        _ => todo!()
    }
}

fn gen_logical_and(left: &Box<Expr>, right: &Box<Expr>, instrs: &mut Vec<Instr>) -> Result<Val> {
    let left = gen_expr_instrs(left, instrs)?;
    let false_label = gen_label("false");
    let end_label = gen_label("end");
    instrs.push(Instr::JumpIfZero { condition: left, target: false_label.to_string() });
    let right = gen_expr_instrs(right, instrs)?;
    instrs.push(Instr::JumpIfZero { condition: right, target: false_label.to_string() });
    let dest = Val::Var(name_generator::gen_tmp_name());
    instrs.push(Instr::Copy { src: Val::Integer(1), dest: dest.clone() });
    instrs.push(Instr::Jump(end_label.to_string()));
    instrs.push(Instr::Label(false_label.to_string()));
    instrs.push(Instr::Copy { src: Val::Integer(0), dest: dest.clone() });
    instrs.push(Instr::Label(end_label.to_string()));
    Ok(dest)
}

fn gen_logical_or(left: &Box<Expr>, right: &Box<Expr>, instrs: &mut Vec<Instr>) -> Result<Val> {
    let left = gen_expr_instrs(left, instrs)?;
    let true_label = gen_label("true");
    let end_label = gen_label("end");
    instrs.push(Instr::JumpIfNotZero { condition: left, target: true_label.to_string() });
    let right = gen_expr_instrs(right, instrs)?;
    instrs.push(Instr::JumpIfNotZero { condition: right, target: true_label.to_string() });
    let dest = Val::Var(name_generator::gen_tmp_name());
    instrs.push(Instr::Copy { src: Val::Integer(0), dest: dest.clone() });
    instrs.push(Instr::Jump(end_label.to_string()));
    instrs.push(Instr::Label(true_label.to_string()));
    instrs.push(Instr::Copy { src: Val::Integer(1), dest: dest.clone() });
    instrs.push(Instr::Label(end_label.to_string()));
    Ok(dest)
}

