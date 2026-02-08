pub mod tacky;
mod tacky_printer;

use crate::name_generator::{self, gen_label};
use crate::parser::ast;
use crate::parser::ast::{AST, BlockItem, Expr, FunctionDefinition, Program, Stmt, ForInit};
use tacky::*;

use anyhow::bail;
use anyhow::Result;

pub fn gen_tacky(ast: AST, print_tacky: bool) -> Result<TackyAST> {
    let tacky_ast = gen_tacky_program(ast)?;

    if print_tacky {
        tacky_printer::print_tacky_ast(&tacky_ast);
    }

    Ok(tacky_ast)
}

fn gen_tacky_program(ast: AST) -> Result<TackyAST> {
    match ast.program {
        Program::FunctionDefinition(FunctionDefinition { name, body}) => {
            let program = gen_tacky_function(name, body)?;
            Ok(TackyAST { program })
        }
    }
}

fn gen_tacky_function(name: String, body: ast::Block) -> Result<TackyProgram> {
    let mut instrs = Vec::new();
    for item in body.items {
        match item {
            BlockItem::Decl(decl) => {
                generate_decl_instrs(decl, &mut instrs)?;
            },
            BlockItem::Stmt(stmt) => {
                generate_stmt_instrs(stmt, &mut instrs)?;
            }
        }
    }

    // Push a dummy return instruction in case the function doesn't have a return statement
    instrs.push(Instr::Return(Val::Integer(0)));

    Ok(TackyProgram::Function(name, instrs))
}

fn generate_decl_instrs(decl: ast::Decl, instrs: &mut Vec<Instr>) -> Result<()> {
    match decl {
        ast::Decl::Decl(name, Some(expr), _) => {
            let val = gen_expr_instrs(expr, instrs)?;
            instrs.push(Instr::Copy(val, Val::Var(name)));
        },
        _ => ()
    }
    Ok(())
}

fn generate_stmt_instrs(stmt: ast::Stmt, instrs: &mut Vec<Instr>) -> Result<()> {
    match stmt {
        Stmt::Return(expr) => {
            gen_return_instrs(expr, instrs)?
        },
        Stmt::Expression(expr) => {
            let _ = gen_expr_instrs(expr, instrs)?;
        },
        Stmt::Null => (),
        Stmt::If(condition, then_stmt, else_stmt) => {
            let end_label = name_generator::gen_label("end");
            let else_label = name_generator::gen_label("else");
            let condition: Val = gen_expr_instrs(condition, instrs)?;
            instrs.push(Instr::JumpIfZero(condition, else_label.clone()));
            generate_stmt_instrs(*then_stmt, instrs)?;
            instrs.push(Instr::Jump(end_label.clone()));
            instrs.push(Instr::Label(else_label));
            if let Some(s) = else_stmt {
                generate_stmt_instrs(*s, instrs)?;
            }
            instrs.push(Instr::Label(end_label))
        },
        Stmt::Compound(block) => {
            for item in block.items {
                match item {
                    BlockItem::Decl(decl) => {
                        generate_decl_instrs(decl, instrs)?;
                    },
                    BlockItem::Stmt(stmt) => {
                        generate_stmt_instrs(stmt, instrs)?;
                    }
                }
            }
        },
        Stmt::Break(label) => {
            instrs.push(Instr::Jump("break_".to_string() + &label));
        },
        Stmt::Continue(label) => {
            instrs.push(Instr::Jump("continue_".to_string() + &label));
        },
        Stmt::DoWhile(body, condition, label) => {
            let start_label = gen_label("start");
            instrs.push(Instr::Label(start_label.clone()));
            generate_stmt_instrs(*body, instrs)?;
            instrs.push(Instr::Label("continue_".to_string() + &label));
            let condition = gen_expr_instrs(condition, instrs)?;
            instrs.push(Instr::JumpIfNotZero(condition, start_label));
            instrs.push(Instr::Label("break_".to_string() + &label));
        },
        Stmt::While(condition, body, label) => {
            let continue_label = "continue_".to_string() + &label;
            let break_label = "break_".to_string() + &label;
            instrs.push(Instr::Label(continue_label.clone()));
            let condition = gen_expr_instrs(condition, instrs)?;
            instrs.push(Instr::JumpIfZero(condition, break_label.clone()));
            generate_stmt_instrs(*body, instrs)?;
            instrs.push(Instr::Jump(continue_label));
            instrs.push(Instr::Label(break_label));
        },
        Stmt::For(init, condition, post, body, label) => {
            let start_label = gen_label("start");
            let continue_label = "continue_".to_string() + &label;
            let break_label = "break_".to_string() + &label;
            if let Some(init) = init {
                match init {
                    ForInit::Decl(d) => {
                        generate_decl_instrs(d, instrs)?;
                    },
                    ForInit::Expr(e) => {
                        gen_expr_instrs(e, instrs)?;
                    }
                }
            }
            instrs.push(Instr::Label(start_label.clone()));
            if let Some(condition) = condition {
                let condition = gen_expr_instrs(condition, instrs)?;
                instrs.push(Instr::JumpIfZero(condition, break_label.clone()));
            }
            generate_stmt_instrs(*body, instrs)?;
            instrs.push(Instr::Label(continue_label));
            if let Some(post) = post {
                gen_expr_instrs(post, instrs)?;
            }
            instrs.push(Instr::Jump(start_label));
            instrs.push(Instr::Label(break_label));
        }
    };
    Ok(())
}

fn gen_return_instrs(expr: Expr, instrs: &mut Vec<Instr>) -> Result<()> {
    let val = gen_expr_instrs(expr, instrs)?;
    instrs.push(Instr::Return(val));
    Ok(())
}

fn gen_expr_instrs(expr: Expr, instrs: &mut Vec<Instr>) -> Result<Val> {
    match expr {
        Expr::Integer(i) => {
            Ok(Val::Integer(i))
        },
        Expr::UnaryOp(operator, expr) => {
            let src = gen_expr_instrs(*expr, instrs)?;
            let dest = Val::Var(name_generator::gen_tmp_name());
            let unary_op = match operator {
                ast::UnaryOp::Negate => UnaryOp::Negate,
                ast::UnaryOp::Complement => UnaryOp::Complement,
                ast::UnaryOp::Not => UnaryOp::Not,
            };
            instrs.push(Instr::Unary(unary_op, src, dest.clone()));
            Ok(dest)
        },
        Expr::BinaryOp(ast::BinaryOp::LogicalAnd, left, right) => {
            gen_logical_and(left, right, instrs)
        },
        Expr::BinaryOp(ast::BinaryOp::LogicalOr, left, right) => {
            gen_logical_or(left, right, instrs)
        },
        Expr::BinaryOp(operator, left, right) => {
            let left = gen_expr_instrs(*left, instrs)?;
            let right = gen_expr_instrs(*right, instrs)?;
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
            instrs.push(Instr::Binary(binary_op, left, right, dest.clone()));
            Ok(dest)
        },
        Expr::Var(name, _) => {
            Ok(Val::Var(name.clone()))
        },
        Expr::Assignment(left, right, _) => {
            match *left {
                Expr::Var(name, _) => {
                    let right = gen_expr_instrs(*right, instrs)?;
                    instrs.push(Instr::Copy(right, Val::Var(name.clone())));
                    Ok(Val::Var(name.clone()))
                },
                _ => {
                    unreachable!();
                }
            }
        },
        Expr::Conditional(condition, middle, right) => {
            let e2_label = name_generator::gen_label("e2");
            let end_label = name_generator::gen_label("end");
            let dest = Val::Var(name_generator::gen_tmp_name());

            let condition = gen_expr_instrs(*condition, instrs)?;
            instrs.push(Instr::JumpIfZero(condition, e2_label.clone()));
            let middle = gen_expr_instrs(*middle, instrs)?;
            instrs.push(Instr::Copy(middle, dest.clone()));
            instrs.push(Instr::Jump(end_label.clone()));
            instrs.push(Instr::Label(e2_label));
            let right = gen_expr_instrs(*right, instrs)?;
            instrs.push(Instr::Copy(right, dest.clone()));
            instrs.push(Instr::Label(end_label));
            Ok(dest)
        }

    }
}

fn gen_logical_and(left: Box<Expr>, right: Box<Expr>, instrs: &mut Vec<Instr>) -> Result<Val> {
    let left = gen_expr_instrs(*left, instrs)?;
    let false_label = name_generator::gen_label("false");
    let end_label = name_generator::gen_label("end");
    instrs.push(Instr::JumpIfZero(left, false_label.clone()));
    let right = gen_expr_instrs(*right, instrs)?;
    instrs.push(Instr::JumpIfZero(right, false_label.clone()));
    let dest = Val::Var(name_generator::gen_tmp_name());
    instrs.push(Instr::Copy(Val::Integer(1), dest.clone()));
    instrs.push(Instr::Jump(end_label.clone()));
    instrs.push(Instr::Label(false_label));
    instrs.push(Instr::Copy(Val::Integer(0), dest.clone()));
    instrs.push(Instr::Label(end_label));
    Ok(dest)
}

fn gen_logical_or(left: Box<Expr>, right: Box<Expr>, instrs: &mut Vec<Instr>) -> Result<Val> {
    let left = gen_expr_instrs(*left, instrs)?;
    let true_label = name_generator::gen_label("true");
    let end_label = name_generator::gen_label("end");
    instrs.push(Instr::JumpIfNotZero(left, true_label.clone()));
    let right = gen_expr_instrs(*right, instrs)?;
    instrs.push(Instr::JumpIfNotZero(right, true_label.clone()));
    let dest = Val::Var(name_generator::gen_tmp_name());
    instrs.push(Instr::Copy(Val::Integer(0), dest.clone()));
    instrs.push(Instr::Jump(end_label.clone()));
    instrs.push(Instr::Label(true_label.clone()));
    instrs.push(Instr::Copy(Val::Integer(1), dest.clone()));
    instrs.push(Instr::Label(end_label));
    Ok(dest)
}

