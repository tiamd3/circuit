use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use pest::pratt_parser::Op;

pub trait BinarySignal:
Default + Clone + Debug + Hash + PartialEq + Eq
{
    fn get_unchecked(&self) -> bool;
    fn from_bool(b: Option<bool>) -> Self;

    fn is_valid(&self) -> bool { true }
    fn from_usize(u: usize) -> Self {
        match u {
            0 => Self::from_bool(Some(false)),
            _ => Self::from_bool(Some(true)),
        }
    }
    fn get_parent(&self) -> Option<usize> { None}

    fn with_parent(b: Option<bool>, p: usize) -> Self { Self::from_bool(b) }

    fn set_parent(&mut self, p: usize) {}

    fn not(&self) -> Self {
        Self::from_bool(Some(!self.get_unchecked()))
    }
    fn and(&self, other: &Self) -> Self {
        Self::from_bool(Some(self.get_unchecked() && other.get_unchecked()))
    }
    fn or(&self, other: &Self) -> Self {
        Self::from_bool(Some(self.get_unchecked() || other.get_unchecked()))
    }

    fn xor(&self, other: &Self) -> Self {
        let l = self.get_unchecked();
        let r = other.get_unchecked();
        Self::from_bool(Some(!l && r || l && !r))
    }
}

#[derive(Clone, Copy, Hash)]
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
impl BinarySignal for Signal {
    fn get_unchecked(&self) -> bool { self.value.unwrap() }

    fn from_bool(b: Option<bool>) -> Self { Self { parent: None, value: b } }
    fn is_valid(&self) -> bool { self.value.is_some() }

    fn get_parent(&self) -> Option<usize> { self.parent }

    fn with_parent(b: Option<bool>, p: usize) -> Self { Self { parent: Some(p), value: b } }

    fn set_parent(&mut self, p: usize) { self.parent = Some(p) }
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