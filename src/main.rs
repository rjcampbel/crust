mod codegen;
mod compiler;
mod emitter;
mod error;
mod gcc;
mod lexer;
mod name_generator;
mod parser;
mod tacky;
mod validator;

use anyhow::Result;
use compiler::Compiler;
use clap::Parser;
use std::path::PathBuf;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to C source file to compile. Running without any additional arguments,
    /// or with only print_* arguments, will run all stages of the compiler and
    /// generate an executable in the same directory as the source file.
    source: PathBuf,

    /// Compile as library (generate .o file instead of executable)
    #[arg(short)]
    c: bool,

    /// Run only the lexer
    #[arg(long)]
    lex: bool,

    /// Run lexer and parser
    #[arg(long)]
    parse: bool,

    /// Run lexer, parser, and validator
    #[arg(long)]
    validate: bool,

    /// Run lexer, parser, validator, and IR generation
    #[arg(long)]
    tacky: bool,

    /// Run lexer, parser, validator, IR generation, and assembly generation
    #[arg(long)]
    codegen: bool,

    /// Print all the scanned tokens
    #[arg(long)]
    print_tokens: bool,

    /// Print the AST after parsing and validating
    #[arg(long)]
    print_ast: bool,

    /// Print the IR
    #[arg(long)]
    print_tacky: bool,

    /// Print the assembly
    #[arg(long)]
    print_assembly: bool,

    /// Additional arguments to pass to the assembler
    #[arg(long, allow_hyphen_values = true, num_args = 0..)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let mut args = Cli::parse();
    let mut compiler = Compiler::new(args.source)?;

    if args.lex {
        compiler.lex(args.print_tokens)?;
        return Ok(());
    }

    if args.parse {
        compiler.parse(args.print_tokens, args.print_ast)?;
        return Ok(());
    }

    if args.validate {
        compiler.validate(args.print_tokens, args.print_ast)?;
        return Ok(());
    }

    if args.tacky {
        compiler.tacky(args.print_tokens, args.print_ast, args.print_tacky)?;
        return Ok(());
    }

    if args.codegen {
        compiler.codegen(args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly)?;
        return Ok(());
    }

    compiler.compile(args.print_tokens, args.print_ast, args.print_tacky, args.print_assembly, args.c, &mut args.args)?;

    Ok(())
}
