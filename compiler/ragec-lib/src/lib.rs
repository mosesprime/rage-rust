use std::{path::PathBuf, error::Error};

use ragec_lexer;

/// Compiler settings.
pub struct Compiler {
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Compiler {
    pub fn new(input: PathBuf, output: PathBuf) -> Self {
        Self { input, output }
    }

    pub fn run(&self) -> Result<(), impl Error> {
        ragec_lexer::add(left, right)
        Ok(())
    }
}
