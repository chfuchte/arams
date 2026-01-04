use arams_core::{compile, execute};

const FACTORIAL_PROGRAM: &str = "load 1
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
fn factorial_program_compiles() {
    let program = compile(FACTORIAL_PROGRAM);
    assert!(program.is_ok());
}

#[test]
fn factorial_program_exectutes() {
    let program = compile(FACTORIAL_PROGRAM).unwrap();
    let regs_map = std::collections::HashMap::from([(1, 3)]);
    let machine = execute(program, Some(regs_map));
    assert!(machine.is_ok());
    let machine = machine.unwrap();
    assert_eq!(machine.get_accumulator(), 6);
}
