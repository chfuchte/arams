use crate::errors::RuntimeError;
use std::collections::HashMap;

pub struct ProgramBuilder {
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.insert(label, self.instructions.len());
    }

    pub fn build(self) -> Program {
        Program::new(self.instructions, self.labels)
    }

    pub fn label_exists(&self, label: &String) -> bool {
        self.labels.contains_key(label)
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
    program_counter: usize,
}

impl Program {
    pub(crate) fn new(
        instructions: Vec<Instruction>,
        labels: HashMap<String, usize>,
    ) -> Self {
        Self {
            instructions,
            labels,
            program_counter: 0,
        }
    }

    pub fn fetch(&mut self) -> Option<Instruction> {
        let instruction = self.instructions.get(self.program_counter)?.clone();
        Some(instruction)
    }

    pub fn advance(&mut self) {
        self.program_counter += 1;
    }

    pub fn goto(&mut self, label: &String) -> Result<(), RuntimeError> {
        if let Some(&line_number) = self.labels.get(label) {
            self.program_counter = line_number;
            Ok(())
        } else {
            Err(RuntimeError::UnknownString {
                label: label.to_string(),
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Load(Operand),
    Store(Address),
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
    Goto(String),
    Jzero(String),
    Jnzero(String),
    End,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Immediate(u64),
    DirectAddress(usize),
    IndirectAddress(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Address {
    Direct(usize),
    Indirect(usize),
}
