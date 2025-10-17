//! 作为基本逻辑门部件的 LogicGate，我们将其所有数据放在栈上  
//! 同时保证输入和输出的参数化定义带来的灵活性


use crate::gate::GateType::{Not, Or};
use crate::gate::LogicGate::{And2, NAnd2, NAnd3, Not1, Or2, Xor2};
use crate::signal::BinarySignal;
use super::{ LogicGate, GateType };

fn print_new_logic_gate_type() {

}
impl LogicGate {
    pub fn new(typ: GateType, input: &[usize], output: &[usize]) -> Self {
        use GateType::*;
        match typ {
            Not(input_size) => {
                Not1([input[0]], [output[0]])
            }
            And(input_size) => {
                And2([input[0], input[1]], [output[0]])
            }
            Or(input_size) => {
                Or2([input[0], input[1]], [output[0]])
            }
            Xor(input_size) => {
                Xor2([input[0], input[1]], [output[0]])
            }
            NAnd(input_size) => {
                match input_size {
                    2 => NAnd2([input[0], input[1]], [output[0]]),
                    3 => NAnd3([input[0], input[1], input[2]], [output[0]]),
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn get_type(&self) -> GateType {
        use GateType::*;
        match self {
            Not1(input,_) => Not(input.len()),
            And2(input,_) => And(input.len()),
            Or2(input,_) => Or(input.len()),
            Xor2(input,_) => Xor(input.len()),
            NAnd2(input, _) => NAnd(input.len()),
            NAnd3(input, _) => NAnd(input.len()),
        }
    }

    pub fn get_input(&self) -> &[usize] {
        use LogicGate::*;
        match self {
            Not1(input, _) => input,
            And2(input, _) | Or2(input, _) | Xor2(input, _) |
            NAnd2(input, _)
            => input,
            NAnd3(input, _) => input,
        }
    }

    pub fn get_output(&self) -> usize {
        use LogicGate::*;
        match self {
            Not1(_, output) => output[0],
            And2(_, output) | Or2(_, output) | Xor2(_, output) |
            NAnd2(_, output) | NAnd3(_, output)
            => output[0],
        }
    }

    pub fn execute<S>(&self, ctx: &mut [S])
    where
        S: BinarySignal
    {
        match self {
            Not1(input, output) => {
                ctx[output[0]] = ctx[input[0]].not();
            }
            And2(input, output) => {
                ctx[output[0]] = ctx[input[0]].and(&ctx[input[1]]);
            }
            Or2(input, output) => {
                ctx[output[0]] = ctx[input[0]].or(&ctx[input[1]]);
            }
            Xor2(input, output) => {
                ctx[output[0]] = ctx[input[0]].xor(&ctx[input[1]]);
            }
            NAnd2(input, output) => {
                ctx[output[0]] = ctx[input[0]].and(&ctx[input[1]]).not();
            }
            NAnd3(input, output) => {
                ctx[output[0]] = ctx[input[0]]
                    .and(&ctx[input[1]])
                    .and(&ctx[input[2]])
                    .not()
            }
        }
    }
}

