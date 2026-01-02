mod gcc;

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
    preprocess(&source)
}

pub fn preprocess(source: &Path) -> Result<()> {
    gcc::preprocess(source)
}