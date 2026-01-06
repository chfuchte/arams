use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Machine {
    pub registers: HashMap<usize, u64>,
    pub accumulator: u64,
}
