use std::{path::PathBuf, fs, io};

use ragec_lexer;

/// Compilation manager.
pub struct Compiler {
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Compiler {
    /// Create new [`Compiler`] instance. Recommended to use only one instance.
    pub fn new(input: PathBuf, output: PathBuf) -> Self {
        Self { input, output }
    }

    /// Execute the compilation.
    pub fn run(&self) -> Result<(), Vec<CompilerError>> {
        let mut errors: Vec<CompilerError> = Default::default();
        let input = match fs::read_to_string(&self.input) {
            Ok(s) => s.to_owned(),
            Err(e) => return Err(vec![CompilerError::Io(e)]),
        };
        let mut lexer = ragec_lexer::Lexer::new(input);
        lexer.tokenize();
        let lexical_analysis_errors = lexer.analyze();
        let mut offset = 0;
        for token in lexer.lexemes() {
            let value = lexer.value(offset, token.length);
            println!("{value}:{token:?}");
            offset += token.length;
        }
        for e in lexical_analysis_errors {
            errors.push(CompilerError::Lexical(e));
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompilerError {
    Io(io::Error),
    Lexical(ragec_lexer::LexicalError),
}
