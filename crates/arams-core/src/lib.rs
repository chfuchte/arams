mod executor;
mod machine;
mod parser;
mod program;
mod source_code;

#[cfg(test)]
mod tests;

pub mod errors;
pub mod tokenizer;

pub use executor::execute;
pub use machine::Machine;
pub use parser::parse;
pub use program::Program;
pub use source_code::IntoSourceCode;

pub fn hello() -> String {
    "Hello from core!".to_string()
}
