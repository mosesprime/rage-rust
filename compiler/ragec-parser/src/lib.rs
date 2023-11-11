use ragec_ast::AstNodeId;
use ragec_token::{Token, TokenKind, WhitespaceKind};
use std::collections::BTreeMap;

pub struct Parser {
    symbol_table: BTreeMap<AstNodeId, Token>,
    _next_id: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self { 
            symbol_table: Default::default(),
            _next_id: 0,
        }
    }

    pub fn run(&mut self, mut input: Vec<Token>) -> Result<(), ParsingError> {
        input.retain(|t| t.kind != TokenKind::Whitespace(WhitespaceKind::Blank)); // remove blank whitespace, retain newline
        for token in input {
            let id = self.next_id();
            let _ = self.symbol_table.insert(id, token);
        }
        Ok(())
    }

    fn next_id(&mut self) -> usize {
        let n = self._next_id;
        self._next_id += 1;
        n
    }
}

#[derive(Debug)]
pub enum ParsingError {}
