use arams_core::IntoSourceCode;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::models::{CompilationError, Machine, RunResult, RuntimeError};

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

    match arams_core::compile(source_code) {
        Ok(program) => match arams_core::execute(program, Some(registers)) {
            Ok(machine) => {
                let result = Machine {
                    registers: machine.get_registers().clone(),
                    accumulator: machine.get_accumulator(),
                };
                Ok(serde_wasm_bindgen::to_value(&RunResult::Ok(result))?)
            }
            Err(err) => Ok(serde_wasm_bindgen::to_value(&RunResult::ExecutionError(
                RuntimeError::from(err),
            ))?),
        },
        Err(errors) => {
            let errors: Vec<CompilationError> = errors.into_iter().map(Into::into).collect();

            Ok(serde_wasm_bindgen::to_value(&RunResult::CompilationError(
                errors,
            ))?)
        }
    }
}
