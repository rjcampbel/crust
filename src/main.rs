mod codegen;
mod emitter;
mod gcc;
mod lexer;
mod parser;
mod tacky;

use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};

use codegen::assembly::AssemblyAST;
use lexer::token::Token;
use parser::ast::AST;
use tacky::tacky::TackyAST;

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

    /// Run only the lexer, parser, and tacky generation
    #[arg(long, short)]
    tacky: bool,

    /// Run only the lexer, parser, and assembly generation
    #[arg(long, short)]
    codegen: bool,

    /// Print all the scanned tokens
    #[arg(long)]
    print_tokens: bool,

    /// Print the AST after parsing
    #[arg(long)]
    print_ast: bool,

    /// Print the tacky ast
    #[arg(long)]
    print_tacky: bool,

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
        let _ = parse(tokens, args.print_ast)?;
        return Ok(());
    }

    if args.tacky {
        let tokens = lex(&pp_source, args.print_tokens)?;
        let ast = parse(tokens, args.print_ast)?;
        let _ = gen_tacky(&ast, args.print_tacky)?;
        return Ok(());
    }

    if args.codegen {
        let tokens = lex(&pp_source, args.print_tokens)?;
        let ast = parse(tokens, args.print_ast)?;
        let tacky = gen_tacky(&ast, args.print_tacky)?;
        let _ = codegen(&tacky, args.print_assembly)?;
        return Ok(());
    }

    build(&pp_source, args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly)?;

    Ok(())
}

fn preprocess(source: &Path, dest: &Path) -> Result<()> {
    gcc::preprocess(source, dest)
}

fn lex(source: &Path, print_tokens: bool) -> Result<Vec<Token>> {
    let tokens = lexer::lex(&source, print_tokens)?;
    Ok(tokens)
}

fn parse(tokens: Vec<Token>, print_ast: bool) -> Result<AST> {
    parser::parse(tokens, print_ast)
}

fn gen_tacky(ast: &AST, print_tacky: bool) -> Result<TackyAST> {
    tacky::gen_tacky(ast, print_tacky)
}

fn codegen(tacky: &TackyAST, print_assembly: bool) -> Result<AssemblyAST> {
    codegen::codegen(tacky, print_assembly)
}

fn build(source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<()> {
    let tokens = lex(&source, print_tokens)?;
    let ast = parse(tokens, print_ast)?;
    let tacky = gen_tacky(&ast, print_tacky)?;
    let assembly_ast = codegen(&tacky, print_assembly)?;
    let output = source.with_extension("s");
    emit_code(&assembly_ast, &output)?;
    assemble(&output, &source.with_extension(""))?;
    Ok(())
}

fn emit_code(assembly_ast: &AssemblyAST, output: &Path) -> Result<()> {
    emitter::emit_code(&assembly_ast, output)
}

fn assemble(source: &Path, output: &Path) -> Result<()> {
    gcc::assemble(source, output)
}