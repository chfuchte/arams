use crate::{
    errors::RuntimeError,
    machine::{Register, Value},
};
use std::collections::HashMap;

pub struct ProgramBuilder {
    lines: Vec<Line>,
    labels: HashMap<Label, LineNumber>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn add_label(&mut self, label: Label, position: LineNumber) {
        self.labels.insert(label, position);
    }

    pub fn build(self) -> Program {
        Program::new(self.lines, self.labels)
    }

    pub fn current_instruction_index(&self) -> LineNumber {
        self.lines.len()
    }
}

pub struct Program {
    pub lines: Vec<Line>,
    pub labels: HashMap<Label, LineNumber>,
    program_counter: LineNumber,
}

impl Program {
    pub(crate) fn new(lines: Vec<Line>, labels: HashMap<Label, LineNumber>) -> Self {
        Self {
            lines,
            labels,
            program_counter: 0,
        }
    }

    pub fn fetch(&mut self) -> Option<Line> {
        let line = self.lines.get(self.program_counter)?.clone();
        Some(line)
    }

    pub fn advance(&mut self) {
        self.program_counter += 1;
    }

    pub fn goto(&mut self, label: &Label) -> Result<(), RuntimeError> {
        if let Some(&line_number) = self.labels.get(label) {
            self.program_counter = line_number;
            Ok(())
        } else {
            Err(RuntimeError::AnyError(format!(
                "Label '{}' not found",
                label
            )))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    instruction: Instruction,
    line_number: LineNumber,
}

impl Line {
    pub fn new(instruction: Instruction, line_number: LineNumber) -> Self {
        Self {
            instruction,
            line_number,
        }
    }

    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }

    pub fn line_number(&self) -> LineNumber {
        self.line_number
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Load(Operand),
    Store(Address),
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
    Goto(Label),
    Jzero(Label),
    Jnzero(Label),
    End,
}

#[derive(Clone, Debug)]
pub enum Operand {
    Immediate(Value),
    DirectAddress(Register),
    IndirectAddress(Register),
}

#[derive(Clone, Debug)]
pub enum Address {
    Direct(Register),
    Indirect(Register),
}

pub type Label = String;

pub type LineNumber = usize;
