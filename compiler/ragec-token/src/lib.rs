mod term;

use crate::term::TermKind;

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

    pub fn new_blank_whitespace(length: usize) -> Self {
        Token::new(TokenKind::Whitespace(WhitespaceKind::Blank), length)
    }
    
    pub fn new_newline_whitespace() -> Self {
        Token::new(TokenKind::Whitespace(WhitespaceKind::NewLine), 1)
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
        Token::new(TokenKind::Term(TermKind::match_term(str)), str.len())
    }

    pub fn new_symbol(kind: SymbolKind, len: usize) -> Self {
        Token::new(TokenKind::Symbol(kind), len)
    }

    pub fn new_match_symbol(chars: &[char]) -> Self {
        match SymbolKind::match_symbol(chars) {
            Some(k) => return Token::new(TokenKind::Symbol(k), chars.len()),
            None => return Token::new(TokenKind::UNKNOWN, chars.len())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// any type of whitespace
    Whitespace(WhitespaceKind),
    /// any type of comment
    Comment(CommentKind),
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

#[derive(Debug, PartialEq, Eq)]
pub enum WhitespaceKind {
    Blank,
    NewLine,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommentKind {
    Line,
    Block,
    // Document,
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
    /// !
    Exclamation,
    /// "
    Quotation,
    /// #
    Number,
    /// $
    Dollar,
    /// %
    Percent,
    /// &
    Ampersand,
    /// '
    Apostrophe,
    /// (
    LParen,
    /// )
    RParen,
    /// *
    Asterisk,
    /// +
    Plus,
    /// ,
    Comma,
    /// -
    Hyphen,
    /// .
    Dot,
    /// /
    Slash,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// <
    Lesser,
    /// =
    Equal,
    /// >
    Greater,
    /// ?
    Question,
    /// @
    At,
    /// [
    LSquare,
    /// \
    Backslash,
    /// ]
    RSquare,
    /// ^
    Caret,
    /// _
    Underscore,
    /// `
    Accent,
    /// {
    LCurly,
    /// |
    Pipe,
    /// }
    RCurly,
    /// ~
    Tilde,

    /// &&
    And,
    /// ||
    Or,
    /// ==
    Equivalent,
    /// !=
    NotEquivalent,
    /// <<
    LeftShift,
    /// <<<
    LeftRotate,
    /// >>
    RightShift,
    /// >>>
    RightRotate,
    /// +=
    Incriment,
    /// -=
    Decriment,
    /// ..
    ExclusiveRange,
    /// ..=
    InclusiveRange,
    

    UNKNOWN,
}

impl SymbolKind {
    pub fn match_symbol(chars: &[char]) -> Option<Self> {
        let k = match chars {
            ['!'] => SymbolKind::Exclamation,
            ['"'] => SymbolKind::Quotation,
            ['#'] => SymbolKind::Number,
            ['$'] => SymbolKind::Dollar,
            ['%'] => SymbolKind::Percent,
            ['&'] => SymbolKind::Ampersand,
            ['\''] => SymbolKind::Apostrophe,
            ['('] => SymbolKind::LParen,
            [')'] => SymbolKind::RParen,
            ['*'] => SymbolKind::Asterisk,
            ['+'] => SymbolKind::Plus,
            [','] => SymbolKind::Comma,
            ['-'] => SymbolKind::Hyphen,
            ['.'] => SymbolKind::Dot,
            ['/'] => SymbolKind::Slash,
            [':'] => SymbolKind::Colon,
            [';'] => SymbolKind::Semicolon,
            ['<'] => SymbolKind::Lesser,
            ['='] => SymbolKind::Equal,
            ['>'] => SymbolKind::Greater,
            ['?'] => SymbolKind::Question,
            ['@'] => SymbolKind::At,
            ['['] => SymbolKind::LSquare,
            ['\\'] => SymbolKind::Backslash,
            [']'] => SymbolKind::RSquare,
            ['^'] => SymbolKind::Caret,
            ['_'] => SymbolKind::Underscore,
            ['`'] => SymbolKind::Accent,
            ['{'] => SymbolKind::LCurly,
            ['|'] => SymbolKind::Pipe,
            ['}'] => SymbolKind::RCurly,
            ['~'] => SymbolKind::Tilde,
            ['!', '='] => SymbolKind::NotEquivalent, 
            ['&', '&'] => SymbolKind::And,
            ['+', '='] => SymbolKind::Incriment,
            ['-', '='] => SymbolKind::Decriment,
            ['.', '.'] => Self::ExclusiveRange,
            ['.', '.', '.'] => Self::InclusiveRange,
            ['<', '<'] => SymbolKind::LeftShift,
            ['<', '<', '<'] => SymbolKind::LeftRotate,
            ['>', '>'] => Self::RightShift,
            ['>', '>', '>'] => Self::RightRotate,
            ['=', '='] => SymbolKind::Equivalent,
            ['|', '|'] => SymbolKind::Or,
            _ => SymbolKind::UNKNOWN, 
        };
        if k == SymbolKind::UNKNOWN { return None; }
        Some(k)
    }
}

