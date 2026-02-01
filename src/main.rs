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
use std::path::PathBuf;

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
    let mut compiler = compiler::Compiler::new();

    if args.lex {
        compiler.lex(&source, args.print_tokens)?;
        return Ok(());
    }

    if args.parse {
        compiler.parse(&source, args.print_tokens, args.print_ast)?;
        return Ok(());
    }

    if args.validate {
        compiler.validate(&source, args.print_tokens, args.print_ast)?;
        return Ok(());
    }

    if args.tacky {
        compiler.tacky(&source, args.print_tokens, args.print_ast, args.print_tacky)?;
        return Ok(());
    }

    if args.codegen {
        compiler.codegen(&source, args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly)?;
        return Ok(());
    }

    compiler.compile(&source, args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly)?;

    Ok(())
}
