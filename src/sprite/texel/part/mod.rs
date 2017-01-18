pub mod err;


pub use self::err::{PartError, Result};
use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'a'.
    ArmLeft = 0x61,
    /// Symbol 'A'.
    ArmRight = 0x41,
    /// Symbol 'b'.
    Boobs = 0x62,
    /// Symbol 'c'.
    Clavicle = 0x63,
    /// Symbol 'e'.
    EarLeft = 0x65,
    /// Symbol 'E'.
    EarRight = 0x45,
    /// Symbol 'y'.
    EyeLeft = 0x79,
    /// Symbol 'Y'.
    EyeRight = 0x59,
    /// Symbol 'o'.
    HairTop = 0x6f,
    /// Symbol 'r'.
    HairLeft = 0x72,
    /// Symbol 'R'.
    HairRight = 0x52,
    /// Symbol 'd',
    HandLeft = 0x64,
    /// Symbol 'D',
    HandRight = 0x44,
    /// Symbol 'm',
    Mouth = 0x6d,
    /// Symbol 't'.
    Tail = 0x74,
    /// Symbol 'l'.
    Bell = 0x6c,
    /// Symbol 'x'.
    ExclamationMark = 0x78,
    /// Symbol 'X'.
    ExclamationMarks = 0x58,
    /// Symbol 'h'.
    Heart = 0x68,
    /// Symbol 'H'.
    Hearts = 0x48,
    /// Symbol 'n'.
    Lantern = 0x6e,
    /// Symbol 'q'.
    QuestionMark = 0x71,
    /// Symbol 'Q'.
    QuestionMarks = 0x51,
    /// Symbol 'w'.
    WoolBall = 0x77,
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
        write!(f, "{}", match *self {
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

impl Default for Part {
    fn default() -> Part {
        Part::None
    }
}
