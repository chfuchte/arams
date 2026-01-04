use std::num::ParseIntError;

use crate::{
    compiler::token::Token,
    errors::{CompileError, CompileErrorKind},
    program::{Address, Instruction, Operand, Program, ProgramBuilder},
};

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Program, Vec<CompileError>> {
    let mut program_builder = ProgramBuilder::new();
    let mut errors = Vec::new();

    let mut token_iter = tokens.iter();

    while let Some(token) = token_iter.next() {
        match token {
            Token::Load { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: arg_line,
                    value,
                }) => match operand_from_str(value) {
                    Ok(operand) => {
                        program_builder.add_instruction(Instruction::Load(operand));
                    }
                    Err(_) => {
                        errors.push(CompileError::new(
                            *arg_line,
                            CompileErrorKind::InvalidArgument,
                            value.to_string(),
                        ));
                    }
                },
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "load".to_string(),
                    ));
                }
            },
            Token::Store { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: arg_line,
                    value,
                }) => match address_from_str(value) {
                    Ok(address) => {
                        program_builder.add_instruction(Instruction::Store(address));
                    }
                    Err(_) => {
                        errors.push(CompileError::new(
                            *arg_line,
                            CompileErrorKind::InvalidArgument,
                            value.to_string(),
                        ));
                    }
                },
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "store".to_string(),
                    ));
                }
            },
            Token::Add { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: arg_line,
                    value,
                }) => match operand_from_str(value) {
                    Ok(operand) => {
                        program_builder.add_instruction(Instruction::Add(operand));
                    }
                    Err(_) => {
                        errors.push(CompileError::new(
                            *arg_line,
                            CompileErrorKind::InvalidArgument,
                            value.to_string(),
                        ));
                    }
                },
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "add".to_string(),
                    ));
                }
            },
            Token::Sub { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: arg_line,
                    value,
                }) => match operand_from_str(value) {
                    Ok(operand) => {
                        program_builder.add_instruction(Instruction::Sub(operand));
                    }
                    Err(_) => {
                        errors.push(CompileError::new(
                            *arg_line,
                            CompileErrorKind::InvalidArgument,
                            value.to_string(),
                        ));
                    }
                },
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "sub".to_string(),
                    ));
                }
            },
            Token::Mul { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: arg_line,
                    value,
                }) => match operand_from_str(value) {
                    Ok(operand) => {
                        program_builder.add_instruction(Instruction::Mul(operand));
                    }
                    Err(_) => {
                        errors.push(CompileError::new(
                            *arg_line,
                            CompileErrorKind::InvalidArgument,
                            value.to_string(),
                        ));
                    }
                },
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "mul".to_string(),
                    ));
                }
            },
            Token::Div { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: arg_line,
                    value,
                }) => match operand_from_str(value) {
                    Ok(operand) => {
                        program_builder.add_instruction(Instruction::Div(operand));
                    }
                    Err(_) => {
                        errors.push(CompileError::new(
                            *arg_line,
                            CompileErrorKind::InvalidArgument,
                            value.to_string(),
                        ));
                    }
                },
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "div".to_string(),
                    ));
                }
            },
            Token::Goto { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: _,
                    value,
                }) => {
                    program_builder.add_instruction(Instruction::Goto(value.to_string()));
                }
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "goto".to_string(),
                    ));
                }
            },
            Token::Jzero { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: _,
                    value,
                }) => {
                    program_builder.add_instruction(Instruction::Jzero(value.to_string()));
                }
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "jzero".to_string(),
                    ));
                }
            },
            Token::Jnzero { line_number } => match token_iter.next() {
                Some(Token::Argument {
                    line_number: _,
                    value,
                }) => {
                    program_builder.add_instruction(Instruction::Jnzero(value.to_string()));
                }
                _ => {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::MissingArgument,
                        "jnzero".to_string(),
                    ));
                }
            },
            Token::End { line_number: _ } => {
                program_builder.add_instruction(Instruction::End);
            }
            Token::LabelDefinition { line_number, value } => {
                if program_builder.label_exists(value) {
                    errors.push(CompileError::new(
                        *line_number,
                        CompileErrorKind::DuplicateStringDefinition,
                        value.to_string(),
                    ));
                    continue;
                }
                program_builder.add_label(value.to_string());
            }
            Token::Argument { line_number, value } => {
                // all arguments are handled at the instructions that require them
                errors.push(CompileError::new(
                    *line_number,
                    CompileErrorKind::UnexpectedToken,
                    value.to_string(),
                ));
            }
            _ => {}
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(program_builder.build())
}

fn operand_from_str(s: &str) -> Result<Operand, ParseIntError> {
    if s.starts_with('#') {
        let value = s.trim_start_matches('#').parse::<u64>()?;
        Ok(Operand::Immediate(value))
    } else if s.starts_with('*') {
        let address = s.trim_start_matches('*').parse::<usize>()?;
        Ok(Operand::IndirectAddress(address))
    } else {
        let address = s.parse::<usize>()?;
        Ok(Operand::DirectAddress(address))
    }
}

fn address_from_str(s: &str) -> Result<Address, ParseIntError> {
    if s.starts_with('*') {
        let address = s.trim_start_matches('*').parse::<usize>()?;
        Ok(Address::Indirect(address))
    } else {
        let address = s.parse::<usize>()?;
        Ok(Address::Direct(address))
    }
}
