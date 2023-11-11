use std::str::Chars;

use ragec_token::{Token, TokenKind, SymbolKind, CommentKind};

use crate::EOF_CHAR;

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl <'a>Tokenizer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self { chars }
    }

    pub fn next(&mut self) -> Token {
        let first = match self.consume() {
            Some(c) => c,
            None => {
                if self.is_eof() {
                    // no token but is end of file
                    return Token::new(TokenKind::EOF, 0);
                } else {
                    // no token yet is not the end of file
                    return Token::new(TokenKind::UNKNOWN, 0);
                }
            },
        };
        return match first {
            // whitespace
            c if is_whitespace(c) => self.whitespace(),

            // slash
            '/' => match self.peek_first() {
                // inline comment
                '/' => match self.peek_second() {
                    '*' => self.block_comment(),
                    // '/' => self.document_comment(),
                    _ => self.line_comment(),
                },
                // slash symbol
                _ => return Token::new_symbol(SymbolKind::Slash),
            }

            // number
            c if c.is_ascii_digit() => match self.peek_first() {
                'x'|'X' => match self.peek_second() {
                    c if c.is_ascii_digit() => self.hex_literal(),
                    _ => return Token::new(TokenKind::UNKNOWN, 1),
                },
                'b'|'B' => match self.peek_second() {
                    c if c.is_ascii_digit() => self.binary_literal(),
                    _ => return Token::new(TokenKind::UNKNOWN, 1),
                },
                _ => self.numeric_literal(),
            },


            // string
            '"' => self.string_literal(),

            // alphabetic or _
            c if is_term_start(c) => self.term(c),

            // 'char'
            '\'' => self.char_literal(),

            // non-slash and non-char symbol
            c if c.is_ascii_punctuation() => return Token::new_match_symbol(c),
            

            EOF_CHAR => {
                if self.is_eof() {
                    // end of file
                    return Token::new(TokenKind::EOF, 0)
                } else {
                    // end of file token, but is not the end of the file
                    return Token::new(TokenKind::UNKNOWN, 1);
                }
            },
            _ => Token::new(TokenKind::UNKNOWN, 1), // WIP
        };
    }
    
    ///
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Peeks the first char. Does not consume.
    fn peek_first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the second char. Does not consume.
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

    fn whitespace(&mut self) -> Token {
        let len = self.consume_while(is_whitespace);
        Token::new(TokenKind::Whitespace, len + 1)
    }

    fn block_comment(&mut self) -> Token {
        let mut len = 1;
        let mut prev = '_';
        loop {
            if self.is_eof() { break; }
            len += 1;
            let new = self.consume().unwrap();
            if prev == '*' && new == '/' { break; }
            prev = new;
        }
        Token::new(TokenKind::Comment(CommentKind::Block), len)
    }

    fn line_comment(&mut self) -> Token {
        let len = self.consume_while(|c| c != '\n');
        Token::new(TokenKind::Comment(CommentKind::Line), len + 1) // add the length consumed plus the '/' already consumed
    }

    fn string_literal(&mut self) -> Token { 
        let tok = Token::new_string_literal(self.consume_while(|c| c != '"') + 2); // add the opening and closing quotes to the length
        let _= self.consume(); //consume the closing quote
        return tok;
    }

    fn numeric_literal(&mut self) -> Token { 
        Token::new_numeric_literal(self.consume_while(|c| c.is_numeric()) + 1)
    }

    fn hex_literal(&mut self) -> Token {
        Token::new_hex_literal(self.consume_while(|c| c.is_numeric() || c == 'x' || c == 'X') + 1)
    }

    fn binary_literal(&mut self) -> Token {
        Token::new_binary_literal(self.consume_while(|c| c.is_numeric() || c == 'b' || c == 'B') + 1)
    }

    fn char_literal(&mut self) -> Token {
        Token::new_char_literal(self.consume_while(|c| c != '\'') + 1)
    }

    fn term(&mut self, c: char) -> Token { 
        let chars = self.chars.as_str();
        let len = self.consume_while(|c| is_term_continue(c)) + 1;
        let s = chars.get(..(len - 1)).unwrap();
        Token::new_term(format!("{}{}", c, s).as_str())
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
    c.is_ascii_alphabetic() || c == '_'
}

/// is char a valid continuation to a term
fn is_term_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// is a subslice a valid term
fn is_term(str: &str) -> bool {
    let mut chars = str.chars();
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
