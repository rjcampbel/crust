mod preprocessor;

use anyhow::Result;

fn main() -> Result<()> {
    preprocess("/Users/ryancampbell/git/crust/src/test/hello_world.c")
}

pub fn preprocess(filename: &str) -> Result<()> {
    preprocessor::preprocess(filename)
}