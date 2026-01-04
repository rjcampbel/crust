mod gcc;
mod lexer;
mod parser;
mod codegen;
mod emitter;

use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use lexer::token::{Token, TokenType};
use parser::ast::Program;
use codegen::assembly::Program as AssemblyProgram;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to C source file to compile
    source: PathBuf,

    /// Run only the lexer
    #[arg(long, short)]
    lex: bool,

    /// Run only the lexer and parser
    #[arg(long, short)]
    parse: bool,

    /// Run only the lexer, parser, and assembly generation
    #[arg(long, short)]
    codegen: bool,

    /// Print all the scanned tokens
    #[arg(long)]
    print_tokens: bool,

    /// Print the AST after parsing
    #[arg(long)]
    print_ast: bool,

    /// Print the assembly AST after code generation
    #[arg(long)]
    print_assembly: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let source = args.source;
    let pp_source = source.with_extension("i");
    preprocess(&source, &pp_source)?;

    if args.lex {
        let _ = lex(&pp_source, args.print_tokens)?;
        return Ok(());
    }

    if args.parse {
        let tokens = lex(&pp_source, args.print_tokens)?;
        let _ = parse(&tokens, args.print_ast)?;
        return Ok(());
    }

    if args.codegen {
        let tokens = lex(&pp_source, args.print_tokens)?;
        let program = parse(&tokens, args.print_ast)?;
        let _ = codegen(&program, args.print_assembly)?;
        return Ok(());
    }

    build(&pp_source, args.print_tokens, args.print_ast, args.print_assembly)?;

    Ok(())
}

fn preprocess(source: &Path, dest: &Path) -> Result<()> {
    gcc::preprocess(source, dest)
}

fn lex(source: &Path, print_tokens: bool) -> Result<Vec<Token>> {
    let tokens = lexer::lex(&source, print_tokens)?;
    Ok(tokens)
}

fn parse(tokens: &Vec<Token>, print_ast: bool) -> Result<Program> {
    parser::parse(&tokens, print_ast)
}

fn codegen(program: &Program, print_assembly: bool) -> Result<AssemblyProgram> {
    codegen::codegen(&program, print_assembly)
}

fn build(source: &Path, print_tokens: bool, print_ast: bool, print_assembly: bool) -> Result<()> {
    let tokens = lex(&source, print_tokens)?;
    let program = parse(&tokens, print_ast)?;
    let assembly_program = codegen(&program, print_assembly)?;
    let output = source.with_extension("s");
    emit_code(&assembly_program, &output)?;
    Ok(())
}

fn emit_code(program: &AssemblyProgram, output: &Path) -> Result<()> {
    emitter::emit_code(&program, output)
}