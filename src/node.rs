use serde::{Deserialize, Serialize};
use crate::gate::{GateType, LogicGate};
use crate::signal::Signal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Gate(LogicGate),
    InBuild(),
    Custom(String, Vec<usize>, usize),
}

impl Node {
    pub fn execute(&self, signals: &[Signal]) -> Signal {
        match self {
            Node::Gate(gate) => {
                gate.execute(signals)
            }
            _ => todo!()
        }
    }

    pub fn execute_mut(&self, signals: &mut [Signal]) {
        match self {
            Node::Gate(gate) => {
                gate.execute_mut(signals);
            }
            _ => todo!()
        }
    }
    
    pub fn get_type(&self) -> GateType {
        match self {
            Node::Gate(gate) => gate.get_type(),
            _ => unreachable!()
        }
    }

    pub fn get_output(&self) -> Vec<usize> {
        match self {
            Node::Gate(gate) => vec![gate.get_output()],
            _ => todo!()
        }
    }
    
    pub fn set_input(&mut self, index: usize, value: usize) {
        match self {
            Node::Gate(gate) => {
                gate.set_input(index, value);
            }
            _ => todo!()
        }
    }

    pub fn get_input(&self) -> &[usize] {
        match self {
            Node::Gate(g) => g.get_input(),
            _ => todo!()
        }
    }
}