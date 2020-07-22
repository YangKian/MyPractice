#[derive(Debug, PartialEq)]
pub enum Result {
    LeptParseOk,
    LeptParseExpectValue,
    LeptParseInvalidValue,
    LeptpParseRootNotSingular,
    LeptParseNumberTooBig,
}

// pub type Result<T> = anyhow::Result<T, LeptError>;
