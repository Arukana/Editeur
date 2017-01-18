mod err;

use std::fmt;
use std::char;

pub use self::err::{EmotionError, Result};

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Emotion {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'a'.
    Angry = 0x61,
    /// Symbol 'h'.
    Happy = 0x68,
    /// Symbol 'l'.
    Love = 0x6c,
    /// Symbol 'm'.
    Malicious = 0x6d,
    /// Symbol 'i'.
    Misunderstanding = 0x69,
    /// Symbol 'o'.
    Shocked = 0x6f,
    /// Symbol 's'.
    Sleepy = 0x73,
    /// Symbol 'e'.
    Speechless = 0x65,
}

impl Emotion {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "Angry" => Ok(Emotion::Angry),
            "Happy" => Ok(Emotion::Happy),
            "Love" => Ok(Emotion::Love),
            "Malicious" => Ok(Emotion::Malicious),
            "Misunderstanding" => Ok(Emotion::Misunderstanding),
            "Shocked" => Ok(Emotion::Shocked),
            "Sleepy" => Ok(Emotion::Sleepy),
            "Speechless" => Ok(Emotion::Speechless),
            "None" => Ok(Emotion::None),
            name => Err(EmotionError::UnknownEmotion(name.to_string())),
        }
    }

    /// The accessor method `is_none` returns a boolean
    /// for None, axiom of emotion.
    pub fn is_none(&self) -> bool {
        self.eq(&Emotion::None)
    }

    pub fn not_empty(&self) -> Option<&Emotion> {
        match *self {
            Emotion::None => None,
            ref other => Some(other),
        }
    }
}

impl fmt::Display for Emotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe {
            char::from_u32_unchecked(*self as u32)
        })
    }
}

impl Default for Emotion {
    fn default() -> Emotion {
        Emotion::None
    }
}
