use crate::signal::BinarySignal;

#[derive(Debug, Copy, Clone)]
pub enum GateType {
    Not,
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub struct LogicGate {
    typ: GateType,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl LogicGate {
    pub fn new(typ: GateType, input: &[usize], output: &[usize]) -> Self {
        LogicGate {
            typ,
            input: input.to_vec(),
            output: output.to_vec(),
        }
    }

    pub fn execute<S>(&self, ctx: &mut [S])
    where
        S: BinarySignal
    {
        use GateType::*;
        match self.typ {
            Not => {
                ctx[self.output[0]] = S::from_bool(!ctx[self.input[0]].get_unchecked())
            }
            And => {
                let l = ctx[self.input[0]].get_unchecked();
                let r = ctx[self.input[1]].get_unchecked();
                ctx[self.output[0]] = S::from_bool(l && r)
            }
            Or => {
                let l = ctx[self.input[0]].get_unchecked();
                let r = ctx[self.input[1]].get_unchecked();
                ctx[self.output[0]] = S::from_bool(l || r)
            }
            Xor => {
                let l = ctx[self.input[0]].get_unchecked();
                let r = ctx[self.input[1]].get_unchecked();
                ctx[self.output[0]] = S::from_bool(
                    (!l && r)  || (l && ! r)
                )
            }
        }
    }
}

