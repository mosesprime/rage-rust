use ragec_token::{Token, TokenKind};

use crate::{LexicalError, LexicalErrorKind};

/// Lexical analysis manager.
pub struct Analyzer<'a> {
    lexemes: &'a Vec<Token>,
    error_hits: Vec<LexicalError>,
}


impl <'a> Analyzer<'a> {
    ///
    pub fn new(lexemes: &'a Vec<Token>) -> Self {
        Self {
            lexemes,
            error_hits: Default::default() }
    }

    ///
    pub fn run(mut self) -> Vec<LexicalError> {
        let mut offset = 0;
        for lexeme in self.lexemes {
            match lexeme.kind {
                TokenKind::UNKNOWN => self.error_hits.push(LexicalError::new(offset, lexeme.length, LexicalErrorKind::UnknownToken)),
                _ => {},
            }
            offset += lexeme.length;
        }
        self.error_hits
    }
}
