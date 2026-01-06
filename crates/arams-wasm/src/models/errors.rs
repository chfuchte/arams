use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompilationError {
    pub line: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct RuntimeError {
    pub message: String,
}
