use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::circuit::Circuit;

pub mod combinator;
mod info;

pub trait LogicComponent<'a, S> : Debug + Deserialize<'a> + Serialize {
    fn execute(&self, circuit: &Circuit<S>) -> bool;
}

