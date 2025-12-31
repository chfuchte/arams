use super::token::Token;

pub(crate) fn tokenize(source_code: impl IntoSourceCode) -> Vec<Token> {
    let mut tokens = vec![];
    let lines = source_code.into_lines();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        let words = trimmed.split_whitespace().collect::<Vec<&str>>();
        let mut words_iter = words.iter();
        while let Some(word) = words_iter.next() {
            match word.to_lowercase().as_str() {
                "load" => tokens.push(Token::Load),
                "store" => tokens.push(Token::Store),
                "add" => tokens.push(Token::Add),
                "sub" => tokens.push(Token::Sub),
                "mul" => tokens.push(Token::Mul),
                "div" => tokens.push(Token::Div),
                "goto" => tokens.push(Token::Goto),
                "jzero" => tokens.push(Token::Jzero),
                "jnzero" => tokens.push(Token::Jnzero),
                "end" => tokens.push(Token::End),
                _ => {
                    // comment
                    if word.starts_with("//") {
                        let comment = words_iter
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                            .join(" ");
                        tokens.push(Token::Comment(comment));
                        break;
                    }

                    // label definition
                    if word.ends_with(':') {
                        let label_name = word.trim_end_matches(':').to_string();
                        tokens.push(Token::LabelDefinition(label_name));
                        continue;
                    } else if let Some(next_word) = words_iter.clone().next() {
                        if next_word.trim() == ":" {
                            let label_name = word.to_string();
                            tokens.push(Token::LabelDefinition(label_name));
                            words_iter.next();
                            continue;
                        }
                    }

                    // operands and label references
                    tokens.push(Token::Argument(word.to_string()));
                }
            }
        }

        if lines.len() > 1 && lines.len() > idx + 1 {
            tokens.push(Token::NewLine);
        }
    }

    tokens
}

pub trait IntoSourceCode {
    fn into_lines(self) -> Vec<String>;
}

impl IntoSourceCode for Vec<String> {
    fn into_lines(self) -> Vec<String> {
        self
    }
}

impl IntoSourceCode for &str {
    fn into_lines(self) -> Vec<String> {
        self.lines().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for String {
    fn into_lines(self) -> Vec<String> {
        self.lines().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for &[String] {
    fn into_lines(self) -> Vec<String> {
        self.iter().cloned().collect()
    }
}
