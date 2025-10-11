use crate::circuit::Circuit;
use crate::gate::{ GateType, LogicGate };
use crate::signal::BinarySignal;

trait LogicComponent {
    fn build<S: BinarySignal>(&self, input: &[usize], circuit: &mut Circuit<S>) -> Vec<usize>;
}

pub struct HalfAdder;

impl LogicComponent for HalfAdder {
    fn build<S>(&self, input: &[usize], circuit: &mut Circuit<S>) -> Vec<usize>
    where
        S: BinarySignal,
    {
        let mut output = vec![0; 2];
        output[0] = circuit.add_gate(GateType::Xor, input);
        output[1] = circuit.add_gate(GateType::And, input);
        output
    }
}