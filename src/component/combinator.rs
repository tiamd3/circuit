use crate::circuit::Circuit;
use super::info::ComponentInfo;
use crate::gate::{GateType, LogicGate};
use crate::signal::BinarySignal;
use super::LogicComponent;

#[derive(Copy, Clone, Debug)]
pub enum ComponentType {
    HalfAdder([usize; 2]),
    BasicRSFF { r: usize, s: usize },
    RSFF { r: usize, s: usize, c: usize },
}

pub struct BasicCombinator {
    typ: ComponentType,
    outputs: Vec<usize>,
    gates: Vec<LogicGate>,
}

impl BasicCombinator {
    fn new(typ: ComponentType, output: &[usize]) -> Self {
        Self {
            typ,
            outputs: Vec::new(),
            gates: Vec::new(),
        }
    }

    fn get_type(&self) -> &ComponentType { &self.typ }
    fn get_outputs(&self) -> &Vec<usize> { &self.outputs }
    pub fn build<S: BinarySignal>(typ: &ComponentType, circuit: &mut Circuit<S>) -> Self {
        match typ {
            ComponentType::HalfAdder(input) => {
                let output = &circuit.advance_output(2);
                let mut cor = Self::new(typ.clone(), output);
                cor.gates.push(LogicGate::new(GateType::Xor(2), input, &output));
                cor.gates.push(LogicGate::new(GateType::And(2), input, &output));
                cor
            }
            ComponentType::BasicRSFF { r, s} => {
                let output = &circuit.advance_output(2);
                let mut cor = Self::new(typ.clone(), output);
                cor.gates.push(LogicGate::new(
                    GateType::NAnd(2),
                    &[*r, output[1]],
                    &[output[0]],
                ));
                cor.gates.push(LogicGate::new(
                    GateType::NAnd(2),
                    &[*s, output[0]],
                    &[output[1]],
                ));
                circuit.set_signals(&[
                    (output[0], S::from_bool(false)),
                    (output[1], S::from_bool(true)),
                ]);
                cor
            }
            ComponentType::RSFF { r, s, c} => {
                let output = &circuit.advance_output(4);
                let mut cor = Self::new(
                    typ.clone(), &[output[0], ]);
                cor.gates.push(LogicGate::new(
                    GateType::NAnd(2),
                    &[*r, *c],
                    &[output[0]],
                ));
                cor.gates.push(LogicGate::new(
                    GateType::NAnd(2),
                    &[*s, *c],
                    &[output[1]],
                ));
                circuit.set_signals(&[
                    (output[0], S::from_bool(false)),
                    (output[1], S::from_bool(true)),
                ]);
                cor
            }
        }
    }
    pub fn execute<S: BinarySignal>(&self, circuit: &mut Circuit<S>) {
        match self.typ {
            ComponentType::HalfAdder(_) => {
                self.gates.iter().for_each(|gate| {
                    gate.execute(circuit.get_signal_map_mut());
                });
            }
            ComponentType::BasicRSFF { r, s } => {
                let mut res = Vec::new();
                self.gates.iter().for_each(|gate| {
                    res.push(gate.process(circuit.get_signal_map()));
                });
                circuit.set_signals(&res);
            }
            ComponentType::RSFF { r, s, c } => {

            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::c_uint;
    use crate::signal::Signal;
    use super::*;

    #[test]
    fn basic_combinator() {
        let mut circuit: Circuit<Signal> = Circuit::new_with_input(2);
        let rs = BasicCombinator::build(
            &ComponentType::BasicRSFF { r: 0, s: 1},
            &mut circuit,
        );
        println!("{:?}", circuit.get_signal_map());
        circuit.set_signals(&[
            (0, Signal::from_bool(true)),
            (1, Signal::from_bool(true)),
        ]);
        rs.execute(&mut circuit);
        println!("{:?}", circuit.get_signal_map());
    }
}