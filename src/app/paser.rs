use std::num::ParseIntError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use crate::circuit::Circuit;
use crate::gate::GateType;
use crate::signal::{BinarySignal, Signal};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CircuitParser;

#[derive(Error, Debug)]
pub enum ParError {
    #[error("over range of: {0}")]
    OverRangeError(String),

    #[error("parse error: {0}")]
    UnDefinedError(String),

    #[error("parse int error at: ({0}, {1})")]
    UsizeParseError(usize, usize),

    #[error("{0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

pub struct CircuitInterpreter<S> {
    circuit: Circuit<S>,
    source: String,
}

impl<S> CircuitInterpreter<S>
where
    S: BinarySignal
{
    pub fn new(source: String) -> Self {
        Self { circuit: Circuit::new() , source }
    }

    fn parse_gate_name(name: &str, input_size: usize) -> Result<GateType, ParError> {
        match name {
            "not" => Ok(GateType::Not(input_size)),
            "and" => Ok(GateType::And(input_size)),
            "or" => Ok(GateType::Or(input_size)),
            "xor" => Ok(GateType::Xor(input_size)),
            _ => Err(ParError::UnDefinedError(format!("Unknown gate: {}", name))),
        }
    }

    fn parse_usize(number: Pair<Rule>) -> Result<usize, ParError> {
        match number.as_str().parse::<usize>() {
            Ok(n) => Ok(n),
            Err(_) => Err(ParError::UsizeParseError(number.line_col().0, number.line_col().1)),
        }
    }

    fn parse_usize_sequence(numbers: Pairs<Rule>) -> Result<Vec<usize>, ParError> {
        let mut output = Vec::new();
        for pair in numbers {
             output.push(Self::parse_usize(pair)?);
        }
        Ok(output)
    }
    pub fn parse(&mut self) -> Result<(), ParError> {
        match CircuitParser::parse(Rule::circuit, &self.source.clone()) {
            Ok(pairs) => {
                //println!("{:#?}", pairs);
                for pair in pairs {
                    self.parse_circuit(pair)?;
                }
                Ok(())
            }
            Err(e) => {
                Err(ParError::PestError(e))
            }
        }
    }


    fn parse_add_gate(&mut self, pair: Pair<Rule>) -> Result<(), ParError> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let gate = inner.next().unwrap();
        let mut vec = Vec::new();
        for number_pair in inner {
            vec.push(Self::parse_usize(number_pair)?);
        }
        match rule {
            Rule::add_gate_default => {
                let gate_type = Self::parse_gate_name(&gate.as_str(), vec.len())?;
                self.circuit.add_gate(gate_type, &vec);
            }
            Rule::add_gate_with_output => {
                let gate_type = Self::parse_gate_name(&gate.as_str(), vec.len()-1)?;
                self.circuit.add_gate_with_output(gate_type, &vec, *vec.last().unwrap());
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn parse_execute(&mut self, pair: Pair<Rule>) -> Result<(), ParError> {
        let mut inner = pair.into_inner();
        let mut input = Vec::new();
        for bool_pair in inner {
            match bool_pair.into_inner().next().unwrap().as_rule() {
                Rule::T_VALUE => { input.push(S::from_bool(true))}
                Rule::F_VALUE => { input.push(S::from_bool(false))}
                _ => unreachable!(),
            }
        }
        self.circuit.execute_gates(&input);
        Ok(())
    }
    fn parse_circuit(&mut self, pair: Pair<Rule>) -> Result<(), ParError> {
        let mut inner = pair.into_inner();
        for pair in inner {
            println!("{:?}", pair.as_rule());
            let rule = pair.as_rule();
            match pair.as_rule() {
                Rule::input_size_s => {
                    let size = Self::parse_usize(pair.into_inner().next().unwrap())?;
                    self.circuit = Circuit::new_with_input(size);
                }
                Rule::add_gate_s => {
                    let inner_pair = pair
                        .into_inner().next().unwrap()
                        .into_inner().next().unwrap();
                    self.parse_add_gate(inner_pair)?;
                }
                Rule::execute => {
                    self.parse_execute(pair)?;
                }
                Rule::check => {
                    let line = pair.line_col().0;
                    let mut inner = pair.into_inner();
                    let size = self.circuit.get_signal_size();
                    let mut output = Vec::new();
                    for pair in inner {
                        let index = Self::parse_usize(pair)?;
                        if index < size {
                            output.push(self.circuit.get_signal(index));
                        } else {
                            return Err(ParError::OverRangeError("signal index range".to_string()))
                        }
                    }
                    println!("[{}] : {:?}", line, output);
                }
                Rule::output_table_s => {
                    let mut inner = pair.into_inner();
                    let output_indexes = Self::parse_usize_sequence(inner)?;
                    self.circuit.for_every_input(&output_indexes, Circuit::print_output);
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    #[test]
    fn test_parser() {
        let source = std::fs::read_to_string("test.dc")
            .expect("Couldn't read file");
        let mut interpreter: CircuitInterpreter<Signal> = CircuitInterpreter::new(source);
        match interpreter.parse() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}