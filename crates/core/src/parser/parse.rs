use crate::{
    IntoSourceCode,
    errors::ParserError,
    machine::{Register, Value},
    program::{Address, Instruction, Line, Operand, Program, ProgramBuilder},
};

/// Parses the given source code into a `Program` which represents the AST of the code.
pub fn parse(source_code: impl IntoSourceCode) -> Result<Program, Vec<ParserError>> {
    let mut program_builder = ProgramBuilder::new();
    let mut errors = Vec::new();

    source_code.into_lines().iter().for_each(|line| {
        if line.trim().is_empty() {
            return;
        }

        let mut words_iter = line.split_whitespace();

        while let Some(word) = words_iter.next() {
            match word.to_lowercase().as_str() {
                "load" => match words_iter.next() {
                    Some(operand_str) => match operand_from_str(operand_str) {
                        Ok(operand) => {
                            program_builder.add_line(Line::new(
                                Instruction::Load(operand),
                                program_builder.current_instruction_index(),
                            ));
                            break;
                        }
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(ParserError::AnyError(
                        "Missing operand for LOAD instruction".to_string(),
                    )),
                },
                "store" => match words_iter.next() {
                    Some(address_str) => match address_from_str(address_str) {
                        Ok(address) => {
                            program_builder.add_line(Line::new(
                                Instruction::Store(address),
                                program_builder.current_instruction_index(),
                            ));
                            break;
                        }
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(ParserError::AnyError(
                        "Missing address for STORE instruction".to_string(),
                    )),
                },
                "add" => match words_iter.next() {
                    Some(operand_str) => match operand_from_str(operand_str) {
                        Ok(operand) => {
                            program_builder.add_line(Line::new(
                                Instruction::Add(operand),
                                program_builder.current_instruction_index(),
                            ));
                            break;
                        }
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(ParserError::AnyError(
                        "Missing operand for ADD instruction".to_string(),
                    )),
                },
                "sub" => match words_iter.next() {
                    Some(operand_str) => match operand_from_str(operand_str) {
                        Ok(operand) => {
                            program_builder.add_line(Line::new(
                                Instruction::Sub(operand),
                                program_builder.current_instruction_index(),
                            ));
                            break;
                        }
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(ParserError::AnyError(
                        "Missing operand for SUB instruction".to_string(),
                    )),
                },
                "mul" => match words_iter.next() {
                    Some(operand_str) => match operand_from_str(operand_str) {
                        Ok(operand) => {
                            program_builder.add_line(Line::new(
                                Instruction::Mul(operand),
                                program_builder.current_instruction_index(),
                            ));
                            break;
                        }
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(ParserError::AnyError(
                        "Missing operand for MUL instruction".to_string(),
                    )),
                },
                "div" => match words_iter.next() {
                    Some(operand_str) => match operand_from_str(operand_str) {
                        Ok(operand) => {
                            program_builder.add_line(Line::new(
                                Instruction::Div(operand),
                                program_builder.current_instruction_index(),
                            ));
                            break;
                        }
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(ParserError::AnyError(
                        "Missing operand for DIV instruction".to_string(),
                    )),
                },
                "goto" => match words_iter.next() {
                    Some(label) => {
                        program_builder.add_line(Line::new(
                            Instruction::Goto(label.to_string()),
                            program_builder.current_instruction_index(),
                        ));
                        break;
                    }
                    None => errors.push(ParserError::AnyError(
                        "Missing label for GOTO instruction".to_string(),
                    )),
                },
                "jzero" => match words_iter.next() {
                    Some(label) => {
                        program_builder.add_line(Line::new(
                            Instruction::Jzero(label.to_string()),
                            program_builder.current_instruction_index(),
                        ));
                        break;
                    }
                    None => errors.push(ParserError::AnyError(
                        "Missing label for JZERO instruction".to_string(),
                    )),
                },
                "jnzero" => match words_iter.next() {
                    Some(label) => {
                        program_builder.add_line(Line::new(
                            Instruction::Jnzero(label.to_string()),
                            program_builder.current_instruction_index(),
                        ));
                        break;
                    }
                    None => errors.push(ParserError::AnyError(
                        "Missing label for JNZERO instruction".to_string(),
                    )),
                },
                "end" => {
                    program_builder.add_line(Line::new(
                        Instruction::End,
                        program_builder.current_instruction_index(),
                    ));
                    break;
                }
                _ if word.ends_with(':') => {
                    let label = word.trim_end_matches(':').to_string();
                    program_builder.add_label(label, program_builder.current_instruction_index());
                }
                _ if word.starts_with("//") => {
                    break;
                }
                _ => {
                    errors.push(ParserError::AnyError(format!(
                        "Unknown instruction or malformed line: '{}'",
                        word
                    )));
                }
            }
        }
    });

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(program_builder.build())
}

fn operand_from_str(s: &str) -> Result<Operand, ParserError> {
    if s.starts_with('#') {
        let value = s
            .trim_start_matches('#')
            .parse::<Value>()
            .map_err(|_| ParserError::AnyError(s.to_string()))?;
        Ok(Operand::Immediate(value))
    } else if s.starts_with('*') {
        let address = s
            .trim_start_matches('*')
            .parse::<Register>()
            .map_err(|_| ParserError::AnyError(s.to_string()))?;
        Ok(Operand::IndirectAddress(address))
    } else {
        let address = s
            .parse::<Register>()
            .map_err(|_| ParserError::AnyError(s.to_string()))?;
        Ok(Operand::DirectAddress(address))
    }
}

fn address_from_str(s: &str) -> Result<Address, ParserError> {
    if s.starts_with('*') {
        let address = s
            .trim_start_matches('*')
            .parse::<Register>()
            .map_err(|_| ParserError::AnyError(s.to_string()))?;
        Ok(Address::Indirect(address))
    } else {
        let address = s
            .parse::<Register>()
            .map_err(|_| ParserError::AnyError(s.to_string()))?;
        Ok(Address::Direct(address))
    }
}
