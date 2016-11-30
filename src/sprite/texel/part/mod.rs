pub mod err;


pub use self::err::{PartError, Result};
use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
    None = 0,

    ArmLeft = 1,
    ArmRight = 2,
    Boobs = 3,
    Clavicle = 4,
    EarLeft = 5,
    EarRight = 6,
    EyeLeft = 7,
    EyeRight = 8,
    HairTop = 9,
    HairLeft = 10,
    HairRight = 11,
    HandLeft = 12,
    HandRight = 13,
    Mouth = 14,
    Tail = 15,

    Bell = 16,
    ExclamationMark = 17,
    ExclamationMarks = 18,
    Heart = 19,
    Hearts = 20,
    Lantern = 21,
    QuestionMark = 22,
    QuestionMarks = 23,
    WoolBall = 24,
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
