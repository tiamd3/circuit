use crate::circuit::Circuit;
use super::info::ComponentInfo;
use crate::gate::LogicGate;

enum ComponentType {
    HalfAdder,
}
pub struct Combinator {
    typ: ComponentType,
    info: ComponentInfo,
    gates: Vec<LogicGate>,
}

// impl Combinator {
//     fn new<S>(typ: ComponentType, input: &[usize], circuit: &Circuit<S>) -> Self {
//
//     }
// }
// /// 如果说这里 HalfAdder 应该有数据，那么数据只有一个作用就是内部状态
// pub struct HalfAdder;
//
//
//     fn build(&self, input: &[usize], circuit: &mut Circuit<S>) -> Vec<usize>
//     {
//         let mut output = vec![0; 2];
//         output[0] = circuit.add_gate(GateType::Xor(2), input);
//         output[1] = circuit.add_gate(GateType::And(2), input);
//         output
//     }
//
//     fn execute(&self, circuit: &Circuit<S>) {
//
//     }
// }
//
// /// 基本RS触发器的执行要求对于与非门实现双短路运算
// ///
// ///
// pub struct BRSFlipFlop;
//
// impl<S: BinarySignal> LogicComponent<S> for BRSFlipFlop {
//     fn build(&self, input: &[usize], circuit: &mut Circuit<S>) -> Vec<usize> {
//         let output = circuit.advance_output(2);
//         let not_q = output[0];
//         let q = output[1];
//         let rd = &[input[0], q];
//         let sd = &[input[1], not_q];
//         circuit.add_gate_with_output(GateType::NAnd(2), rd, not_q);
//         circuit.add_gate_with_output(GateType::NAnd(2), sd, q);
//         output
//     }
// }
//

