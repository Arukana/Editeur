mod err;

use std::fmt;

pub use self::err::{EmotionError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Emotion {
    Happy,
    Malicious,
    None,
}

impl Emotion {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "Happy" => Ok(Emotion::Happy),
            "Malicious" => Ok(Emotion::Malicious),
            "None" => Ok(Emotion::None),
            _ => Err(EmotionError::UnknownEmotion),
        }
    }

    /// The accessor method `is_none` returns a boolean
    /// for None, axiom of emotion.
    pub fn is_none(&self) -> bool {
        self.eq(&Emotion::None)
    }
}

impl fmt::Display for Emotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self{
            Emotion::Happy => 'H',
            Emotion::Malicious => 'M',
            Emotion::None => '_',
        })
    }
}

impl Default for Emotion {
    fn default() -> Emotion {
        Emotion::None
    }
}
