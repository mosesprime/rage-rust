use crate::{LexicalError, LexicalToken, LexicalErrorKind, LexicalTokenKind};

/// Lexical analysis manager.
pub struct Analyzer<'a> {
    lexemes: &'a Vec<LexicalToken>,
    error_hits: Vec<LexicalError>,
}


impl <'a> Analyzer<'a> {
    ///
    pub fn new(lexemes: &'a Vec<LexicalToken>) -> Self {
        Self {
            lexemes,
            error_hits: Default::default() }
    }

    ///
    pub fn run(mut self) -> Vec<LexicalError> {
        let mut cursor = 0;
        for lexeme in self.lexemes {
            match lexeme.kind {
                LexicalTokenKind::UNKNOWN => self.error_hits.push(LexicalError { loc: cursor, kind: LexicalErrorKind::UnknownToken }),
                _ => {},
            }
            cursor += 1;
        }
        self.error_hits
    }
}
