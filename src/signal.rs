use std::fmt;
use std::hash::Hash;
use pest::pratt_parser::Op;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, Deserialize, Serialize)]
pub struct Signal {
    parent: Option<usize>,
    value: Option<bool>,
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Signal {}

impl Signal {
    pub fn get_unchecked(&self) -> bool { self.value.unwrap() }

    pub fn from_bool(b: Option<bool>) -> Self { Self { parent: None, value: b } }
    pub fn is_valid(&self) -> bool { self.value.is_some() }

    pub fn from_usize(u: usize) -> Self {
        match u {
            0 => Self::from_bool(Some(false)),
            _ => Self::from_bool(Some(true)),
        }
    }
    pub fn get_parent(&self) -> Option<usize> { self.parent }

    pub fn with_parent(b: Option<bool>, p: usize) -> Self { Self { parent: Some(p), value: b } }

    pub fn set_parent(&mut self, p: usize) { self.parent = Some(p) }

    pub fn not(&self) -> Self {
        Self::from_bool(Some(!self.get_unchecked()))
    }
    pub fn and(&self, other: &Self) -> Self {
        Self::from_bool(Some(self.get_unchecked() && other.get_unchecked()))
    }
    pub fn or(&self, other: &Self) -> Self {
        Self::from_bool(Some(self.get_unchecked() || other.get_unchecked()))
    }

    pub fn xor(&self, other: &Self) -> Self {
        let l = self.get_unchecked();
        let r = other.get_unchecked();
        Self::from_bool(Some(!l && r || l && !r))
    }
}

impl Default for Signal {
    fn default() -> Self { Signal { parent: None, value: None } }
}

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(true) => write!(f, "1"),
            Some(false) => write!(f, "0"),
            None => write!(f, "_"),
        }
    }
}