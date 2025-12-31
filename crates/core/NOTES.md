# Notes

## Tokenize Function

As it turned out, a simple for loop is more efficient than flat_mapping. I still want to preserve this solution for future experiments.

```rs
fn tokenize_itered(source_code: impl IntoSourceCode) -> Vec<Token> {
    let lines = source_code.into_lines();
    let line_count = lines.len();

    lines
        .iter()
        .enumerate()
        .flat_map(|(idx, line)| {
            let trimmed = line.trim();
            let mut words_iter = trimmed.split_whitespace().peekable();
            let mut line_tokens = vec![];

            while let Some(word) = words_iter.next() {
                if word.eq_ignore_ascii_case("load") {
                    line_tokens.push(Token::Load);
                } else if word.eq_ignore_ascii_case("store") {
                    line_tokens.push(Token::Store);
                } else if word.eq_ignore_ascii_case("add") {
                    line_tokens.push(Token::Add);
                } else if word.eq_ignore_ascii_case("sub") {
                    line_tokens.push(Token::Sub);
                } else if word.eq_ignore_ascii_case("mul") {
                    line_tokens.push(Token::Mul);
                } else if word.eq_ignore_ascii_case("div") {
                    line_tokens.push(Token::Div);
                } else if word.eq_ignore_ascii_case("goto") {
                    line_tokens.push(Token::Goto);
                } else if word.eq_ignore_ascii_case("jzero") {
                    line_tokens.push(Token::Jzero);
                } else if word.eq_ignore_ascii_case("jnzero") {
                    line_tokens.push(Token::Jnzero);
                } else if word.eq_ignore_ascii_case("end") {
                    line_tokens.push(Token::End);
                } else if word.starts_with("//") {
                    let comment = words_iter.collect::<Vec<&str>>().join(" ");
                    line_tokens.push(Token::Comment(comment));
                    break;
                } else if word.ends_with(':') {
                    let label_name = word.trim_end_matches(':').to_string();
                    line_tokens.push(Token::LabelDefinition(label_name));
                } else if let Some(&next_word) = words_iter.peek() {
                    if next_word == ":" {
                        line_tokens.push(Token::LabelDefinition(word.to_string()));
                        words_iter.next();
                    } else {
                        line_tokens.push(Token::Argument(word.to_string()));
                    }
                } else {
                    line_tokens.push(Token::Argument(word.to_string()));
                }
            }

            if line_count > 1 && line_count > idx + 1 {
                line_tokens.push(Token::NewLine);
            }

            line_tokens
        })
        .collect()
}
```
