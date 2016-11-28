mod err;

use std::fmt;

pub use self::err::{EmotionError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Emotion {
    Angry,
    Happy,
    Love,
    Malicious,
    Misunderstanding,
    Shocked,
    Sleepy,
    Speechless,
    None,
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
        write!(f, "{}", match *self{
            Emotion::Angry => 'A',
            Emotion::Happy => 'H',
            Emotion::Love => 'L',
            Emotion::Malicious => 'M',
            Emotion::Misunderstanding => 'I',
            Emotion::Shocked => 'S',
            Emotion::Sleepy => 'E',
            Emotion::Speechless => 'C',
            Emotion::None => '_',
        })
    }
}

impl Default for Emotion {
    fn default() -> Emotion {
        Emotion::None
    }
}
