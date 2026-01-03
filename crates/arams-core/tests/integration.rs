use arams_core::*;
use std::collections::HashMap;

#[test]
fn test_machine_creation() {
    let mut machine = Machine::default();
    machine.add(10);
    assert_eq!(machine.get_accumulator(), 10);
    machine.store(0);
    assert_eq!(*machine.get_register_value(0), 10);
}

#[test]
fn test_program_execution() {
    let source_code = "
    // Berechnung der Fakultaet
//Eingabe in R1: n
//Ergebnis in R0: n!


//Wir fangen den Fall 0! und 1! ab:
		Load 1
		JZero 01Faku
		Sub #1
		JZero 01Faku

		Load 1
		Store 2
Anfang:		Load 1
		Sub #1
		JZero Ende
		Store 1
		Mul 2
		Store 2
		Goto Anfang

01Faku:		Load #1
		End

Ende:		LOAD 2
 		END";

    let program = parse(source_code);
    assert!(program.is_ok());
    let program = program.unwrap();

    let mut regs_map = HashMap::new();
    regs_map.insert(1, 5);

    let execution_result = execute(program, Some(regs_map));
    assert!(execution_result.is_ok());
    let machine = execution_result.unwrap();
    assert_eq!(machine.get_accumulator(), 120); // 5! = 120 
}
