pub mod err;


pub use self::err::{PartError, Result};
use std::fmt;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
    EyeLeft,
    EyeRight,
    EarLeft,
    EarRight,
    HandLeft,
    HandRight,
    Mouth,
    Tail,

    Lantern,
    WoolBall,
    Bell,
    Heart,
    Hearts,
    QuestionMark,
    QuestionMarks,
    ExclamationMark,
    ExclamationMarks,

    None,
}

impl Part {
    pub fn new(part: &str) -> Result<Self> {
        match part {
            "EyeLeft" => Ok(Part::EyeLeft),
            "EyeRight" => Ok(Part::EyeRight),
            "EarLeft" => Ok(Part::EarLeft),
            "EarRight" => Ok(Part::EarRight),
            "HandLeft" => Ok(Part::HandLeft),
            "HandRight" => Ok(Part::HandRight),
            "Mouth" => Ok(Part::Mouth),
            "Tail" => Ok(Part::Tail),

            "Lantern" => Ok(Part::Lantern),
            "WoolBall" => Ok(Part::WoolBall),
            "Bell" => Ok(Part::Bell),
            "Heart" => Ok(Part::Heart),
            "Hearts" => Ok(Part::Hearts),
            "QuestionMark" => Ok(Part::QuestionMark),
            "QuestionMarks" => Ok(Part::QuestionMarks),
            "ExclamationMark" => Ok(Part::ExclamationMark),
            "ExclamationMarks" => Ok(Part::ExclamationMarks),

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
