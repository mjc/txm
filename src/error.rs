#[derive(Debug, Clone, Default, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ParseError(pub String);
