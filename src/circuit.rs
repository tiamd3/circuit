use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::gate::GateType;
use crate::component::LogicComponent;
use crate::signal::{BinarySignal, Signal};

pub enum BuildError {
    SampleOutput(usize, usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Gate(GateType, Vec<usize>, usize),
    InBuild,
    Custom(String, Vec<usize>, usize),
}

impl Node {
    pub fn execute<S: BinarySignal>(&self, signals: &[S]) -> S {
        match self {
            Node::Gate(gate_type, input, output) => {
                gate_type.execute(signals)
            }
            _ => todo!()
        }
    }

    pub fn execute_mut<S: BinarySignal>(&self, signals: &mut [S]) {
        match self {
            Node::Gate(gate_type, input, output) => {
                let input = Circuit::choose_signals(signals, input);
                signals[*output] = gate_type.execute(&input);
            }
            _ => todo!()
        }
    }

    pub fn get_output(&self) -> Vec<usize> {
        match self {
            Node::Gate(.., output) => vec![*output],
            _ => todo!()
        }
    }

    pub fn get_input(&self) -> &[usize] {
        match self {
            Node::Gate(_, input, _) => input,
            _ => todo!()
        }
    }

    pub fn set_output<S: BinarySignal>(&mut self, signals: &mut [S], output: &[S]) {
        match self {
            Node::Gate(.., o) => {
                signals[*o] = output[0].clone();
            }
            _ => todo!()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CombiPattern {
    input: usize,
    output: Vec<usize>,
    pipeline: Vec<Node>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Circuit<S>
{
    signal_map: Vec<S>,
    input: usize,

    patterns: HashMap<String, CombiPattern>,
    //inbuilds: Vec<BasicCombinator>,
    //customs: Vec<Combinator>,
    pipeline: Vec<Node>,
}

impl<S> Circuit<S> 
where 
    S: BinarySignal
{
    pub fn new() -> Circuit<S> {
        Self::new_with_input(0)
    }

    pub fn new_with_input(input: usize) -> Circuit<S>
    {
        let signal_map = vec![S::default(); input];
        Self {
            signal_map,
            input,
            //customs: Vec::new(),
            //inbuilds: Vec::new(),
            patterns: HashMap::new(),
            pipeline: Vec::new(),
        }
    }

    pub fn get_input(&self) -> usize { self.input }

    pub fn get_signals_mut(&mut self) -> &mut Vec<S> { &mut self.signal_map }
    pub fn get_signals(&self) -> &Vec<S> { &self.signal_map }

    pub fn build_combi(&self, gates_vec: &[usize]) {
        let gates_vec = gates_vec.iter()
            .map(|i| self.pipeline[*i].clone())
            .collect::<Vec<Node>>();

    }

    pub fn choose_signals(signals: &[S], indexes: &[usize]) -> Vec<S> {
        indexes.iter().map(move |&i| signals[i].clone()).collect::<Vec<S>>()
    }

    pub fn get_a_signal(&self, index: usize) -> S { self.signal_map[index].clone() }

    pub fn signal_size(&self) -> usize { self.signal_map.len() }
    
    pub fn clone_signals(&self, indexes: &[usize]) -> Vec<S> { 
        indexes.iter().map(move |&i| self.signal_map[i].clone()).collect::<Vec<S>>()
    }

    pub fn set_input(&mut self, input: &[usize]) {
        for (i, u) in input.iter().enumerate() {
            self.signal_map[i] = S::from_usize(*u);
        }
    }

    fn gain_gate_type(gate_type: &str, input_size: usize) -> GateType {
        match (gate_type, input_size) {
            ("and", _) => GateType::And,
            ("or", _) => GateType::Or,
            ("xor", 2) => GateType::Xor,
            ("not", 1) => GateType::Not,
            ("nand", _) => GateType::NAnd,
            _ => unreachable!()
        }
    }
    pub fn add_gate(&mut self, gate_type: &str, input: &[usize]) -> (usize, usize) {
        let gate_type = Self::gain_gate_type(gate_type, input.len());
        let new_gate = Node::Gate(gate_type, input.to_vec(), self.signal_size());
        self.pipeline.push(new_gate);
        let node_index = self.pipeline.len() - 1;
        self.signal_map.push(S::with_parent(None, node_index));
        (node_index, self.signal_size() - 1)
    }

    pub fn add_gate_exact(&mut self, gate_type: &str, input: &[usize], output: usize) -> usize {
        let gate_type = Self::gain_gate_type(gate_type, input.len());
        let new_gate = Node::Gate(gate_type, input.to_vec(), output);
        self.pipeline.push(new_gate);
        let node_index = self.pipeline.len() - 1;
        self.signal_map.push(S::with_parent(None, node_index));
        node_index
    }
    pub fn pop(&mut self) -> Option<Node> {
        self.pipeline.pop()
    }
    pub fn advance_output(&mut self, signals: &[usize]) -> Vec<usize> {
        let mut outputs = Vec::new();
        for i in 0..signals.len() {
            self.signal_map.push(S::from_usize(signals[i]));
            outputs.push(i + self.signal_size());
        }
        outputs
    }

    pub fn execute_gates(&mut self) {
        for node in &self.pipeline {
            let res = node.execute(&Circuit::choose_signals(
                self.get_signals(),
                node.get_input()
            ));
            self.signal_map[node.get_output()[0]] = res;
        }
    }

    pub fn execute_from_output(&self) {

    }
    pub fn execute_gates_exact(&mut self, gates: &[usize]) -> Vec<S> {
        let mut results = self.signal_map.clone();

        let mut gates_vec = gates.iter()
            .map(|i| (false, self.pipeline[*i].clone()))
            .collect::<Vec<(bool, Node)>>();
        for (flag, node) in gates_vec.iter_mut() {
            let input = &Circuit::choose_signals(
                &results,
                node.get_input()
            );
            if input.contains(&S::from_usize(0)) {
                continue;
            } else {
                node.execute_mut(&mut results);
                *flag = true;
            }
        }

        results
    }

    pub fn for_every_input(&mut self, indexes: &[usize], f: fn(&Circuit<S>, &[usize])) {
        let mut input_vec = vec![S::default(); self.input];
        let n = 2u32.pow(self.input as u32);
        //print!("{}", n);
        for i in 0..n {
            let mut temp = i;
            for j in 0..self.input {
                let v= temp % 2 == 1;
                temp /= 2;
                input_vec[self.input - j - 1] = S::from_bool(Some(v));
            }
            self.execute_gates();
            f(self, indexes);
        }
    }
    

    pub fn print_output(&self, indexes: &[usize]) {
        let outputs = Self::choose_signals(self.get_signals(), indexes);
        println!("{:?}", outputs);
    }
}

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    use super::*;

    #[test]
    fn test_circuit() {
        let mut circuit: Circuit<Signal> = Circuit::new_with_input(3);
        let and1 = circuit.add_gate("and", &[0, 1]);
        let rs_q2 = circuit.advance_output(&[1, 0]);
        let (q, nq) = (rs_q2[0], rs_q2[1]);
        circuit.add_gate_exact("nand", &[0, q], nq);
        circuit.add_gate_exact("nand", &[1, nq], q);
        circuit.set_input(&[0, 1]);
        println!("{:?}", circuit.get_signals());
    }
}