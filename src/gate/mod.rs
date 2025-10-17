pub mod basic;

#[derive(Debug, Copy, Clone)]
pub enum GateType {
    Not(usize),
    And(usize),
    Or(usize),
    Xor(usize),
    NAnd(usize),
}

#[derive(Debug)]
pub enum LogicGate {
    Not1([usize; 1], [usize; 1]),
    And2([usize; 2], [usize; 1]),
    Or2([usize; 2], [usize; 1]),
    Xor2([usize; 2], [usize; 1]),
    NAnd2([usize; 2], [usize; 1]),
    NAnd3([usize; 3], [usize; 1]),
}
