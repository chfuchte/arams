use crate::{
    compiler::source_code::IntoSourceCode,
    compiler::token::Token,
    errors::{CompileError, CompileErrorKind},
};

pub fn tokenize(source_code: impl IntoSourceCode) -> Result<Vec<Token>, Vec<CompileError>> {
    let mut tokens = vec![];
    let mut errors = vec![];

    let lines = source_code.into_lines();

    lines.iter().enumerate().for_each(|(line_number, line)| {
        let mut words_iter = line.split_whitespace();

        while let Some(word) = words_iter.next() {
            match word.to_lowercase().as_str() {
                "load" => {
                    tokens.push(Token::Load {
                        line_number: line_number + 1,
                    });
                }
                "store" => {
                    tokens.push(Token::Store {
                        line_number: line_number + 1,
                    });
                }
                "add" => {
                    tokens.push(Token::Add {
                        line_number: line_number + 1,
                    });
                }
                "sub" => {
                    tokens.push(Token::Sub {
                        line_number: line_number + 1,
                    });
                }
                "mul" => {
                    tokens.push(Token::Mul {
                        line_number: line_number + 1,
                    });
                }
                "div" => {
                    tokens.push(Token::Div {
                        line_number: line_number + 1,
                    });
                }
                "goto" => {
                    tokens.push(Token::Goto {
                        line_number: line_number + 1,
                    });
                }
                "jzero" => {
                    tokens.push(Token::Jzero {
                        line_number: line_number + 1,
                    });
                }
                "jnzero" => {
                    tokens.push(Token::Jnzero {
                        line_number: line_number + 1,
                    });
                }
                "end" => {
                    tokens.push(Token::End {
                        line_number: line_number + 1,
                    });
                }
                _ if word.starts_with("//") => {
                    let comment = words_iter.collect::<Vec<&str>>().join(" ");
                    tokens.push(Token::Comment {
                        line_number: line_number + 1,
                        value: comment,
                    });
                    break; // all remaining words are part of the comment and already processed
                }
                _ if word.ends_with(':') => {
                    let label_name = word.trim_end_matches(':').to_string();
                    if label_name.is_empty() {
                        errors.push(CompileError::new(
                            line_number + 1,
                            CompileErrorKind::UnknownToken,
                            word.to_string(),
                        ));
                        continue;
                    }
                    tokens.push(Token::LabelDefinition {
                        line_number: line_number + 1,
                        value: label_name,
                    });
                }
                _ if tokens.last().is_some_and(|last_token| {
                    !matches!(
                        last_token,
                        Token::NewLine { line_number: _ }
                            | Token::Comment {
                                line_number: _,
                                value: _
                            }
                            | Token::LabelDefinition {
                                line_number: _,
                                value: _
                            }
                            | Token::Argument {
                                line_number: _,
                                value: _
                            }
                    )
                }) =>
                {
                    tokens.push(Token::Argument {
                        line_number: line_number + 1,
                        value: word.to_string(),
                    });
                }
                _ if tokens.last().is_some_and(|last_token| {
                    matches!(
                        last_token,
                        Token::Argument {
                            line_number: _,
                            value: _
                        }
                    )
                }) =>
                {
                    errors.push(CompileError::new(
                        line_number + 1,
                        CompileErrorKind::ExpectedToken,
                        format!("new line, got '{}'", word),
                    ));
                }
                _ => {
                    errors.push(CompileError::new(
                        line_number + 1,
                        CompileErrorKind::UnknownToken,
                        word.to_string(),
                    ));
                }
            }
        }

        if lines.len() > 1 && lines.len() > line_number + 1 {
            tokens.push(Token::NewLine {
                line_number: line_number + 1,
            });
        }
    });

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(tokens)
}
