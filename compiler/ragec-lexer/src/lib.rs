mod analyzer;
//mod sanitizer;
mod tokenizer;

use ragec_token::{Token, TokenKind};

use crate::analyzer::Analyzer;
use crate::tokenizer::Tokenizer;

const EOF_CHAR: char = '\0';

/// Lexical analyzer.
pub struct Lexer {
    input: String,
    lexemes: Vec<Token>,
    //output: Vec<Tokens>,
}

impl Lexer {
    /// New instance of [`Lexer`].
    pub fn new(input: impl ToString) -> Self {
        Self { 
            input: input.to_string(),
            lexemes: Default::default(),
        }
    }
    
    /// Get the value at the given location.
    pub fn value(&self, offset: usize, len: usize) -> String {
        self.input.chars().skip(offset).take(len).collect()
    }

    ///
    pub fn lexemes(&self) -> &Vec<Token> {
        &self.lexemes
    }

    /// Runs the [`Tokenizer`].
    pub fn tokenize(&mut self) {
        let mut tokenizer = Tokenizer::new(self.input.chars());
        self.lexemes = std::iter::from_fn(move || {
            let token = tokenizer.next();
            if token.kind != TokenKind::EOF { Some(token) } else { None }
        }).collect();
    }

    /// Runs all the [`Analyzer`] passes. Returns a list of [`LexicalError`]s.
    pub fn analyze(&self) -> Vec<LexicalError> {
        Analyzer::new(&self.lexemes).run()
    }
}





#[derive(Debug)]
pub struct LexicalError {
    pub offset: usize,
    pub length: usize,
    pub kind: LexicalErrorKind,
}

impl LexicalError {
    pub fn new(offset: usize, length: usize, kind: LexicalErrorKind) -> Self {
        Self { offset, length, kind }
    }
}

#[derive(Debug)]
pub enum LexicalErrorKind {
    UnknownToken,
}

