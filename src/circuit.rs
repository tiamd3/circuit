use std::fmt::Debug;
use crate::gate::{GateType, LogicGate};
use crate::component::LogicComponent;
use crate::signal::{ BinarySignal };
#[derive(Debug)]
pub struct Circuit<S>
{
    signal_map: Vec<S>,
    input: usize,
    gates: Vec<LogicGate>,
    components: Vec<Box<dyn LogicComponent<S>>>,
    next_valid_id: usize,
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
            gates: Vec::new(),
            components: Vec::new(),
            next_valid_id: input,
        }
    }

    pub fn get_signal_size(&self) -> usize {
        self.signal_map.len()
    }

    pub fn add_gate(&mut self, gate_type: GateType, input: &[usize]) -> usize {
        let new_gate = LogicGate::new(gate_type, input, &[self.next_valid_id]);
        self.next_valid_id += 1;
        self.gates.push(new_gate);
        self.signal_map.push(S::default());
        self.next_valid_id - 1
    }
    
    pub fn add_gate_with_output(
        &mut self, gate_type: GateType, input: &[usize], output: usize) {
        let new_gate = LogicGate::new(gate_type, input, &[output]);
        self.gates.push(new_gate);
        self.signal_map.push(S::default());
    }
    
    pub fn init_signal(&mut self, signals: &[(usize, bool)]) {
        for (index, signal) in signals {
            self.signal_map[*index] = S::from_bool(*signal);
        }
    }
    
    pub fn advance_output(&mut self, n: usize) -> Vec<usize> {
        let mut outputs = Vec::new();
        for i in 0..n {
            outputs.push(i + self.next_valid_id);
        }
        self.next_valid_id += n;
        outputs
    }

    pub fn execute_gates(&mut self, input: &[S]) {
        input.iter()
            .enumerate()
            .for_each(|(i, signal)| self.signal_map[i] = signal.clone());
        self.gates
            .iter()
            .for_each(|gate| { gate.execute(&mut self.signal_map)});
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
                input_vec[self.input - j - 1] = S::from_bool(v);
            }
            self.execute_gates(&input_vec);
            f(self, indexes);
        }
    }
    pub fn get_signal(&self, index: usize) -> &S {
        &self.signal_map[index]
    }

    pub fn print_output(&self, indexes: &[usize]) {
        let outputs = indexes.iter()
            .map(|&index| self.get_signal(index))
            .collect::<Vec<_>>();
        println!("{:?}", outputs);
    }
}
