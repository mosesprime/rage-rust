use std::{path::PathBuf, fs, io};

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

    pub fn run(&self) -> io::Result<()> {
        let input = fs::read_to_string(&self.input)?.to_owned();
        let lexer = ragec_lexer::Lexer::new(input);
        let tokens: Vec<ragec_lexer::LexicalToken> = lexer.tokenize().collect();
        println!("{tokens:?}");
        Ok(())
    }
}
