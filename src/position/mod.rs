mod err;

use std::fmt;

pub use self::err::{PostureError, Result};

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Posture {
    None = b'_',
    BustAngry = b'A',
    BustHappy = b'H',
    BustLove = b'L',
    BustMalicious = b'M',
    BustMisunderstanding = b'm',
    BustNormal = b'n',
    BustPlaying = b'P',
    BustShocked = b'S',
    BustSleepy = b's',
    BustSpeechless = b'p',
    BustSurprised = b'U',
    LyingAngry = b'a',
    LyingHappy = b'h',
    LyingLove = b'l',
    LyingMalicious = b'I',
    LyingMisunderstanding = b'i',
    LyingNormal = b'N',
    LyingPlaying = b'R',
    LyingShocked = b'D',
    LyingSleepy = b'E',
    LyingSpeechless = b'C',
    LyingSurprised = b'u',
    SeizaAngry = b'G',
    SeizaHappy = b'Y',
    SeizaLove = b'V',
    SeizaMalicious = b'c',
    SeizaMisunderstanding = b'e',
    SeizaNormal = b'o',
    SeizaPlaying = b'g',
    SeizaShocked = b'K',
    SeizaSleepy = b'y',
    SeizaSpeechless = b'Z',
    SeizaSurprised = b'd',
}

impl Posture {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "None" => Ok(Posture::None),
            "BustAngry" => Ok(Posture::BustAngry),
            "BustHappy" => Ok(Posture::BustHappy),
            "BustLove" => Ok(Posture::BustLove),
            "BustMalicious" => Ok(Posture::BustMalicious),
            "BustMisunderstanding" => Ok(Posture::BustMisunderstanding),
            "BustNormal" => Ok(Posture::BustNormal),
            "BustPlaying" => Ok(Posture::BustPlaying),
            "BustShocked" => Ok(Posture::BustShocked),
            "BustSleepy" => Ok(Posture::BustSleepy),
            "BustSpeechless" => Ok(Posture::BustSpeechless),
            "BustSurprised" => Ok(Posture::BustSurprised),
            "LyingAngry" => Ok(Posture::LyingAngry),
            "LyingHappy" => Ok(Posture::LyingHappy),
            "LyingLove" => Ok(Posture::LyingLove),
            "LyingMalicious" => Ok(Posture::LyingMalicious),
            "LyingMisunderstanding" => Ok(Posture::LyingMisunderstanding),
            "LyingNormal" => Ok(Posture::LyingNormal),
            "LyingPlaying" => Ok(Posture::LyingPlaying),
            "LyingShocked" => Ok(Posture::LyingShocked),
            "LyingSleepy" => Ok(Posture::LyingSleepy),
            "LyingSpeechless" => Ok(Posture::LyingSpeechless),
            "LyingSurprised" => Ok(Posture::LyingSurprised),
            "SeizaAngry" => Ok(Posture::SeizaAngry),
            "SeizaHappy" => Ok(Posture::SeizaHappy),
            "SeizaLove" => Ok(Posture::SeizaLove),
            "SeizaMalicious" => Ok(Posture::SeizaMalicious),
            "SeizaMisunderstanding" => Ok(Posture::SeizaMisunderstanding),
            "SeizaNormal" => Ok(Posture::SeizaNormal),
            "SeizaPlaying" => Ok(Posture::SeizaPlaying),
            "SeizaShocked" => Ok(Posture::SeizaShocked),
            "SeizaSleepy" => Ok(Posture::SeizaSleepy),
            "SeizaSpeechless" => Ok(Posture::SeizaSpeechless),
            "SeizaSurprised" => Ok(Posture::SeizaSurprised),
            name => Err(PostureError::UnknownPosture(name.to_string())),
        }
    }
}

impl fmt::Display for Posture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            _ => "None",
        })
    }
}

impl Default for Posture {
    fn default() -> Posture {
        Posture::None
    }
}
