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

    fn next_id(&mut self) -> usize {
        let n = self.next_id;
        self.next_id += 1;
        n
    }
}
