use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use eframe::egui::UserAttentionType;
use serde::{Deserialize, Serialize};
use crate::signal::Signal;
use crate::node::{Node, NodeType};
use crate::pattern::{Pattern, PatternGate, PniType};
use crate::msic::*;

pub enum BuildError {
    SampleOutput(usize, usize),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Circuit
{
    signals: Vec<Signal>,
    input: usize,

    pattern_range: (usize, usize),
    patterns: HashMap<String, Pattern>,
    pipeline: Vec<Node>,
}

impl Circuit {
    pub fn new(input: usize) -> Circuit
    {
        Self {
            signals: vec![Signal::default(); input],
            input,
            pattern_range: (0, 0),
            patterns: HashMap::new(),
            pipeline: Vec::new(),
        }
    }
    
    pub fn new_with_basic_pattern(input: usize) -> Circuit {
        let mut circuit = Self::new(input);
        Pattern::build_basic_rsff(&mut circuit);
        Pattern::build_rsff(&mut circuit);
        circuit
    }

    pub fn get_input(&self) -> Vec<usize> {
        let mut res = vec![0; self.input];
        for i in 0..self.input {
            res[i] = i;
        }
        res
    }
    pub fn get_patterns_mut(&mut self) -> &mut HashMap<String, Pattern> { &mut self.patterns }
    pub fn get_patterns(&self) -> &HashMap<String, Pattern> { &self.patterns }
    pub fn get_signals_mut(&mut self) -> &mut Vec<Signal> { &mut self.signals }
    pub fn get_signals(&self) -> &Vec<Signal> { &self.signals }
    pub fn get_signal(&self, index: usize) -> Signal { self.signals[index].clone() }
    pub fn signals_size(&self) -> usize { self.signals.len() }

    pub fn get_node(&self, index: usize) -> &Node { &self.pipeline[index] }
    pub fn get_pipeline(&self) -> &Vec<Node> { &self.pipeline }

    pub fn get_pattern(&self, name: &str) -> &Pattern { &self.patterns[name] }

    pub fn get_parent_node_index(&self, signal: usize) -> Option<usize> {
        if let Some(signal) = self.signals.get(signal) {
            if let Some(node) = signal.get_parent() {
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn get_parent_node(&self, signal: usize) -> Option<&Node> {
        self.get_parent_node_index(signal).map(|i| &self.pipeline[i])
    }

    pub fn get_nodes_vec(&self, from: usize, to: usize) -> Vec<&Node> {
        let mut res = Vec::new();
        for i in from..to {
            res.push(&self.pipeline[i]);
        }
        res
    }

    pub fn apply_pattern_named(
        &mut self,
        pattern_name: &str,
        signals: &[(&str, usize)]) -> &Node {
        let pattern = self.get_pattern(pattern_name);
        let (inputs, outputs) = pattern.gain_inputs_outputs(signals);
        self.pipeline.push(Node::new(
            NodeType::Pattern(pattern_name.to_string()),
            inputs,
            outputs.clone(),
        ));
        &self.pipeline.last().unwrap()
    }
    pub fn apply_pattern(&mut self, pattern_name: &str, inputs: &[usize]) -> &Node {
        let output_size = self.get_pattern(pattern_name).output_size();
        let outputs = self.advance_output(output_size);
        self.pipeline.push(Node::new(
            NodeType::Pattern(pattern_name.to_string()),
            inputs.to_vec(),
            outputs.clone()));
        &self.pipeline.last().unwrap()
    }

    pub fn apply_flipflop(&mut self, pattern_name: &str, inputs: &[usize]) -> &Node {
        let outputs = self.advance_output_for_flipflop();
        self.pipeline.push(Node::new(
            NodeType::Pattern(pattern_name.to_string()),
            inputs.to_vec(),
            outputs.clone()
        ));
        &self.pipeline.last().unwrap()
    }

    pub fn build_begin(&mut self) { 
        self.pattern_range.0 = self.pipeline.len();
        self.pattern_range.1 = self.signals.len();
    }
    
    pub fn build_end(
        &mut self,
        pattern_name: &str,
        inputs: &[usize],
        outputs: &[usize]) -> &mut Pattern
    {
        let mut signal_start = self.pattern_range.1;
        let mut pattern_start = self.pattern_range.0;
        let mut nodes = self.pipeline[pattern_start..].to_vec();
        let mut pattern_nodes = Vec::new();
        
        for node in nodes.iter_mut() {
            let mut node_inputs = Vec::new();
            for input in node.get_input() {
                if let Some(outside_input) = inputs.iter().position(|x| x == input) {
                    node_inputs.push(PniType::Input(outside_input));
                } else {
                    node_inputs.push(PniType::NodeOutput(
                        self.get_parent_node_index(*input).unwrap() - pattern_start
                    ))
                }
            }
            pattern_nodes.push(PatternGate::new(
                node.get_type().clone(),
                node_inputs,
            ));
        }

        let output_node = outputs.iter()
            .map(|x| self.get_parent_node_index(*x).unwrap() - pattern_start)
            .collect();

        let new_pattern = Pattern::new(
            inputs.len(),
            output_node,
            pattern_nodes,
        );

        while self.signals.len() != signal_start { self.signals.pop(); }
        while self.pipeline.len() != pattern_start { self.pipeline.pop(); }

        self.patterns.insert(pattern_name.to_string(), new_pattern);
        self.patterns.get_mut(pattern_name).unwrap()
    }

    pub fn add_gate_t(&mut self, gate_type: NodeType, input: &[usize]) -> usize {
        let new_gate = Node::new(
            gate_type,
            input.to_vec(),
            vec![self.signals_size()]
        );

        self.pipeline.push(new_gate);
        let node_index = self.pipeline.len() - 1;
        self.signals.push(Signal::with_parent(None, node_index));
        self.signals_size() - 1
    }
    
    pub fn add_gate(&mut self, gate_type: &str, input: &[usize]) -> usize {
        let gate_type = gain_gate_type(gate_type);
        self.add_gate_t(gate_type, input)
    }

    pub fn advance_output(&mut self, size: usize) -> Vec<usize> {
        let mut output = Vec::new();
        for i in 0..size {
            self.signals.push(Signal::default());
            output.push(self.signals_size() - 1);
        }
        output
    }

    pub fn advance_output_for_flipflop(&mut self) -> Vec<usize> {
        self.advance_output_with_value(&[0, 1])
    }
    pub fn advance_output_with_value(&mut self, signals: &[usize]) -> Vec<usize> {
        let mut outputs = Vec::new();
        for i in 0..signals.len() {
            self.signals.push(Signal::from_usize(signals[i]));
            outputs.push(self.signals_size() - 1);
        }
        outputs
    }

    pub fn init_input(&mut self, input: &[usize]) {
        for (i, u) in input.iter().enumerate() {
            self.signals[i] = Signal::from_usize(*u);
        }
    }

    pub fn execute_sequential_mut(&mut self, input_signals: &[usize]) {
        self.signals = self.execute_sequential(input_signals);
    }
    pub fn execute_sequential(&self, input_signals: &[usize]) -> Vec<Signal> {
        let input_signals = usize_signal_vec(input_signals);
        let mut results = self.signals.clone();
        input_signals.iter()
            .enumerate()
            .for_each(|(i,v)| results[i] = *v);

        for node in &self.pipeline {
            node.execute_mut(self, &mut results);
        }
        results
    }

    pub fn truth_table(
        circuit: &Circuit,
        input_size: usize,
        choose_outputs: &[usize],
    ) -> Vec<(Vec<Signal>, Vec<Signal>)> {
        let n = 2u32.pow(input_size as u32) as usize;
        let mut result = vec![(vec![], vec![]); n];

        for i in 0..n {
            let mut temp = i;
            let mut line_input = vec![0usize; input_size];
            for j in 0..input_size {
                let v= temp % 2;
                temp /= 2;
                line_input[input_size - j - 1] = v;
            }
            result[i].0 = usize_signal_vec(&line_input);
            let res = circuit.execute_sequential(&line_input);
            result[i].1 = choose_outputs.iter()
                .map(|&x| res.get(x).unwrap().clone()).collect()
        }
        result
    }

    pub fn print_truth_table(truth_table: &Vec<(Vec<Signal>, Vec<Signal>)>) {
        for line in truth_table {
            for value in line.0.iter() {
                print!("{:?} ", value);
            }
            print!(": ");
            for value in line.1.iter() {
                print!("{:?} ", value);
            }
            println!();
        }
    }

}

#[cfg(test)]
mod tests {
    use eframe::egui::debug_text::print;
    use crate::signal::Signal;
    use super::*;

    #[test]
    fn test_truth_table() {
        let mut circuit = Circuit::new(3);
        let input = circuit.get_input();
        let and_output = circuit.add_gate(
            "and",
            &input[0..2]);
        let or_output = circuit.add_gate(
            "or",
            &input[0..2]
        );
        let truth_table = Circuit::truth_table(
            &circuit,
            circuit.get_input().len(), 
            &[and_output, or_output],);
        Circuit::print_truth_table(&truth_table);
    }

    #[test]
    fn half_adder() {
        let mut circuit = Circuit::new(2);
        let input = circuit.get_input();
        let go1 = circuit.add_gate(
            "and",
            &[input[0], input[1]]);
        let go2 = circuit.add_gate(
            "xor",
            &[input[0], input[1]]);

        let res = circuit.execute_sequential(&[1, 1]);
        let output = [go1, go2];
        print_signals(&res, &output);
    }

    #[test]
    fn full_adder() {
        let mut circuit = Circuit::new(3);
        let input = circuit.get_input();
        let (a, b, c0) = (input[0], input[1], input[2]);

        circuit.build_begin();

        let go1 = circuit.add_gate(
            "xor",
            &[a, b]
        );
        let s = circuit.add_gate(
            "xor",
            &[c0, go1]
        );

        let go2 = circuit.add_gate(
            "and",
            &[a, b]
        );
        let go3 = circuit.add_gate(
            "and",
            &[c0, go1]
        );
        let c1 = circuit.add_gate(
            "or",
            &[go2, go3]
        );

        let output = [s, c1];

        let fa_pattern = circuit
            .build_end(
            "full_adder",
            &input, &output)
            .set_name(&["a", "b", "c0"], &["s", "c1"])
            .set_description("a + b + c0 -> c1 s");
        
        println!("{}", fa_pattern);

        circuit.apply_pattern("full_adder", &input);

        let res = circuit.execute_sequential(&[1, 1, 0]);

        print_signals(&res, &output);
    }
}