#[derive(Debug)]
pub enum ARAMSError {
    ParserError(ParserError),
    RuntimeError(RuntimeError),
}

impl std::fmt::Display for ARAMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ARAMSError::ParserError(e) => write!(f, "Parser error: {}", e),
            ARAMSError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
        }
    }
}

impl std::error::Error for ARAMSError {}

#[derive(Debug)]
pub enum ParserError {
    AnyError(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::AnyError(s) => write!(f, "Error: {}", s),
        }
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug)]
pub enum RuntimeError {
    AnyError(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::AnyError(s) => write!(f, "Error: {}", s),
        }
    }
}

impl std::error::Error for RuntimeError {}
