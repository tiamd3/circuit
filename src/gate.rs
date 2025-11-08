use serde::{Deserialize, Serialize};
use crate::signal::Signal;
use GateType::*;
use LogicGate::*;
use std::fmt;

pub struct GateOutput {
    pub gate_id: usize,
    pub output: usize,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GateType {
    Not(usize),
    And(usize),
    Or(usize),
    Xor(usize),
    NAnd(usize),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum LogicGate {
    Not1([usize; 1], usize),
    And2([usize; 2], usize),
    Or2([usize; 2], usize),
    Xor2([usize; 2], usize),
    NAnd2([usize; 2], usize),
    NAnd3([usize; 3], usize),
}
impl LogicGate {
    
    pub fn new(typ: GateType, input: &[usize], output: usize) -> Self {
        match typ {
            Not(input_size) => {
                Not1([input[0]], output)
            }
            And(input_size) => {
                And2([input[0], input[1]], output)
            }
            Or(input_size) => {
                Or2([input[0], input[1]], output)
            }
            Xor(_) => {
                Xor2([input[0], input[1]], output)
            }
            NAnd(input_size) => {
                match input_size {
                    2 => NAnd2([input[0], input[1]], output),
                    3 => NAnd3([input[0], input[1], input[2]], output),
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn get_type(&self) -> GateType {
        match self {
            Not1(input,_) => Not(input.len()),
            And2(input,_) => And(input.len()),
            Or2(input,_) => Or(input.len()),
            Xor2(input,_) => Xor(input.len()),
            NAnd2(input, _) => NAnd(input.len()),
            NAnd3(input, _) => NAnd(input.len()),
        }
    }

    pub fn set_input(&mut self, index: usize, value: usize) {
        match self {
            Not1(input,_) => input[index] = value,
            And2(input,_) => input[index] = value,
            Or2(input,_) => input[index] = value,
            Xor2(input,_) => input[index] = value,
            NAnd2(input, _) => input[index] = value,
            NAnd3(input, _) => input[index] = value,
        }
    }

    pub fn get_input(&self) -> &[usize] {
        match self {
            Not1(input, _) => input,
            And2(input, _) | Or2(input, _) | Xor2(input, _) |
            NAnd2(input, _)
            => input,
            NAnd3(input, _) => input,
        }
    }

    pub fn get_output(&self) -> usize {
        match self {
            Not1(_, output) => *output,
            And2(_, output) | Or2(_, output) | Xor2(_, output) |
            NAnd2(_, output) | NAnd3(_, output)
            => *output,
        }
    }
    
    pub fn execute(&self, ctx: &[Signal]) -> Signal {
        match self {
            Not1(input, output) => {
                ctx[input[0]].not()
            }
            And2(input, output) => {
                ctx[input[0]].and(&ctx[input[1]])
            }
            Or2(input, output) => {
                ctx[input[0]].or(&ctx[input[1]])
            }
            Xor2(input, output) => {
                ctx[input[0]].xor(&ctx[input[1]])
            }
            NAnd2(input, output) => {
                ctx[input[0]].and(&ctx[input[1]]).not()
            }
            NAnd3(input, output) => {
                ctx[input[0]]
                    .and(&ctx[input[1]])
                    .and(&ctx[input[2]])
                    .not()
            }
        }
    }

    pub fn execute_mut(&self, ctx: &mut [Signal]) {
        match self {
            Not1(input, output) => {
                ctx[*output] = ctx[input[0]].not();
            }
            And2(input, output) => {
                ctx[*output] = ctx[input[0]].and(&ctx[input[1]]);
            }
            Or2(input, output) => {
                ctx[*output] = ctx[input[0]].or(&ctx[input[1]]);
            }
            Xor2(input, output) => {
                ctx[*output] = ctx[input[0]].xor(&ctx[input[1]]);
            }
            NAnd2(input, output) => {
                ctx[*output] = ctx[input[0]].and(&ctx[input[1]]).not();
            }
            NAnd3(input, output) => {
                ctx[*output] = ctx[input[0]]
                    .and(&ctx[input[1]])
                    .and(&ctx[input[2]])
                    .not()
            }
        }
    }
}

impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Not(_) => write!(f, "not"),
            And(_) => write!(f, "and"),
            Or(_) => write!(f, "or"),
            Xor(_) => write!(f, "xor"),
            NAnd(_) => write!(f, "nand"),
        }
    }
}