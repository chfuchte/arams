use crate::{
    IntoSourceCode,
    lsp::errors::LSPError,
    lsp::token::{LSPToken, LSPTokenKind},
};

pub fn analyze(source_code: impl IntoSourceCode) -> Vec<Vec<LSPToken>> {
    let lines = source_code.into_lines();

    lines
        .iter()
        .enumerate()
        .map(|(line_number, line_str)| {
            let mut tokens: Vec<LSPToken> = vec![];

            let mut words_iter = line_str.split_whitespace();

            while let Some(word) = words_iter.next() {
                match word.to_lowercase().as_str() {
                    _ if tokens.last().is_some_and(|last_token| {
                        matches!(
                            last_token.kind(),
                            LSPTokenKind::Load
                                | LSPTokenKind::Store
                                | LSPTokenKind::Add
                                | LSPTokenKind::Sub
                                | LSPTokenKind::Mul
                                | LSPTokenKind::Div
                                | LSPTokenKind::Goto
                                | LSPTokenKind::Jzero
                                | LSPTokenKind::Jnzero
                                | LSPTokenKind::End
                        )
                    }) =>
                    {
                        // new line should have started here instead, so this should be unreachable with passing code
                        tokens.push(LSPToken::new(
                            LSPTokenKind::Unknown,
                            word.to_string(),
                            vec![LSPError::UnexpectedToken(word.to_string())],
                        ));
                    }
                    "load" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Load,
                                word.to_string(),
                                vec![],
                            ));
                            tokens.push(analyze_operand(arg));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Load,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "store" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Store,
                                word.to_string(),
                                vec![],
                            ));
                            tokens.push(analyze_address(arg));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Store,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "add" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(LSPTokenKind::Add, word.to_string(), vec![]));
                            tokens.push(analyze_operand(arg));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Add,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "sub" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(LSPTokenKind::Sub, word.to_string(), vec![]));
                            tokens.push(analyze_operand(arg));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Sub,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "mul" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(LSPTokenKind::Mul, word.to_string(), vec![]));
                            tokens.push(analyze_operand(arg));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Mul,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "div" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(LSPTokenKind::Div, word.to_string(), vec![]));
                            tokens.push(analyze_operand(arg));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Div,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "goto" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Goto,
                                word.to_string(),
                                vec![],
                            ));
                            tokens.push(LSPToken::new(
                                LSPTokenKind::JumpArgument,
                                arg.to_string(),
                                vec![],
                            ));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Goto,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "jzero" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Jzero,
                                word.to_string(),
                                vec![],
                            ));
                            tokens.push(LSPToken::new(
                                LSPTokenKind::JumpArgument,
                                arg.to_string(),
                                vec![],
                            ));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Jzero,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "jnzero" => match words_iter.next() {
                        Some(arg) => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Jnzero,
                                word.to_string(),
                                vec![],
                            ));
                            tokens.push(LSPToken::new(
                                LSPTokenKind::JumpArgument,
                                arg.to_string(),
                                vec![],
                            ));
                        }
                        None => {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Jnzero,
                                word.to_string(),
                                vec![LSPError::MissingArgument],
                            ));
                        }
                    },
                    "end" => {
                        tokens.push(LSPToken::new(LSPTokenKind::End, word.to_string(), vec![]));
                    }
                    _ if word.ends_with(':') => {
                        let label = word.trim_end_matches(':');
                        if !label.is_empty() {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::LabelDefinition,
                                label.to_string(),
                                vec![],
                            ));
                        } else {
                            tokens.push(LSPToken::new(
                                LSPTokenKind::Unknown,
                                word.to_string(),
                                vec![LSPError::UnknownToken(word.to_string())],
                            ));
                        }
                    }
                    _ if word.starts_with("//") => {
                        let comment = words_iter.collect::<Vec<&str>>().join(" ");
                        tokens.push(LSPToken::new(LSPTokenKind::Comment, comment, vec![]));
                        break; // all remaining words are part of the comment and already processed
                    }
                    _ => {
                        tokens.push(LSPToken::new(
                            LSPTokenKind::Unknown,
                            word.to_string(),
                            vec![LSPError::UnknownToken(word.to_string())],
                        ));
                    }
                }
            }

            if lines.len() > 1 && lines.len() > line_number + 1 {
                tokens.push(LSPToken::new(LSPTokenKind::NewLine, String::new(), vec![]));
            }

            tokens
        })
        .collect()
}

fn analyze_operand(s: &str) -> LSPToken {
    if s.starts_with('#') {
        match s.trim_start_matches('#').parse::<u64>() {
            Ok(_) => LSPToken::new(LSPTokenKind::ImmediateArgument, s.to_string(), vec![]),
            Err(_) => LSPToken::new(
                LSPTokenKind::ImmediateArgument,
                s.to_string(),
                vec![LSPError::InvalidArgument(s.to_string())],
            ),
        }
    } else if s.starts_with('*') {
        match s.trim_start_matches('*').parse::<usize>() {
            Ok(_) => LSPToken::new(LSPTokenKind::IndirectAddressArgument, s.to_string(), vec![]),
            Err(_) => LSPToken::new(
                LSPTokenKind::IndirectAddressArgument,
                s.to_string(),
                vec![LSPError::InvalidArgument(s.to_string())],
            ),
        }
    } else {
        match s.parse::<usize>() {
            Ok(_) => LSPToken::new(LSPTokenKind::DirectAddressArgument, s.to_string(), vec![]),
            Err(_) => LSPToken::new(
                LSPTokenKind::DirectAddressArgument,
                s.to_string(),
                vec![LSPError::InvalidArgument(s.to_string())],
            ),
        }
    }
}

fn analyze_address(s: &str) -> LSPToken {
    if s.starts_with('*') {
        match s.trim_start_matches('*').parse::<usize>() {
            Ok(_) => LSPToken::new(LSPTokenKind::IndirectAddressArgument, s.to_string(), vec![]),
            Err(_) => LSPToken::new(
                LSPTokenKind::IndirectAddressArgument,
                s.to_string(),
                vec![LSPError::InvalidArgument(s.to_string())],
            ),
        }
    } else {
        match s.parse::<usize>() {
            Ok(_) => LSPToken::new(LSPTokenKind::DirectAddressArgument, s.to_string(), vec![]),
            Err(_) => LSPToken::new(
                LSPTokenKind::DirectAddressArgument,
                s.to_string(),
                vec![LSPError::InvalidArgument(s.to_string())],
            ),
        }
    }
}
