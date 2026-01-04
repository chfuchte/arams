use std::collections::HashMap;

use crate::errors::RuntimeError;

#[derive(Clone, Debug, PartialEq)]
pub struct Machine {
    accumulator: u64,
    registers: HashMap<usize, u64>,
    running: bool,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_preseeded_registers(registers: HashMap<usize, u64>) -> Self {
        Self {
            accumulator: 0,
            registers,
            running: true,
        }
    }

    pub fn get_accumulator(&self) -> u64 {
        self.accumulator
    }

    pub fn get_registers(&self) -> &HashMap<usize, u64> {
        &self.registers
    }

    pub fn get_register_value(&self, register: usize) -> &u64 {
        self.registers.get(&register).unwrap_or(&0)
    }

    pub fn is_stopped(&self) -> bool {
        !self.running
    }

    pub fn set_register_value(&mut self, register: usize, value: u64) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.registers.insert(register, value);

        Ok(())
    }

    pub fn add(&mut self, value: u64) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.accumulator = self.accumulator.wrapping_add(value);

        Ok(())
    }

    pub fn sub(&mut self, value: u64) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.accumulator = self.accumulator.wrapping_sub(value);

        Ok(())
    }

    pub fn mul(&mut self, value: u64) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.accumulator = self.accumulator.wrapping_mul(value);

        Ok(())
    }

    pub fn div(&mut self, value: u64) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        if value == 0 {
            self.running = false;
        } else {
            self.accumulator = self.accumulator.wrapping_div(value);
        }

        Ok(())
    }

    pub fn load(&mut self, value: u64) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.accumulator = value;

        Ok(())
    }

    pub fn store(&mut self, address: usize) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.registers.insert(address, self.accumulator);

        Ok(())
    }

    pub fn end(&mut self) -> Result<(), RuntimeError> {
        if !self.running {
            return Err(RuntimeError::MachineStopped);
        }

        self.running = false;

        Ok(())
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            accumulator: 0,
            registers: HashMap::new(),
            running: true,
        }
    }
}
