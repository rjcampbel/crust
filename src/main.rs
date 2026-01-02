mod gcc;
mod lexer;

use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};

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
    codegen: bool
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let source = args.source;
    let pp_source = source.with_extension("i");
    preprocess(&source, &pp_source)?;

    if args.lex {
        lex(&pp_source)?
    }

    Ok(())
}

pub fn preprocess(source: &Path, dest: &Path) -> Result<()> {
    gcc::preprocess(source, dest)
}

pub fn lex(source: &Path) -> Result<()> {
    lexer::lex(&source)?;
    Ok(())
}