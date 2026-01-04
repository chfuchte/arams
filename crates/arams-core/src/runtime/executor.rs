use std::collections::HashMap;

use crate::{
    errors::RuntimeError,
    program::{Address, Instruction, Operand, Program},
    runtime::machine::Machine,
};

pub fn execute(
    mut program: Program,
    registers: Option<HashMap<usize, u64>>,
) -> Result<Machine, RuntimeError> {
    let mut machine = match registers {
        Some(regs) => Machine::new_with_preseeded_registers(regs),
        None => Machine::new(),
    };

    while !machine.is_stopped() {
        let instruction = match program.fetch() {
            Some(l) => l,
            None => break,
        };

        match instruction {
            Instruction::Load(operand) => {
                machine.load(match operand {
                    Operand::Immediate(value) => value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(addr) as usize)
                    }
                })?;
            }
            Instruction::Store(address) => {
                machine.store(match address {
                    Address::Direct(addr) => addr,
                    Address::Indirect(addr) => *machine.get_register_value(addr) as usize,
                })?;
            }
            Instruction::Add(operand) => {
                machine.add(match operand {
                    Operand::Immediate(value) => value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(addr) as usize)
                    }
                })?;
            }
            Instruction::Sub(operand) => {
                machine.sub(match operand {
                    Operand::Immediate(value) => value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(addr) as usize)
                    }
                })?;
            }
            Instruction::Mul(operand) => {
                machine.mul(match operand {
                    Operand::Immediate(value) => value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(addr) as usize)
                    }
                })?;
            }
            Instruction::Div(operand) => {
                machine.div(match operand {
                    Operand::Immediate(value) => value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(addr) as usize)
                    }
                })?;
            }
            Instruction::Goto(label) => {
                program.goto(&label)?;
                continue;
            }
            Instruction::Jzero(label) => {
                if machine.get_accumulator() == 0 {
                    program.goto(&label)?;
                    continue;
                }
            }
            Instruction::Jnzero(label) => {
                if machine.get_accumulator() != 0 {
                    program.goto(&label)?;
                    continue;
                }
            }
            Instruction::End => {
                machine.end()?;
            }
        }

        program.advance();
    }

    if !machine.is_stopped() {
        machine.end()?;
    }

    Ok(machine)
}
