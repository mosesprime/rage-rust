mod analyzer;
//mod sanitizer;
mod tokenizer;

use crate::analyzer::Analyzer;
use crate::tokenizer::Tokenizer;

const EOF_CHAR: char = '\0';

/// Lexical analyzer.
pub struct Lexer {
    input: String,
    lexemes: Vec<LexicalToken>,
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
    pub fn lexemes(&self) -> &Vec<LexicalToken> {
        &self.lexemes
    }

    /// Runs the [`Tokenizer`].
    pub fn tokenize(&mut self) {
        let mut tokenizer = Tokenizer::new(self.input.chars());
        self.lexemes = std::iter::from_fn(move || {
            let token = tokenizer.next();
            if token.kind != LexicalTokenKind::EOF { Some(token) } else { None }
        }).collect();
    }

    /// Runs all the [`Analyzer`] passes. Returns a list of [`LexicalError`]s.
    pub fn analyze(&self) -> Vec<LexicalError> {
        Analyzer::new(&self.lexemes).run()
    }
}



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexicalToken {
    /// the kind of token
    pub kind: LexicalTokenKind,
    /// number of chars in token
    pub length: usize,
}

impl LexicalToken {
    pub fn new(kind: LexicalTokenKind, length: usize) -> Self {
        Self {
            kind,
            length,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexicalTokenKind {
    /// any type of whitespace
    Whitespace,
    /// any type of comment
    Comment,
    /// any type of literal
    Literal,
    /// Delimiter or operator.
    Symbol,
    /// Identifier or keyword.
    Term,

    /// End of file.
    EOF,
    ///
    UNKNOWN,
}

#[derive(Debug)]
pub struct LexicalError {
    pub loc: usize,
    pub kind: LexicalErrorKind,
}

#[derive(Debug)]
pub enum LexicalErrorKind {
    UnknownToken,
}

