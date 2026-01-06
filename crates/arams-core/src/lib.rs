mod compiler;
mod errors;
mod program;
mod runtime;

pub mod lsp;

pub use compiler::{IntoSourceCode, compile};
pub use errors::{CompileError, CompileErrorKind, RuntimeError};
pub use program::Program;
pub use runtime::{Machine, execute};
