use crate::parser::parse;

#[test]
fn test_parse_complex_program_is_ok() {
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

    let program = parse(program.join("\n"));
    assert!(program.is_ok());
}
