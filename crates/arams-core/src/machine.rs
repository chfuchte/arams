use std::collections::HashMap;

#[derive(Debug)]
pub struct Machine {
    accumulator: Value,
    registers: HashMap<Register, Value>,
    state: MachineState,
}

impl Machine {
    pub fn get_accumulator(&self) -> Value {
        self.accumulator
    }

    pub fn get_registers(&self) -> &HashMap<Register, Value> {
        &self.registers
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.state, MachineState::Stopped(_))
    }

    pub fn get_stopped_reason(&self) -> Option<&StoppedReason> {
        match &self.state {
            MachineState::Stopped(reason) => Some(reason),
            _ => None,
        }
    }

    pub fn preseed_register(&mut self, register: Register, value: Value) {
        self.registers.insert(register, value);
    }

    pub fn preseed_registers(&mut self, registers: HashMap<Register, Value>) {
        self.registers = registers;
    }

    pub fn end(&mut self) {
        self.state = MachineState::Stopped(StoppedReason::EndInstruction);
    }

    pub fn get_register_value(&self, register: Register) -> &Value {
        self.registers.get(&register).unwrap_or(&0)
    }

    pub fn set_register_value(&mut self, register: Register, value: Value) {
        self.registers.insert(register, value);
    }

    pub fn add(&mut self, value: Value) {
        self.accumulator = self.accumulator.wrapping_add(value);
    }

    pub fn sub(&mut self, value: Value) {
        self.accumulator = self.accumulator.wrapping_sub(value);
    }

    pub fn mul(&mut self, value: Value) {
        self.accumulator = self.accumulator.wrapping_mul(value);
    }

    pub fn div(&mut self, value: Value) {
        if value == 0 {
            self.state = MachineState::Stopped(StoppedReason::DivisionByZero);
            return;
        }
        self.accumulator = self.accumulator.wrapping_div(value);
    }

    pub fn load(&mut self, value: Value) {
        self.accumulator = value;
    }

    pub fn store(&mut self, address: Register) {
        self.set_register_value(address, self.accumulator);
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            accumulator: 0,
            registers: HashMap::new(),
            state: MachineState::Running,
        }
    }
}

pub type Register = usize;

pub type Value = u64;

#[derive(Debug)]
pub enum MachineState {
    Running,
    Stopped(StoppedReason),
}

#[derive(Debug)]
pub enum StoppedReason {
    EndInstruction,
    DivisionByZero,
}
