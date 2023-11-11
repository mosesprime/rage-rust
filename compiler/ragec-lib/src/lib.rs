use std::{path::PathBuf, fs, io, str::Chars};

use ragec_lexer::{self, Lexer};
use ragec_token::{self, Token};

/// Compilation manager.
pub struct Compiler {
    path: PathBuf,
    input: String,
}

impl Compiler {
    /// Create new [`Compiler`] instance. Recommended to use only one instance.
    pub fn new(path: PathBuf) -> Result<Self, CompilerError> {
        match fs::read_to_string(&path) {
            Ok(input) => {
                return Ok(Self { path, input });
            },
            Err(e) => return Err(CompilerError::Io(e)),
        };
    }

    /// Execute the entire compilation.
    pub fn run(&self) -> Result<(), Vec<CompilerError>> {
        self.run_lexer()
    }

    /// Execute the compilation.
    pub fn run_lexer(&self) -> Result<(), Vec<CompilerError>> {
        let tokens: Vec<Token> = match Lexer::new().run(self.input.chars()) {
            Ok(t) => t,
            Err(err) => {
                let mut vec: Vec<CompilerError> = Default::default();
                for e in err {
                    vec.push(CompilerError::Lexical(e));
                }  
                return Err(vec);
            },
        };
        let mut offset = 0;
        for token in tokens {
            let value = self.value_from_input(offset, token.length);
            println!("{value}:{token:?}");
            offset += token.length;
        }
        Ok(())
    }

    pub fn value_from_input(&self, offset: usize, length: usize) -> String {
        self.input.chars().skip(offset).take(length).collect()
    }
}

#[derive(Debug)]
pub enum CompilerError {
    Io(io::Error),
    Lexical(ragec_lexer::LexicalError),
}
