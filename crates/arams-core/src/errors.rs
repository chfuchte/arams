#[derive(Debug)]
pub struct CompileError {
    line_number: usize,
    kind: CompileErrorKind,
    context: String,
}

impl CompileError {
    pub fn new(line_number: usize, kind: CompileErrorKind, context: String) -> Self {
        CompileError {
            line_number,
            kind,
            context,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn kind(&self) -> &CompileErrorKind {
        &self.kind
    }

    pub fn context(&self) -> &str {
        &self.context
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            CompileErrorKind::UnknownToken => {
                write!(
                    f,
                    "Unknown token '{}' at line {}",
                    self.context, self.line_number
                )
            }
            CompileErrorKind::ExpectedToken => {
                write!(f, "Expected {} at line {}", self.context, self.line_number)
            }
            CompileErrorKind::UnexpectedToken => {
                write!(
                    f,
                    "Unexpected token '{}' at line {}",
                    self.context, self.line_number
                )
            }
            CompileErrorKind::DuplicateLabelDefinition => {
                write!(
                    f,
                    "Duplicate label definition '{}' at line {}",
                    self.context, self.line_number
                )
            }
            CompileErrorKind::MissingArgument => {
                write!(
                    f,
                    "Missing argument for '{}' at line {}",
                    self.context, self.line_number
                )
            }
            CompileErrorKind::InvalidArgument => {
                write!(
                    f,
                    "Invalid or malformed argument '{}' at line {}",
                    self.context, self.line_number
                )
            }
        }
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug)]
pub enum CompileErrorKind {
    UnknownToken,
    ExpectedToken,
    UnexpectedToken,
    DuplicateLabelDefinition,
    MissingArgument,
    InvalidArgument,
}

#[derive(Debug)]
pub enum RuntimeError {
    UnknownLabel { label: String },
    MachineStopped,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UnknownLabel { label } => {
                write!(f, "Tried to jump to unknown label '{}'", label)
            }
            RuntimeError::MachineStopped => {
                write!(f, "Tried to operate on a stopped machine")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
