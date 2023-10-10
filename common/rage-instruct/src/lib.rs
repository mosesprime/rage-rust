
pub struct Instruction {
    pub op: Operation,
}

pub enum Operation {
    ///
    Add,
    ///
    Load,

    /// Bad operation.
    UNKNOWN,
}
