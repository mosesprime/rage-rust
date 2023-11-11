mod analyzer;
mod tokenizer;

use std::str::Chars;

use ragec_token::{Token, TokenKind};

use crate::analyzer::Analyzer;
use crate::tokenizer::Tokenizer;

const EOF_CHAR: char = '\0';

/// Lexical analyzer.
pub struct Lexer {}

impl Lexer {
    /// New instance of [`Lexer`].
    pub fn new() -> Self {
        Self {}
    }
   
    /// Execute the [`Tokenizer`] and run all [`Analyzer`] passes.
    /// Returns a result of [`Vec<Token>`] or a [`Vec<LexicalError>`].
    pub fn run(&self, input: Chars) -> Result<Vec<Token>, Vec<LexicalError>> {
        let mut tokenizer = Tokenizer::new(input);
        let lexemes = std::iter::from_fn(move || {
            let token = tokenizer.next();
            if token.kind != TokenKind::EOF { Some(token) } else { None }
        }).collect();
        Analyzer::new().run(&lexemes)?;
        Ok(lexemes)

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

