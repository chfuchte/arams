use arams_core::{IntoSourceCode, compile, execute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Machine {
    pub registers: HashMap<usize, u64>,
    pub accumulator: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CompilationError {
    pub line: usize,
    pub message: String,
}

impl From<arams_core::CompileError> for CompilationError {
    fn from(err: arams_core::CompileError) -> Self {
        CompilationError {
            line: err.line_number(),
            message: err.to_string(),
        }
    }
}

#[wasm_bindgen]
pub fn check(input: JsValue) -> Result<JsValue, JsError> {
    let source_code = if input.is_string() {
        serde_wasm_bindgen::from_value::<String>(input)?.into_lines()
    } else if input.is_array() {
        serde_wasm_bindgen::from_value(input)?
    } else {
        return Err(JsError::new(
            "Input must be a string or an array of strings",
        ));
    };

    match compile(source_code) {
        Ok(_) => Ok(JsValue::TRUE),
        Err(errors) => {
            let compilation_errors: Vec<CompilationError> =
                errors.into_iter().map(CompilationError::from).collect();
            Ok(serde_wasm_bindgen::to_value(&compilation_errors)?)
        }
    }
}

#[wasm_bindgen]
pub fn run(input: JsValue, registers: JsValue) -> Result<JsValue, JsError> {
    let source_code = if input.is_string() {
        serde_wasm_bindgen::from_value::<String>(input)?.into_lines()
    } else if input.is_array() {
        serde_wasm_bindgen::from_value(input)?
    } else {
        return Err(JsError::new(
            "Input must be a string or an array of strings",
        ));
    };

    let registers: HashMap<usize, u64> = serde_wasm_bindgen::from_value(registers)?;

    match compile(source_code) {
        Ok(program) => match execute(program, Some(registers)) {
            Ok(machine) => {
                let result = Machine {
                    registers: machine.get_registers().clone(),
                    accumulator: machine.get_accumulator(),
                };
                Ok(serde_wasm_bindgen::to_value(&result)?)
            }
            Err(execution_error) => Err(JsError::new(&execution_error.to_string())),
        },
        Err(errors) => {
            let compilation_errors: Vec<CompilationError> =
                errors.into_iter().map(CompilationError::from).collect();
            Ok(serde_wasm_bindgen::to_value(&compilation_errors)?)
        }
    }
}
