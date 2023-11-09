#[derive(Debug, Eq, PartialEq)]
pub struct Token {
     /// number of chars in token
    pub length: usize,
    /// the kind of token
    pub kind: TokenKind,

}

impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Self {
        Self {
            length,
            kind,
        }
    }

    pub fn new_whitespace(length: usize) -> Self {
        Token::new(TokenKind::Whitespace, length)
    }

    pub fn new_string_literal(length: usize) -> Self {
        Token::new(TokenKind::Literal(LiteralKind::String), length)
    }

    pub fn new_numeric_literal(length: usize) -> Self {
        Token::new(TokenKind::Literal(LiteralKind::Numeric), length)
    }

    pub fn new_binary_literal(length: usize) -> Self {
        Token::new(TokenKind::Literal(LiteralKind::Binary), length)
    }

    pub fn new_char_literal(length: usize) -> Self {
        Token::new(TokenKind::Literal(LiteralKind::Char), length)
    }

    pub fn new_hex_literal(length: usize) -> Self {
        Token::new(TokenKind::Literal(LiteralKind::Hex), length)
    }

    pub fn new_term(str: &str) -> Self {
        Token::new(TokenKind::Term(match_term(str)), str.len())
    }

    pub fn new_symbol(kind: SymbolKind) -> Self {
        Token::new(TokenKind::Symbol(kind), 1)
    }

    pub fn new_match_symbol(c: char) -> Self {
        Token::new_symbol(match_symbol(c))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// any type of whitespace
    Whitespace,
    /// any type of comment
    Comment,
    /// any type of literal
    Literal(LiteralKind),
    /// Delimiter or operator.
    Symbol(SymbolKind),
    /// Identifier or keyword.
    Term(TermKind),

    /// End of file.
    EOF,
    ///
    UNKNOWN,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TermKind {
    Identifier,

    Return,
    Mut,
    Dyn,
    Match,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LiteralKind {
    String,
    Numeric,
    Binary,
    Char,
    Hex,
}

// Token symbols. Complex symbols/operators are handled later.
#[derive(Debug, Eq, PartialEq)]
pub enum SymbolKind {
    Exclamation,
    Quotation,
    Number,
    Dollar,
    Percent,
    Ampersand,
    Apostrophe,
    LParen,
    RParen,
    Asterisk,
    Plus,
    Comma,
    Hyphen,
    Dot,
    Slash,
    Colon,
    Semicolon,
    Lesser,
    Equal,
    Greater,
    Question,
    At,
    LSquare,
    Backslash,
    RSquare,
    Caret,
    Underscore,
    Accent,
    LCurly,
    Pipe,
    RCurly,
    Tilde,

    UNKNOWN,
}

fn match_term(str: &str) -> TermKind {
    match str {
        "return" => TermKind::Return,
        "mut" => TermKind::Mut,
        "dyn" => TermKind::Dyn,
        "match" => TermKind::Match,
        _ => TermKind::Identifier,
    }
}

fn match_symbol(c: char) -> SymbolKind {
    match c {
        '!' => SymbolKind::Exclamation,
        '"' => SymbolKind::Quotation,
        '#' => SymbolKind::Number,
        '$' => SymbolKind::Dollar,
        '%' => SymbolKind::Percent,
        '&' => SymbolKind::Ampersand,
        '\'' => SymbolKind::Apostrophe,
        '(' => SymbolKind::LParen,
        ')' => SymbolKind::RParen,
        '*' => SymbolKind::Asterisk,
        '+' => SymbolKind::Plus,
        ',' => SymbolKind::Comma,
        '-' => SymbolKind::Hyphen,
        '.' => SymbolKind::Dot,
        '/' => SymbolKind::Slash,
        ':' => SymbolKind::Colon,
        ';' => SymbolKind::Semicolon,
        '<' => SymbolKind::Lesser,
        '=' => SymbolKind::Equal,
        '>' => SymbolKind::Greater,
        '?' => SymbolKind::Question,
        '@' => SymbolKind::At,
        '[' => SymbolKind::LSquare,
        '\\' => SymbolKind::Backslash,
        ']' => SymbolKind::RSquare,
        '^' => SymbolKind::Caret,
        '_' => SymbolKind::Underscore,
        '`' => SymbolKind::Accent,
        '{' => SymbolKind::LCurly,
        '|' => SymbolKind::Pipe,
        '}' => SymbolKind::RCurly,
        '~' => SymbolKind::Tilde,
        _ => SymbolKind::UNKNOWN, 
    }       
}
