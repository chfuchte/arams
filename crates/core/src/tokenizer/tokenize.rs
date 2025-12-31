use super::token::Token;

pub(crate) fn tokenize(source_code: impl IntoSourceCode) -> Vec<Token> {
    let mut tokens = vec![];
    let lines = source_code.into_lines();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        let mut words_iter = trimmed.split_whitespace().peekable();

        while let Some(word) = words_iter.next() {
            if word.eq_ignore_ascii_case("load") {
                tokens.push(Token::Load);
            } else if word.eq_ignore_ascii_case("store") {
                tokens.push(Token::Store);
            } else if word.eq_ignore_ascii_case("add") {
                tokens.push(Token::Add);
            } else if word.eq_ignore_ascii_case("sub") {
                tokens.push(Token::Sub);
            } else if word.eq_ignore_ascii_case("mul") {
                tokens.push(Token::Mul);
            } else if word.eq_ignore_ascii_case("div") {
                tokens.push(Token::Div);
            } else if word.eq_ignore_ascii_case("goto") {
                tokens.push(Token::Goto);
            } else if word.eq_ignore_ascii_case("jzero") {
                tokens.push(Token::Jzero);
            } else if word.eq_ignore_ascii_case("jnzero") {
                tokens.push(Token::Jnzero);
            } else if word.eq_ignore_ascii_case("end") {
                tokens.push(Token::End);
            } else if word.starts_with("//") {
                let comment = words_iter.collect::<Vec<&str>>().join(" ");
                tokens.push(Token::Comment(comment));
                break;
            } else if word.ends_with(':') {
                let label_name = word.trim_end_matches(':').to_string();
                tokens.push(Token::LabelDefinition(label_name));
            } else if let Some(&next_word) = words_iter.peek() {
                if next_word == ":" {
                    let label_name = word.to_string();
                    tokens.push(Token::LabelDefinition(label_name));
                    words_iter.next();
                } else {
                    tokens.push(Token::Argument(word.to_string()));
                }
            } else {
                tokens.push(Token::Argument(word.to_string()));
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

impl IntoSourceCode for &String {
    fn into_lines(self) -> Vec<String> {
        self.lines().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for &[String] {
    fn into_lines(self) -> Vec<String> {
        self.iter().cloned().collect()
    }
}

impl IntoSourceCode for &[&str] {
    fn into_lines(self) -> Vec<String> {
        self.iter().map(|&line| line.to_string()).collect()
    }
}
