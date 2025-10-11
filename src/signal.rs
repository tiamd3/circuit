use std::fmt;

pub trait BinarySignal {
    fn get_unchecked(&self) -> bool;
    fn from_bool(b: bool) -> Self;
}

#[derive(Clone, Copy)]
pub struct Signal(Option<bool>);

impl BinarySignal for Signal {
    fn get_unchecked(&self) -> bool {
        self.0.unwrap()
    }
    
    fn from_bool(b: bool) -> Self {
        Signal(Some(b))
    }
}

impl Default for Signal {
    fn default() -> Self {
        Signal(None)
    }
}
impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(true) => write!(f, "1"),
            Some(false) => write!(f, "0"),
            None => write!(f, "_"),
        }
    }
}