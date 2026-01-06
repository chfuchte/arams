use arams_core::IntoSourceCode;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AnalyzeToken {
    pub kind: String,
    pub lexeme: String,
    pub errors: Vec<String>,
    pub about: String,
}

impl From<arams_core::lsp::LSPToken> for AnalyzeToken {
    fn from(token: arams_core::lsp::LSPToken) -> Self {
        Self {
            kind: token.kind().to_string(),
            lexeme: token.lexeme().to_string(),
            errors: token.errors().iter().map(ToString::to_string).collect(),
            about: token.kind().get_about_text(),
        }
    }
}

#[wasm_bindgen]
pub fn analyze(input: JsValue) -> Result<JsValue, JsError> {
    let source_code = if input.is_string() {
        serde_wasm_bindgen::from_value::<String>(input)?.into_lines()
    } else if input.is_array() {
        serde_wasm_bindgen::from_value(input)?
    } else {
        return Err(JsError::new(
            "Input must be a string or an array of strings",
        ));
    };

    let lines = arams_core::lsp::analyze(source_code);

    Ok(serde_wasm_bindgen::to_value(
        &lines
            .iter()
            .map(|line| {
                line.iter()
                    .cloned()
                    .map(AnalyzeToken::from)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )?)
}
