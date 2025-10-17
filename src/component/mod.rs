use std::fmt::Debug;
use crate::circuit::Circuit;

pub mod combinator;
mod info;

pub trait LogicComponent<S> : Debug {
    fn execute(&self, circuit: &Circuit<S>) -> bool;
}

