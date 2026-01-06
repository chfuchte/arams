use serde::{Deserialize, Serialize};

use super::{CompilationError, Machine, RuntimeError};

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum RunResult {
    #[serde(rename = "machine")]
    Ok(Machine),
    #[serde(rename = "compilation_error")]
    CompilationError(Vec<CompilationError>),
    #[serde(rename = "execution_error")]
    ExecutionError(RuntimeError),
}
