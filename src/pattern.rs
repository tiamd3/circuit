use serde::{Deserialize, Serialize};
use std::fmt;
use std::collections::HashMap;
use crate::circuit::Circuit;
use crate::msic::gain_gate_type;
use crate::node::{Node, NodeType};
use crate::signal::Signal;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
    input_size: usize,
    output: Vec<(usize, usize)>,
    pattern: Vec<PatternNode>,
    
    port_name: (Vec<String>, Vec<String>),
    description: String,
}

pub fn add_node(pattern: &mut Vec<PatternNode>, node_type: &str, node_inputs: Vec<PniType>) -> usize {
    pattern.push(PatternNode::new(
        gain_gate_type(node_type),
        node_inputs,
    ));
    pattern.len() - 1
}

impl Pattern {

    pub fn new(input_size: usize, output: Vec<(usize, usize)>, pattern: Vec<PatternNode>) -> Pattern {
        Self { 
            input_size, 
            output, 
            pattern, 
            port_name: (Vec::new(), Vec::new()),
            description: "".to_string(),
        }
    }
    
    pub fn get_pattern(&self) -> &Vec<PatternNode> { &self.pattern }

    pub fn get_pattern_mut(&mut self) -> &mut Vec<PatternNode> { &mut self.pattern }

    pub fn get_output(&self) -> &Vec<(usize, usize)> { &self.output }
    
    pub fn input_size(&self) -> usize { self.input_size }

    pub fn output_size(&self) -> usize { self.output.len() }

    pub fn gain_inputs_outputs(&self, signals: &[(&str, usize)]) -> (Vec<usize>, Vec<usize>) {
        let mut outputs = vec![0; self.output.len()];
        let mut inputs = vec![0; self.input_size];
        let port_names = signals.iter()
            .map(|(name, index)| (name.to_string(), *index))
            .collect::<HashMap<String, usize>>();
        for (i, input) in self.get_input_name().iter().enumerate() {
            inputs[i] = port_names[input];
        }
        for (i, output) in self.get_output_name().iter().enumerate() {
            outputs[i] = port_names[output];
        }
        (inputs, outputs)
    }
    pub fn set_name(
        &mut self, 
        input_name: &[&str], 
        output_name: &[&str],
    ) -> &mut Pattern {
        let input_name = input_name.iter().map(|&s| s.to_string()).collect();
        let output_name = output_name.iter()
            .map(|s| s.to_string()).collect();
        self.port_name = (input_name, output_name);
        self
    }
    
    pub fn get_input_name(&self) -> &Vec<String> { &self.port_name.0 }
    pub fn get_output_name(&self) -> &Vec<String> { &self.port_name.1 }
    
    pub fn set_description(&mut self, d: &str) -> &mut Pattern {
        self.description = d.to_string();
        self
    }

    pub fn print_information(&self) {
        print!("input: {:?}", self.get_input_name());
        print!("output: {:?}\n", self.get_output_name());
        println!("function: {}", self.description);
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum PniType {
    Input(usize),
    NodeOutput((usize, usize)),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatternNode {
    gate_type: NodeType,
    inputs: Vec<PniType>,
}

impl PatternNode {
    pub fn new(gate_type: NodeType, inputs: Vec<PniType>) -> Self {
        Self { gate_type, inputs }
    }
    pub fn execute(
        &self,
        input_signals: &[Signal],
        output_signals: &[&[Signal]],
    ) -> Signal {
        let input_signals = self.inputs.iter().map(|i| {
            match i {
                PniType::Input(i) => input_signals[*i],
                PniType::NodeOutput((node, output_index)) =>  {
                    output_signals[*node][*output_index]
                },
            }
        }).collect::<Vec<Signal>>();
        
        Node::execute_gate(self.get_type(), &input_signals)
    }
    
    pub fn get_type(&self) -> &NodeType { &self.gate_type }
    
    pub fn get_input(&self) -> &Vec<PniType> { &self.inputs }
}

impl fmt::Debug for PniType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(v) => write!(f, "I({})", v),
            Self::NodeOutput(v) => write!(f, "N({}, {})", v.0, v.1),
        }
    }
}
impl fmt::Display for PatternNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.gate_type, self.inputs)
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "input size: {}, output node: {:?}\n", self.input_size, self.output)?;
        for (i, node) in self.pattern.iter().enumerate() {
            write!(f, "{} => N({})", node, i)?;
            if self.output.iter().any(|(x, _)| i == *x) {
                write!(f, "*")?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::msic::{print_signals, usize_signal_vec};
    use super::*;

    #[test]
    fn test_flipflop() {
        let mut circuit = Circuit::new_with_basic_pattern(3);
        let input = circuit.get_input();

        let rs_output = circuit.apply_flipflop(
            "rsff",
            &input,
        );

        println!("{:?}", circuit.get_pipeline());
        println!("{:?}", circuit.get_signals());

        circuit.execute_sequential_mut(&[0, 1, 1]);
        println!("{:?}", circuit.get_signals());
        //circuit.get_pattern("rsff").print_information();
    }
}