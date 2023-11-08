use std::str::Chars;

use crate::{LexicalToken, LexicalTokenKind, EOF_CHAR};

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl <'a>Tokenizer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self { chars }
    }

    pub fn next(&mut self) -> LexicalToken {
        let first = match self.consume() {
            Some(c) => c,
            None => {
                if self.is_eof() {
                    // no token but is end of file
                    return LexicalToken::new(LexicalTokenKind::EOF, 0);
                } else {
                    // no token yet is not the end of file
                    return LexicalToken::new(LexicalTokenKind::UNKNOWN, 0);
                }
            },
        };
        let token = match first {
            // whitespace
            c if is_whitespace(c) => self.whitespace(),

            // slash
            '/' => match self.peek_first() {
                // inline comment
                '/' => self.line_comment(),
                // slash symbol
                _ => self.symbol(),
            }

            '"' => self.string_literal(),

            c if c.is_ascii_digit() => match self.peek_first() {
                'x'|'X' => self.hex_literal(),
                'b'|'B' => self.binary_literal(),
                c2 if c2.is_ascii_digit() => self.numeric_literal(),
                _ => return LexicalToken::new(LexicalTokenKind::UNKNOWN, 1),
            },

            c if is_term_start(c) => self.term(),

            c if c.is_ascii_punctuation() => self.symbol(),
            
            '\'' => self.char_literal(),

            EOF_CHAR => {
                if self.is_eof() {
                    // end of file
                    return LexicalToken::new(LexicalTokenKind::EOF, 0);
                } else {
                    // end of file token, but is not the end of the file
                    return LexicalToken::new(LexicalTokenKind::UNKNOWN, 1);
                }
            },
            _ => LexicalToken::new(LexicalTokenKind::UNKNOWN, 1), // WIP
        };
        return token;
    }
    
    ///
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    ///
    fn peek_first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    ///
    fn peek_second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// wip
    fn consume_while(&mut self, mut predicate: impl FnMut(char)->bool) -> usize {
        let mut len = 0;
        while predicate(self.peek_first()) && !self.is_eof() {
            len += 1;
            self.consume().unwrap();
        }
        return len;
    }

    fn whitespace(&mut self) -> LexicalToken {
        let len = self.consume_while(is_whitespace);
        LexicalToken::new(LexicalTokenKind::Whitespace, len + 1)
    }

    fn line_comment(&mut self) -> LexicalToken {
        let len = self.consume_while(|c| c != '\n');
        LexicalToken::new(LexicalTokenKind::Comment, len + 1) // add the length consumed plus the '/' already consumed
    }

    fn symbol(&mut self) -> LexicalToken {
        LexicalToken::new(LexicalTokenKind::Symbol, 1)
    }

    fn string_literal(&mut self) -> LexicalToken { 
        let tok = LexicalToken::new(LexicalTokenKind::Literal, self.consume_while(|c| c != '"') + 2); // add the opening and closing quotes to the length
        let _= self.consume(); //consume the closing quote
        return tok;
    }

    fn numeric_literal(&mut self) -> LexicalToken { 
        LexicalToken::new(LexicalTokenKind::Literal, self.consume_while(|c| c.is_numeric()) + 1)
    }

    fn hex_literal(&mut self) -> LexicalToken {
        LexicalToken::new(LexicalTokenKind::Literal, self.consume_while(|c| c.is_numeric() || c == 'x' || c == 'X') + 1)
    }

    fn binary_literal(&mut self) -> LexicalToken {
        LexicalToken::new(LexicalTokenKind::Literal, self.consume_while(|c| c.is_numeric() || c == 'b' || c == 'B') + 1)
    }

    fn char_literal(&mut self) -> LexicalToken {
        LexicalToken::new(LexicalTokenKind::Literal, self.consume_while(|c| c != '\'') + 1)
    }

    fn term(&mut self) -> LexicalToken { 
        LexicalToken::new(
            LexicalTokenKind::Term, 
            self.consume_while(|c| is_term_continue(c)) + 1
        )
    }
}

/// is char a valid whiespace
fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// is char a valid start to a term
fn is_term_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// is char a valid continuation to a term
fn is_term_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// is a subslice a valid term
fn is_term(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(first) = chars.next() {
        is_term_start(first) && chars.all(is_term_continue)
    } else {
        false
    }
}

/// is char a valid symbol except double quotes
fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '"'
}
