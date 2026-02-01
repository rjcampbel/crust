mod codegen;
mod emitter;
mod gcc;
mod lexer;
mod parser;
mod tacky;
mod validator;
mod error;
mod name_generator;
mod compiler;

use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

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

    /// Run only the lexer, parser, and validator
    #[arg(long, short)]
    validate: bool,

    /// Run only the lexer, parser, validator, and assembly generation
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
        let mut compiler = compiler::Compiler::new();
        compiler.lex(&pp_source, args.print_tokens)?;
        return Ok(());
    }

    if args.parse {
        let mut compiler = compiler::Compiler::new();
        compiler.parse(&pp_source, args.print_tokens, args.print_ast)?;
        return Ok(());
    }

    if args.validate {
        let mut compiler = compiler::Compiler::new();
        compiler.validate(&pp_source, args.print_tokens, args.print_ast)?;
        return Ok(());
    }

    if args.tacky {
        let mut compiler = compiler::Compiler::new();
        compiler.tacky(&pp_source, args.print_tokens, args.print_ast, args.print_tacky)?;
        return Ok(());
    }

    if args.codegen {
        let mut compiler = compiler::Compiler::new();
        compiler.codegen(&pp_source, args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly)?;
        return Ok(());
    }

    build(&pp_source, args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly)?;

    Ok(())
}

fn preprocess(source: &Path, dest: &Path) -> Result<()> {
    gcc::preprocess(source, dest)
}

fn build(source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<()> {
    let mut compiler = compiler::Compiler::new();
    compiler.compile(&source, print_tokens, print_ast, print_tacky, print_assembly)?;
    fs::remove_file(&source)?;
    let s_source = source.with_extension("s");
    fs::remove_file(&s_source)?;
    Ok(())
}