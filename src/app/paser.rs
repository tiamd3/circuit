use std::num::ParseIntError;
use eframe::App;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use crate::circuit::Circuit;
use crate::gate::GateType;
use crate::signal::{BinarySignal, Signal};
use super::repl::{ Repl };

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DigicirParser;

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

pub struct Interpreter;

impl Interpreter {
    pub fn parse(repl: &mut Repl, code: &str) -> Result<(), ParError> {
        match DigicirParser::parse(Rule::repl, code) {
            Ok(pairs) => {
                println!("{:#?}", pairs);
                for pair in pairs {
                    let pair = next_pair_unwrap(pair);
                    match pair.as_rule() {
                        Rule::state => {
                            Self::parse_state(repl, pair)?;
                        }
                        Rule::circuit => {
                            CircuitInterpreter::parse_circuit(repl.get_mut_circuit(), pair)?;
                        }
                        _ => unreachable!()
                    }
                }
                Ok(())
            }
            Err(e) => {
                Err(ParError::PestError(e))
            }
        }
    }

    pub fn parse_state(repl: &mut Repl, pair: Pair<Rule>) -> Result<(), ParError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::table_outputs => {
                    let outputs = parse_usize_sequence(pair.into_inner())?;
                    repl.update_truth_table(&outputs);
                }
                _ => unreachable!()
            }
        }
        Ok(())
    }
}
pub struct CircuitInterpreter;

impl CircuitInterpreter
{
    fn parse_gate_name(name: &str, input_size: usize) -> Result<GateType, ParError> {
        match name {
            "not" => Ok(GateType::Not(input_size)),
            "and" => Ok(GateType::And(input_size)),
            "or" => Ok(GateType::Or(input_size)),
            "xor" => Ok(GateType::Xor(input_size)),
            _ => Err(ParError::UnDefinedError(format!("Unknown gate: {}", name))),
        }
    }



    fn parse_add_gate(circuit: &mut Circuit<Signal>, pair: Pair<Rule>) -> Result<(), ParError> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let gate = inner.next().unwrap();
        let mut vec = Vec::new();
        for number_pair in inner {
            vec.push(parse_usize(number_pair)?);
        }
        match rule {
            Rule::add_gate_default => {
                let gate_type = Self::parse_gate_name(&gate.as_str(), vec.len())?;
                circuit.add_gate(gate_type, &vec);
            }
            Rule::add_gate_with_output => {
                let gate_type = Self::parse_gate_name(&gate.as_str(), vec.len()-1)?;
                circuit.add_gate_with_output(gate_type, &vec, *vec.last().unwrap());
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn parse_execute(circuit: &mut Circuit<Signal>, pair: Pair<Rule>) -> Result<(), ParError> {
        let mut inner = pair.into_inner();
        let mut input = Vec::new();
        for bool_pair in inner {
            match bool_pair.into_inner().next().unwrap().as_rule() {
                Rule::T_VALUE => { input.push(Signal::from_bool(true))}
                Rule::F_VALUE => { input.push(Signal::from_bool(false))}
                _ => unreachable!(),
            }
        }
        circuit.execute_gates(&input);
        Ok(())
    }
    fn parse_circuit(circuit: &mut Circuit<Signal>, pair: Pair<Rule>) -> Result<(), ParError> {
        let mut inner = pair.into_inner();
        for pair in inner {
            println!("{:?}", pair.as_rule());
            let rule = pair.as_rule();
            match pair.as_rule() {
                Rule::input_size_s => {
                    let size = parse_usize(pair.into_inner().next().unwrap())?;
                    *circuit = Circuit::new_with_input(size);
                }
                Rule::add_gate_s => {
                    let inner_pair = pair
                        .into_inner().next().unwrap()
                        .into_inner().next().unwrap();
                    Self::parse_add_gate(circuit, inner_pair)?;
                }
                Rule::execute => {
                    Self::parse_execute(circuit, pair)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

fn next_pair_unwrap(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().unwrap()
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
        output.push(parse_usize(pair)?);
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    #[test]
    fn test_parser() {
        let source = std::fs::read_to_string("test.dc")
            .expect("Couldn't read file");
        let mut circuit: Circuit<Signal> = Circuit::new();
       

    }
}