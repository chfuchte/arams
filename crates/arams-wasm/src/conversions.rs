use crate::models::{CompilationError, RuntimeError};

impl From<arams_core::CompileError> for CompilationError {
    fn from(err: arams_core::CompileError) -> Self {
        Self {
            line: err.line_number(),
            message: err.to_string(),
        }
    }
}

impl From<arams_core::RuntimeError> for RuntimeError {
    fn from(err: arams_core::RuntimeError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}
