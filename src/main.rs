
#[derive(Debug)]
pub struct Circuit {
    signal_map: Vec<Signal>,
    input_indexes: Vec<usize>,
    output_indexes: Vec<usize>,
    circuit_graph: Vec<WiredGate>,
    next_valid_id: usize,
}

impl Circuit {
    pub fn new(input: usize, output: usize) -> Circuit {
        let signal_map = vec![Signal(None); (input + output) * 2];
        let mut input_indexes = vec![0; input];
        let mut output_indexes = vec![0; output];
        let mut next_valid_id = 0;
        for i in 0..input { input_indexes[i] = i; }
        next_valid_id += input * 2;
        for i in 0..output { output_indexes[i] = i + next_valid_id; }
        next_valid_id += output * 2;
        next_valid_id += 1;
        Self {
            signal_map,
            input_indexes,
            output_indexes,
            circuit_graph: Vec::new(),
            next_valid_id,
        }
    }
    
    pub fn add_gate(&mut self, mut gate: WiredGate) {
        gate.set_output(self.next_valid_id);
        self.next_valid_id += 1;
        self.circuit_graph.push(gate);
    }

    pub fn add_gate_to_output(&mut self, mut gate: WiredGate) {
        let output_index = self.output_indexes[gate.get_output()];
        gate.set_output(output_index);
        self.circuit_graph.push(gate);
    }
    
    pub fn get_input(&self, index: usize) -> &Signal {
        &self.signal_map[self.input_indexes[index]]
    }
    
    pub fn execute(&mut self, input: &[Signal]) -> Vec<Signal> {
        self.input_indexes
            .iter()
            .for_each(|i| self.signal_map[*i] = input[*i]);
        self.circuit_graph
            .iter()
            .for_each(|gate| { gate.execute(&mut self.signal_map)});
        self.output_indexes
            .iter()
            .map(|i| self.signal_map[*i]).collect::<Vec<Signal>>()
    }    
    
    pub fn get_output(&self, index: usize) -> &Signal {
        &self.signal_map[self.output_indexes[index]]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Signal(Option<bool>);

#[derive(Clone, Copy, Debug)]
pub enum WiredGate {
    Not([usize; 1], usize),
}

impl WiredGate {
    pub fn execute(&self, ctx: &mut [Signal]) {
        use WiredGate::*;
        match self {
            Not(input, output)=> {
                ctx[*output].0 = ctx[input[0]].0.map(|x| !x);
            }
        }
    }
    
    pub fn set_output(&mut self, output: usize) { 
        use WiredGate::*;
        match self {
            Not(_, o) => { *o = output; }
        }
    }

    pub fn get_output(&self) -> usize {
        use WiredGate::*;
        match self {
            Not(_, o) => *o,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut circuit = Circuit::new(1, 1);
        circuit.add_gate_to_output(WiredGate::Not([0], 0));
        let output = circuit.execute(&[Signal(Some(true))]);
        println!("{:#?}", circuit);
        println!("{:?}", output);
    }
}































