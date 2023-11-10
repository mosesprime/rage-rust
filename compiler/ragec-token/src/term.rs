/// Term. Can be an Identifier or a keyword.
#[derive(Debug, Eq, PartialEq)]
pub enum TermKind {
    Identifier,

    Return,
    Mut,
    Dyn,
    Impl,
    Match,
}

impl TermKind {
    /// Returns the [`TermKind`] of a given [`str`].
    pub fn match_term(str: &str) -> TermKind {
        match str {
            "return" => TermKind::Return,
            "mut" => TermKind::Mut,
            "dyn" => TermKind::Dyn,
            "impl" => TermKind::Impl,
            "match" => TermKind::Match,
            _ => TermKind::Identifier,
        }
    }
}
