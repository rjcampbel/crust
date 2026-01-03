mod gcc;
mod lexer;
mod parser;

use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use lexer::token::Token;

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
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let source = args.source;
    let pp_source = source.with_extension("i");
    preprocess(&source, &pp_source)?;

    if args.lex {
        let _ = lex(&pp_source, args.print_tokens)?;
    }

    if args.parse {
        let tokens = lex(&pp_source, args.print_tokens)?;
        parse(&tokens)?
    }
    Ok(())
}

pub fn preprocess(source: &Path, dest: &Path) -> Result<()> {
    gcc::preprocess(source, dest)
}

pub fn lex(source: &Path, print_tokens: bool) -> Result<Vec<Token>> {
    let tokens = lexer::lex(&source, print_tokens)?;
    Ok(tokens)
}

pub fn parse(tokens: &Vec<Token>) -> Result<()> {
    parser::parse(&tokens)
}