#[derive(Debug, Clone)]
pub enum LSPError {
    UnknownToken(String),
    UnexpectedToken(String),
    MissingArgument,
    InvalidArgument(String),
    DuplicateLabelDefinition(String),
    LabelNotFound(String),
}

impl std::fmt::Display for LSPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LSPError::UnknownToken(token) => write!(f, "Unknown token '{}'", token),
            LSPError::UnexpectedToken(token) => write!(f, "Unexpected token '{}'", token),
            LSPError::MissingArgument => write!(f, "Expected an argument but none was found"),
            LSPError::InvalidArgument(arg) => {
                write!(f, "Argument '{}' is invalid or malformed", arg)
            }
            LSPError::DuplicateLabelDefinition(label) => {
                write!(f, "Label '{}' is defined more than once", label)
            }
            LSPError::LabelNotFound(label) => {
                write!(f, "Definition for label '{}' not found", label)
            }
        }
    }
}

impl std::error::Error for LSPError {}
