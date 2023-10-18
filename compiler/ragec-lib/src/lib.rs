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
        let tokens = lexer.tokenize();
        let mut offset = 0;
        for token in tokens {
            let value = lexer.value(offset, token.length);
            println!("{value}:{token:?}");
            offset += token.length;
        }
        Ok(())
    }
}
