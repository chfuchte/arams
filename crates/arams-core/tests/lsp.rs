use arams_core::lsp::{LSPError, LSPTokenKind, analyze};

const FACTORIAL_PROGRAM: &str = "load 1
    store // no argument here to trigger an error
    jzero return_one
    sub #1
    jzero return_one
    load 1
    store 2
    loop: load 1
    sub #1
    jzero break
    store 1
    mul 2
    store 2
    goto loop
    return_one: load #1
    end
    break: load 2
    end";

#[test]
fn factorial_program_gets_analyzed() {
    let lines = analyze(FACTORIAL_PROGRAM);
    assert_eq!(lines.len(), 18);

    println!("{:#?}", lines);

    // line 1: load 1
    assert_eq!(lines[0][0].kind(), &LSPTokenKind::Load);
    assert_eq!(lines[0][0].lexeme(), "load");
    assert_eq!(lines[0][0].errors().len(), 0);
    assert_eq!(lines[0][1].kind(), &LSPTokenKind::DirectAddressArgument);
    assert_eq!(lines[0][1].lexeme(), "1");
    assert_eq!(lines[0][1].errors().len(), 0);
    assert_eq!(lines[0][2].kind(), &LSPTokenKind::NewLine);
    // line 2: store // no argument here to trigger an error
    assert_eq!(lines[1][0].kind(), &LSPTokenKind::Store);
    assert_eq!(lines[1][0].lexeme(), "store");
    assert_eq!(lines[1][1].errors().len(), 1);
    assert!(matches!(
        lines[1][1].errors()[0],
        LSPError::InvalidArgument(_)
    ));
}
