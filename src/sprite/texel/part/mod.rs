pub mod err;


pub use self::err::{PartError, Result};
use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
    None = b'_',

    ArmLeft = b'a',
    ArmRight = b'A',
    Boobs = b'b',
    Clavicle = b'c',
    EarLeft = b'e',
    EarRight = b'E',
    EyeLeft = b'y',
    EyeRight = b'Y',
    HairTop = b'o',
    HairLeft = b'r',
    HairRight = b'R',
    HandLeft = b'd',
    HandRight = b'D',
    Mouth = b'm',
    Tail = b't',

    Bell = b'l',
    ExclamationMark = b'x',
    ExclamationMarks = b'X',
    Heart = b'h',
    Hearts = b'H',
    Lantern = b'n',
    QuestionMark = b'q',
    QuestionMarks = b'Q',
    WoolBall = b'w',
}

impl Part {
    pub fn new(part: &str) -> Result<Self> {
        match part {
            "ArmLeft" => Ok(Part::ArmLeft),
            "ArmRight" => Ok(Part::ArmRight),
            "Boobs" => Ok(Part::Boobs),
            "Clavicle" => Ok(Part::Clavicle),
            "EarLeft" => Ok(Part::EarLeft),
            "EarRight" => Ok(Part::EarRight),
            "EyeLeft" => Ok(Part::EyeLeft),
            "EyeRight" => Ok(Part::EyeRight),
            "HairTop" => Ok(Part::HairTop),
            "HairLeft" => Ok(Part::HairLeft),
            "HairRight" => Ok(Part::HairRight),
            "HandLeft" => Ok(Part::HandLeft),
            "HandRight" => Ok(Part::HandRight),
            "Mouth" => Ok(Part::Mouth),
            "Tail" => Ok(Part::Tail),

            "Bell" => Ok(Part::Bell),
            "ExclamationMark" => Ok(Part::ExclamationMark),
            "ExclamationMarks" => Ok(Part::ExclamationMarks),
            "Heart" => Ok(Part::Heart),
            "Hearts" => Ok(Part::Hearts),
            "Lantern" => Ok(Part::Lantern),
            "QuestionMark" => Ok(Part::QuestionMark),
            "QuestionMarks" => Ok(Part::QuestionMarks),
            "WoolBall" => Ok(Part::WoolBall),

            "None" => Ok(Part::None),
            name => Err(PartError::UnknownPart(name.to_string())),
        }
    }

    pub fn not_empty(&self) -> Option<&Part> {
        match *self {
            Part::None => None,
            ref other => Some(other)
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match *self {
                  Part::ArmLeft => "Ml",
                  Part::ArmRight => "Mr",
                  Part::Boobs => "Oo",
                  Part::Clavicle => "Cl",
                  Part::EarLeft => "Al",
                  Part::EarRight => "Ar",
                  Part::EyeLeft => "El",
                  Part::EyeRight => "Er",
                  Part::HairTop => "Rt",
                  Part::HairLeft => "Rl",
                  Part::HairRight => "Rr",
                  Part::HandLeft => "Hl",
                  Part::HandRight => "Hr",
                  Part::Mouth => "Mo",
                  Part::Tail => "Ta",

                  Part::Bell => "Be",
                  Part::ExclamationMark => "Xm",
                  Part::ExclamationMarks => "Xs",
                  Part::Heart => "He",
                  Part::Hearts => "Hs",
                  Part::Lantern => "La",
                  Part::QuestionMark => "Qm",
                  Part::QuestionMarks => "Qs",
                  Part::WoolBall => "Wb",

                  Part::None => "__",
               })
    }
}
