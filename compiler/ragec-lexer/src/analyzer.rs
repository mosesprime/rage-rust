use ragec_token::{Token, TokenKind};

use crate::{LexicalError, LexicalErrorKind};

/// Lexical analysis manager.
pub struct Analyzer {
    error_hits: Vec<LexicalError>,
}


impl Analyzer {
    /// New instance of [`Analyzer`].
    pub fn new() -> Self {
        Self {
            error_hits: Default::default()
        }
    }

    /// Executes the [`Analyzer`] passes on the given [`Vec<Token>`].
    /// Returns a result containing [`Ok(())`] or [`Err(Vec<LexicalError>)`].
    pub fn run(mut self, mut lexemes: &Vec<Token>) -> Result<(), Vec<LexicalError>> {
        let mut offset = 0;
        for lexeme in lexemes {
            match lexeme.kind {
                TokenKind::UNKNOWN => self.error_hits.push(LexicalError::new(offset, lexeme.length, LexicalErrorKind::UnknownToken)),
                _ => {},
            }
            offset += lexeme.length;
        }
        if self.error_hits.len() > 0 {
            return Err(self.error_hits);
        }
        Ok(())
    }
}
