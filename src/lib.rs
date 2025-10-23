//!
//! An interesting simulation of a digital circuit
//!

#![allow(unused)]
mod signal;
mod circuit;
pub mod component;

mod gate;

pub mod app;
mod util;

use std::fmt::Debug;
use signal::BinarySignal;
use circuit::Circuit;

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    use super::*;

    #[test]
    fn test() {

    }
}































