use crate::{
    Machine, Program,
    errors::RuntimeError,
    machine::{Register, Value},
    program::{Address, Instruction, Operand},
};
use std::collections::HashMap;

pub fn execute(
    mut program: Program,
    registers: Option<HashMap<Register, Value>>,
) -> Result<Machine, RuntimeError> {
    let mut machine = Machine::default();

    if let Some(regs) = registers {
        machine.preseed_registers(regs);
    }

    while !machine.is_stopped() {
        let line = match program.fetch() {
            Some(l) => l,
            None => break,
        };
        let instruction = line.instruction();

        match instruction {
            Instruction::Load(operand) => {
                machine.load(match operand {
                    Operand::Immediate(value) => *value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(*addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(*addr) as usize)
                    }
                });
            }
            Instruction::Store(address) => {
                machine.store(match address {
                    Address::Direct(addr) => *addr,
                    Address::Indirect(addr) => *machine.get_register_value(*addr) as usize,
                });
            }
            Instruction::Add(operand) => {
                machine.add(match operand {
                    Operand::Immediate(value) => *value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(*addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(*addr) as usize)
                    }
                });
            }
            Instruction::Sub(operand) => {
                machine.sub(match operand {
                    Operand::Immediate(value) => *value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(*addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(*addr) as usize)
                    }
                });
            }
            Instruction::Mul(operand) => {
                machine.mul(match operand {
                    Operand::Immediate(value) => *value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(*addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(*addr) as usize)
                    }
                });
            }
            Instruction::Div(operand) => {
                machine.div(match operand {
                    Operand::Immediate(value) => *value,
                    Operand::DirectAddress(addr) => *machine.get_register_value(*addr),
                    Operand::IndirectAddress(addr) => {
                        *machine.get_register_value(*machine.get_register_value(*addr) as usize)
                    }
                });
            }
            Instruction::Goto(label) => {
                program.goto(label)?;
                continue;
            }
            Instruction::Jzero(label) => {
                if machine.get_accumulator() == 0 {
                    program.goto(label)?;
                    continue;
                }
            }
            Instruction::Jnzero(label) => {
                if machine.get_accumulator() != 0 {
                    program.goto(label)?;
                    continue;
                }
            }
            Instruction::End => {
                machine.end();
            }
        }

        program.advance();
    }

    if !machine.is_stopped() {
        return Err(RuntimeError::AnyError("program did not terminate".into()));
    }

    Ok(machine)
}
