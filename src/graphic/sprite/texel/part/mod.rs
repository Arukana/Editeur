pub mod err;


pub use self::err::{PartError, Result};
use std::fmt;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
    EyeLeft,
    EyeRight,
    None,
}

impl Part {
    pub fn new(part: &str) -> Result<Self> {
        match part {
            "EyeLeft" => Ok(Part::EyeLeft),
            "EyeRight" => Ok(Part::EyeRight),
            "None" => Ok(Part::None),
            _ => Err(PartError::UnknownPart),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Part::EyeLeft => "El",
                   Part::EyeRight => "Er",
                   Part::None => "__",
               })
    }
}
