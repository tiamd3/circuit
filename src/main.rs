use std::fmt;

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
        let signal_map = vec![Signal(None); input + output];
        let mut input_indexes = vec![0; input];
        let mut output_indexes = vec![0; output];
        let mut next_valid_id = 0;
        for i in 0..input { input_indexes[i] = i; }
        next_valid_id += input;
        for i in 0..output { output_indexes[i] = i + next_valid_id; }
        next_valid_id += output;
        Self {
            signal_map,
            input_indexes,
            output_indexes,
            circuit_graph: Vec::new(),
            next_valid_id,
        }
    }
    
    pub fn add_gate(&mut self, mut gate: WiredGate) -> usize {
        gate.set_output(self.next_valid_id);
        self.next_valid_id += 1;
        self.circuit_graph.push(gate);
        self.signal_map.push(Signal(None));
        self.next_valid_id - 1
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

#[derive(Clone, Copy)]
pub struct Signal(Option<bool>);

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(true) => write!(f, "1"),
            Some(false) => write!(f, "0"),
            None => write!(f, "_"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WiredGate {
    Not([usize; 1], usize),
    And([usize; 2], usize),
    Or([usize; 2], usize),
}

impl WiredGate {
    pub fn execute(&self, ctx: &mut [Signal]) {
        use WiredGate::*;
        match self {
            Not(input, output)=> {
                ctx[*output].0 = ctx[input[0]].0.map(|x| !x);
            }
            And(input, output) => {
                let l = ctx[input[0]].0.unwrap();
                let r = ctx[input[1]].0.unwrap();
                ctx[*output].0 = Some(l && r)
            }
            Or(input, output) => {
                let l = ctx[input[0]].0.unwrap();
                let r = ctx[input[1]].0.unwrap();
                ctx[*output].0 = Some(l || r);
            }
        }
    }
    
    pub fn set_output(&mut self, output: usize) { 
        use WiredGate::*;
        match self {
            Or(_,o)|Not(_,o)|And(_, o) => *o = output
        }
    }

    pub fn get_output(&self) -> usize {
        use WiredGate::*;
        match self {
            Not(_,o)|Or(_,o)|And(_,o) => *o,
        }
    }
}

fn u32_to_bool_vec(num: u32) -> Vec<bool> {
    //let res_vec = vec![];
    vec![]
}
fn test_circuit(circuit: &mut Circuit, input_num: usize) {
    let mut input_vec = vec![Signal(None); input_num];
    let n = 2u32.pow(input_num as u32);
    //print!("{}", n);
    for i in 0..n {
        let mut temp = i;
        for j in 0..input_num {
            let v= temp % 2 == 1;
            temp /= 2;
            input_vec[input_num - j - 1] = Signal(Some(v));
        }
        let res = circuit.execute(&input_vec);
        println!("i: {:?}, o: {:?}", input_vec, res);
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
        let mut circuit = Circuit::new(2, 1);
        let not_0 = circuit.add_gate(WiredGate::Not([0], 0));
        let not_1 = circuit.add_gate(WiredGate::Not([1], 0));
        let and_1 = circuit.add_gate(WiredGate::And([1, not_0], 0));
        let and_2 = circuit.add_gate(WiredGate::And([0, not_1], 0));
        circuit.add_gate_to_output(WiredGate::Or([and_1, and_2], 0));

        test_circuit(&mut circuit, 2);
        //println!("{:#?}", circuit);
        //println!("{:?}", output);
    }
}































