use serde::{Deserialize, Serialize};
use crate::signal::BinarySignal;

#[derive(Debug, Copy, Hash, Clone, Serialize, Deserialize)]
pub enum GateType {
    Not,
    And,
    Or,
    Xor,
    NAnd,
}

impl GateType {
    pub fn execute<S: BinarySignal>(&self, input: &[S]) -> S {
        match self {
            GateType::Not => input[0].not(),
            GateType::And => {
                input.iter().fold(S::from_bool(Some(true)), |acc, v| acc.and(v))
            }
            GateType::Or => {
                input.iter().fold(S::from_bool(Some(false)), |acc, v| acc.or(v))
            }
            GateType::Xor => {
                input[0].xor(&input[1])
            }
            GateType::NAnd => {
                input.iter().fold(S::from_bool(Some(true)), |acc, v| acc.and(v)).not()
            }
        }
    }
}
