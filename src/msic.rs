use crate::node::NodeType;
use crate::signal::Signal;

pub fn gain_gate_type(node_type: &str) -> NodeType {
    use NodeType::*;
    match node_type {
        "not" => Not,
        "and" => And,
        "or" => Or,
        "xor" => Xor,
        "nand" => NAnd,
        _ => unreachable!()
    }
}

pub fn usize_signal_vec(usize_vec: &[usize]) -> Vec<Signal> {
    usize_vec.iter().map(|v| Signal::from_usize(*v)).collect()
}

pub fn choose_signals(signals: &[Signal], indexes: &[usize]) -> Vec<Signal> {
    indexes.iter().map(|&i| signals[i].clone()).collect()
}

pub fn print_signals(signals: &[Signal], indexes: &[usize]) {
    let outputs = choose_signals(signals, indexes);
    println!("{:?}", outputs);
}

