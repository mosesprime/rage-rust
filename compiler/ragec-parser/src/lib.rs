use ragec_token::Token;
use std::collections::HashMap;

pub struct Parser {
    next_id: usize,
    table: HashMap<usize, ()>
}

impl Parser {
    pub fn new() -> Self {
        Self { 
            next_id: 0,
            table: Default::default(),
        }
    }

    pub fn run(&self) -> Result<(), ParsingError> {
        Ok(())
    }

    fn next_id(&mut self) -> usize {
        let n = self.next_id;
        self.next_id += 1;
        n
    }
}

#[derive(Debug)]
pub enum ParsingError {}
