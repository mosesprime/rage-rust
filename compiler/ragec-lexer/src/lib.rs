use std::{fmt::Display, str::Chars};

/// Lexical tokenizer and analyzer.
pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { chars: input.chars() }
    }

    /// Produce lexical tokens.
    pub fn tokenize(&self) -> Result<Vec<LexicalToken>, LexicalError> {
        todo!()
    }
    
    ///
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

#[derive(Debug)]
pub enum LexicalToken {
    ///
    Whitespace,
    ///
    Comment,
    ///
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

impl LexicalToken {
    //
}

#[derive(Debug)]
pub struct LexicalError {
    message: String,
    type: LexicalErrorType,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ message: {} type: {} }}", self.message, self.type)
    }
}

#[derive(Debug)]
pub enum LexicalErrorType {
    //
}
