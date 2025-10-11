use crate::gate::{GateType, LogicGate};
use crate::signal::{ BinarySignal };
#[derive(Debug)]
pub struct Circuit<S>
{
    signal_map: Vec<S>,
    input: usize,
    circuit_graph: Vec<LogicGate>,
    next_valid_id: usize,
}

impl<S> Circuit<S> 
where 
    S: BinarySignal
{
    pub fn new(input: usize) -> Circuit<S>
    {
        let signal_map = vec![S::default(); input];
        Self {
            signal_map,
            input,
            circuit_graph: Vec::new(),
            next_valid_id: input,
        }
    }

    pub fn add_gate(&mut self, gate_type: GateType, input: &[usize]) -> usize {
        let new_gate = LogicGate::new(gate_type, input, &[self.next_valid_id]);
        self.next_valid_id += 1;
        self.circuit_graph.push(new_gate);
        self.signal_map.push(S::default());
        self.next_valid_id - 1
    }

    /// *测试*，尝试添加一个异或门
    pub fn add_xor_gate(&mut self, input: &[usize; 2]) -> usize {
        use GateType::*;
        let not_0 = self.add_gate(Not, &[0]);
        let not_1 = self.add_gate(Not, &[1]);
        let and_1 = self.add_gate(And, &[0, not_1]);
        let and_2 = self.add_gate(And, &[1, not_0]);
        self.add_gate(Or, &[and_1, and_2])
    }

    pub fn execute(&mut self, input: &[S]) {
        input.iter()
            .enumerate()
            .for_each(|(i, signal)| self.signal_map[i] = signal.clone());
        self.circuit_graph
            .iter()
            .for_each(|gate| { gate.execute(&mut self.signal_map)});
    }

    pub fn get_signal(&self, index: usize) -> &S {
        &self.signal_map[index]
    }
}
