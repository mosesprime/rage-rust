use std::{path::PathBuf, fs, io, str::Chars};

use ragec_lexer::{self, Lexer, LexicalError};
use ragec_parser::{Parser, ParsingError};
use ragec_token::{self, Token};

/// A single compilation unit.
pub struct Compiler {
    lexer: Lexer,
    parser: Parser,
}

impl Compiler {
    /// Create new [`Compiler`] instance. Recommended to use only one instance.
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
            parser: Parser::new(),
        }
    }

    /// Execute the entire compilation of a source path.
    pub fn run(&mut self, path: PathBuf) -> Result<(), Vec<CompilerError>> {
        let input = match fs::read_to_string(&path) {
            Ok(s) => s.to_owned(),
            Err(e) => return Err(vec![CompilerError::Io(e)]),
        };
        let tokens = self.run_lexer(input.chars())?;
        let mut offset = 0;
        for token in &tokens {
            let value = self.value_from_input(input.chars(), offset, token.length);
            println!("{value}:{token:?}");
            offset += token.length;
        }
        let p = self.run_parser(tokens)?;
        Ok(())
    }

    /// Execute the [`Lexer`].
    fn run_lexer(&self, input: Chars) -> Result<Vec<Token>, Vec<CompilerError>> {
        let tokens = match self.lexer.run(input) {
            Ok(t) => t,
            Err(err) => {
                let mut vec: Vec<CompilerError> = Default::default();
                for e in err {
                    vec.push(CompilerError::Lexical(e));
                }  
                return Err(vec);
            },
        };
        Ok(tokens)
    }

    /// Execute the [`Parser`]
    fn run_parser(&mut self, input: Vec<Token>) -> Result<(), Vec<CompilerError>> {
        let tree = self.parser.run(input);
        Ok(())
    }

    pub fn value_from_input(&self, input: Chars, offset: usize, length: usize) -> String {
        input.skip(offset).take(length).collect()
    }
}

#[derive(Debug)]
pub enum CompilerError {
    Io(io::Error),
    Lexical(LexicalError),
    Parsing(ParsingError),
}
