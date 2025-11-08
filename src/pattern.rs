use serde::{Deserialize, Serialize};
use crate::gate::GateType;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
    input_size: usize,
    output: Vec<usize>,
    pattern: Vec<PatternNode>,
    port_name: (Vec<String>, HashMap<String, usize>),
    description: String,
}

impl Pattern {
    pub fn new(input_size: usize, output: Vec<usize>, pattern: Vec<PatternNode>) -> Pattern {
        Self { 
            input_size, 
            output, pattern, 
            port_name: (Vec::new(), HashMap::new()),
            description: "".to_string(),
        }
    }

    pub fn set_name(
        &mut self, 
        input_name: &[&str], 
        output_name: &[(&str, usize)],
    ) -> &mut Pattern {
        let input_name = input_name.iter().map(|&s| s.to_string()).collect();
        let output_name = output_name.iter()
            .map(|(s, i)| (s.to_string(), *i)).collect();
        self.port_name = (input_name, output_name);
        self
    }
    
    pub fn set_description(&mut self, d: &str) -> &mut Pattern {
        self.description = d.to_string();
        self
    }
}

#[derive(Deserialize, Serialize)]
pub enum PniType {
    Input(usize),
    NodeOutput(usize),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PatternNode {
    gate_type: GateType,
    input: Vec<PniType>,
}

impl PatternNode {
    pub fn new(gate_type: GateType, input: Vec<PniType>) -> Self {
        Self { gate_type, input }
    }
}

impl fmt::Debug for PniType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(v) => write!(f, "I({})", v),
            Self::NodeOutput(v) => write!(f, "N({})", v),
        }
    }
}
impl fmt::Display for PatternNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {}", self.input, self.gate_type)
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "input size: {}, output node: {:?}\n", self.input_size, self.output)?;
        for node in &self.pattern {
            write!(f, "{}\n", node)?;
        }
        Ok(())
    }
}