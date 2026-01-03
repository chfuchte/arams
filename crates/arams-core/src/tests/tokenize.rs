use crate::tokenizer::{Token, tokenize};

#[test]
fn test_empty_program() {
    let empty_program = "";
    let whitespace_only_program = "     ";
    let lf_newline_program = "\n\n\n ";
    let crlf_newline_program = "\r\n\r\n\r\n";

    let tokens = tokenize(empty_program);
    assert!(tokens.is_empty(), "Expected no tokens for empty program");

    let tokens = tokenize(whitespace_only_program);
    assert!(
        tokens.is_empty(),
        "Expected no tokens for whitespace only program"
    );

    let tokens = tokenize(lf_newline_program);
    assert_eq!(tokens.len(), 3, "Expected 3 newline tokens");
    assert!(tokens.iter().all(|token| matches!(token, Token::NewLine)));

    let tokens = tokenize(crlf_newline_program);
    assert_eq!(tokens.len(), 2, "Expected 2 newline tokens");
    assert!(tokens.iter().all(|token| matches!(token, Token::NewLine)));
}

#[test]
fn test_tokenize_complex_program() {
    let program = [
        "// This is a sample program",
        "start:       load #10   // Load the value 10",
        "store 2",
        "       add *3",
        "              sub 1",
        "mul 4",
        "div 2",
        "goto start",
        "jzero e",
        "jnzero start",
        "e:     ",
        "end",
    ];

    let tokens = tokenize(program.join("\n"));
    assert_eq!(
        tokens,
        vec![
            Token::Comment("This is a sample program".to_string()),
            Token::NewLine,
            Token::LabelDefinition("start".to_string()),
            Token::Load,
            Token::Argument("#10".to_string()),
            Token::Comment("Load the value 10".to_string()),
            Token::NewLine,
            Token::Store,
            Token::Argument("2".to_string()),
            Token::NewLine,
            Token::Add,
            Token::Argument("*3".to_string()),
            Token::NewLine,
            Token::Sub,
            Token::Argument("1".to_string()),
            Token::NewLine,
            Token::Mul,
            Token::Argument("4".to_string()),
            Token::NewLine,
            Token::Div,
            Token::Argument("2".to_string()),
            Token::NewLine,
            Token::Goto,
            Token::Argument("start".to_string()),
            Token::NewLine,
            Token::Jzero,
            Token::Argument("e".to_string()),
            Token::NewLine,
            Token::Jnzero,
            Token::Argument("start".to_string()),
            Token::NewLine,
            Token::LabelDefinition("e".to_string()),
            Token::NewLine,
            Token::End,
        ]
    );
}
