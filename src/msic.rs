use crate::gate::GateType;
use crate::signal::Signal;

pub fn gain_gate_type(gate_type: &str, input_size: usize) -> GateType {
    match (gate_type, input_size) {
        ("and", is) => GateType::And(is),
        ("or", is) => GateType::Or(is),
        ("xor", 2) => GateType::Xor(2),
        ("not", 1) => GateType::Not(1),
        ("nand", is) => GateType::NAnd(is),
        _ => unreachable!()
    }
}

pub fn usize_signal_vec(usize_vec: &[usize]) -> Vec<Signal> {
    usize_vec.iter().map(|v| Signal::from_usize(*v)).collect()
}

pub fn choose_signals(signals: &[Signal], indexes: &[usize]) -> Vec<Signal> {
    indexes.iter().map(|&i| signals[i].clone()).collect()
}

pub fn print_output(signals: &[Signal], indexes: &[usize]) {
    let outputs = choose_signals(signals, indexes);
    println!("{:?}", outputs);
}