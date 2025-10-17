use std::fmt;
use std::fmt::Debug;

pub trait BinarySignal: 
Default + Clone + Debug
{
    fn get_unchecked(&self) -> bool;
    fn from_bool(b: bool) -> Self;

    fn not(&self) -> Self {
        Self::from_bool(!self.get_unchecked())
    }
    fn and(&self, other: &Self) -> Self {
        Self::from_bool(self.get_unchecked() && other.get_unchecked())
    }
    fn or(&self, other: &Self) -> Self {
        Self::from_bool(self.get_unchecked() || other.get_unchecked())
    }

    fn xor(&self, other: &Self) -> Self {
        let l = self.get_unchecked();
        let r = other.get_unchecked();
        Self::from_bool(!l && r || l && !r)
    }
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