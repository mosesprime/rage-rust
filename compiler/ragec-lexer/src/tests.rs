use crate::{Lexer, LexicalToken, LexicalTokenKind};

#[test]
fn test_lex_string() {
    let input = "let x = \"hi mom\"";
    let lexer = Lexer::new(input);
    let output: Vec<LexicalToken> = lexer.tokenize().collect();
    let mut expect = vec![
        LexicalToken {kind: LexicalTokenKind::Term, length: 3},
        LexicalToken {kind: LexicalTokenKind::Whitespace, length: 1},
        LexicalToken {kind: LexicalTokenKind::Symbol, length: 1},
        LexicalToken {kind: LexicalTokenKind::Whitespace, length: 1},
        LexicalToken {kind: LexicalTokenKind::Literal, length: 8},
    ];
    assert_eq!(output, expect);
}
