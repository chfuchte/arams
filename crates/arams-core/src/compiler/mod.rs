mod parser;
mod source_code;
mod token;
mod tokenizer;

pub use source_code::IntoSourceCode;

pub fn compile(
    source_code: impl IntoSourceCode,
) -> Result<crate::program::Program, Vec<crate::errors::CompileError>> {
    let tokens = self::tokenizer::tokenize(source_code)?;
    let program = self::parser::parse(tokens)?;

    Ok(program)
}
