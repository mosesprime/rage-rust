use std::str::Chars;

const EOF_CHAR: char = '\0';

/// Lexical analyzer.
pub struct Lexer {
}

impl Lexer {
    pub fn new() -> Self {
        Self { }
    }

    // WIP
    pub fn run(&self, input: &str) -> Vec<LexicalToken> {
        let iter = tokenize(input);
        Vec::from_iter(iter)
    }
}

/// Produce lexical token iterator.
fn tokenize(input: &str) -> impl Iterator<Item = LexicalToken> + '_ {
    let mut tokenizer = Tokenizer::new(input.chars());
    std::iter::from_fn(move || {
        let token = tokenizer.next();
        if token.kind != LexicalTokenKind::EOF { Some(token) } else { None }
    })
}    

struct Tokenizer<'a> {
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
                    return LexicalToken::new(LexicalTokenKind::EOF, 0);
                } else {
                    return LexicalToken::new(LexicalTokenKind::UNKNOWN, 0);
                }
            },
        };
        let token = match first {
            EOF_CHAR => {
                if self.is_eof() {
                    return LexicalToken::new(LexicalTokenKind::EOF, 0);
                } else {
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
}

#[derive(Clone, Debug)]
pub struct LexicalToken {
    /// the kind of token
    pub kind: LexicalTokenKind,
    /// number of chars in token
    pub length: u32,
}

impl LexicalToken {
    pub fn new(kind: LexicalTokenKind, length: u32) -> Self {
        Self {
            kind,
            length,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
