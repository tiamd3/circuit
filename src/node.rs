use std::vec;
use serde::{Deserialize, Serialize};
use crate::pattern::Pattern;
use crate::circuit::Circuit;
use crate::signal::Signal;
use NodeType::*;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    And, Or, Not, Xor, NAnd,
    Pattern(String)
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    node_type: NodeType,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl Node {
    pub fn new(node_type: NodeType, inputs: Vec<usize>, outputs: Vec<usize>) -> Self {
        Node { node_type, inputs, outputs }
    }

    pub fn execute(&self, circuit: &Circuit, signals: &[Signal]) -> Vec<Signal> {
        let mut result = signals.to_vec();
        self.execute_mut(circuit, &mut result);
        result
    }

    pub fn execute_mut(&self, circuit: &Circuit, signals: &mut [Signal]) {
        match self.get_type() {
            Not | And | Or | Xor | NAnd  =>  {
                let input_signals = self.get_input().iter()
                    .map(|i| signals[*i])
                    .collect::<Vec<Signal>>();
                let res = Self::execute_gate(self.get_type(), &input_signals);
                signals[self.get_output()[0]] = res;
            }
            Pattern(name) => {
                let pattern = circuit.get_patterns().get(name).unwrap();
                self.execute_pattern_mut(pattern, signals);
            }
        }
    }

    //核心，Pattern的运行函数
    pub fn execute_pattern_mut(
        &self,
        pattern: &Pattern,
        signals: &mut [Signal],

    ) {
        let nodes = pattern.get_pattern();
        //Pattern中的node没有保存输出，这里的output实际上是node的索引下标，用来指向用作输出的node
        let output_nodes = pattern.get_output();
        let input_signals = self.get_input().iter()
            .map(|i| signals[*i].clone())
            .collect::<Vec<Signal>>();
        //用来存储每个node的输出
        let mut output_signals = vec![Signal::default(); nodes.len()];
        //预读信号提供给触发器使用
        self.get_output().iter().enumerate().for_each(|(i, o)| {
            output_signals[output_nodes[i]] = signals[*o].clone();
        });
        for (i, node) in nodes.iter().enumerate() {
            let res = node.execute(&input_signals, &output_signals);
            output_signals[i]= res.clone();
        }
        self.get_output().iter().enumerate()
            .for_each(|(node, i)|  {
                signals[*i] = output_signals[output_nodes[node]]
            });
    }

    pub fn execute_gate(gate: &NodeType, input_signals: &[Signal]) -> Signal {
        match gate {
            Not => {
                Self::execute_not(&input_signals[0])
            }
            And => {
                Self::execute_and(&input_signals)
            }
            Or => {
                Self::execute_or(&input_signals)
            }
            Xor => {
                Self::execute_xor(&input_signals[0], &input_signals[1])
            }
            NAnd => {
                Self::execute_nand(&input_signals)
            }
            _ => unreachable!()
        }
    }

    pub fn execute_not(signals: &Signal) -> Signal { signals.not() }

    pub fn execute_and(signals: &[Signal]) -> Signal {
        signals.iter().fold(
            Signal::from_bool(Some(true)),
            |b, x| b.and(x)
        )
    }

    pub fn execute_or(signals: &[Signal]) -> Signal {
        signals.iter().fold(
            Signal::from_bool(Some(false)),
            |b, x| b.or(x)
        )
    }

    pub fn execute_xor(l: &Signal, r: &Signal) -> Signal {
        l.and(&r.not()).or(&r.and(&l.not()))
    }

    pub fn execute_nand(signals: &[Signal]) -> Signal {
        Self::execute_and(signals).not()
    }

    pub fn get_type(&self) -> &NodeType { &self.node_type }

    pub fn get_output(&self) -> &[usize] { &self.outputs }

    pub fn get_input(&self) -> &[usize] { &self.inputs }

}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, input: {:?}, output: {:?}", self.node_type, self.inputs, self.outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_node() {
        let and_node = Node::new(And, vec![0, 1], vec![2]);
        let signals = vec![
            Signal::from_usize(0),
            Signal::from_usize(1),
        ];
        let res = Node::execute_gate(and_node.get_type(), &signals);
        println!("{:?}", res);
    }
}