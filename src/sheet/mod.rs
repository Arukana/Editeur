mod err;

use std::fmt;

pub use self::err::{SheetError, Result};

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Sheet {
    /// Symbol '_'.
    None = 0x5f,
    BustAngry = b'A' as u32,
    BustHappy = b'H' as u32,
    BustLove = b'L' as u32,
    BustMalicious = b'M' as u32,
    BustMisunderstanding = b'm' as u32,
    BustNormal = b'n' as u32,
    BustPlaying = b'P' as u32,
    BustShocked = b'S' as u32,
    BustSleepy = b's' as u32,
    BustSpeechless = b'p' as u32,
    BustSurprised = b'U' as u32,
    LyingAngry = b'a' as u32,
    LyingHappy = b'h' as u32,
    LyingLove = b'l' as u32,
    LyingMalicious = b'I' as u32,
    LyingMisunderstanding = b'i' as u32,
    LyingNormal = b'N' as u32,
    LyingPlaying = b'R' as u32,
    LyingShocked = b'D' as u32,
    LyingSleepy = b'E' as u32,
    LyingSpeechless = b'C' as u32,
    LyingSurprised = b'u' as u32,
    SeizaAngry = b'G' as u32,
    SeizaHappy = b'Y' as u32,
    SeizaLove = b'V' as u32,
    SeizaMalicious = b'c' as u32,
    SeizaMisunderstanding = b'e' as u32,
    SeizaNormal = b'o' as u32,
    SeizaPlaying = b'g' as u32,
    SeizaShocked = b'K' as u32,
    SeizaSleepy = b'y' as u32,
    SeizaSpeechless = b'Z' as u32,
    SeizaSurprised = b'd' as u32,
}

impl Sheet {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "None" => Ok(Sheet::None),
            "BustAngry" => Ok(Sheet::BustAngry),
            "BustHappy" => Ok(Sheet::BustHappy),
            "BustLove" => Ok(Sheet::BustLove),
            "BustMalicious" => Ok(Sheet::BustMalicious),
            "BustMisunderstanding" => Ok(Sheet::BustMisunderstanding),
            "BustNormal" => Ok(Sheet::BustNormal),
            "BustPlaying" => Ok(Sheet::BustPlaying),
            "BustShocked" => Ok(Sheet::BustShocked),
            "BustSleepy" => Ok(Sheet::BustSleepy),
            "BustSpeechless" => Ok(Sheet::BustSpeechless),
            "BustSurprised" => Ok(Sheet::BustSurprised),
            "LyingAngry" => Ok(Sheet::LyingAngry),
            "LyingHappy" => Ok(Sheet::LyingHappy),
            "LyingLove" => Ok(Sheet::LyingLove),
            "LyingMalicious" => Ok(Sheet::LyingMalicious),
            "LyingMisunderstanding" => Ok(Sheet::LyingMisunderstanding),
            "LyingNormal" => Ok(Sheet::LyingNormal),
            "LyingPlaying" => Ok(Sheet::LyingPlaying),
            "LyingShocked" => Ok(Sheet::LyingShocked),
            "LyingSleepy" => Ok(Sheet::LyingSleepy),
            "LyingSpeechless" => Ok(Sheet::LyingSpeechless),
            "LyingSurprised" => Ok(Sheet::LyingSurprised),
            "SeizaAngry" => Ok(Sheet::SeizaAngry),
            "SeizaHappy" => Ok(Sheet::SeizaHappy),
            "SeizaLove" => Ok(Sheet::SeizaLove),
            "SeizaMalicious" => Ok(Sheet::SeizaMalicious),
            "SeizaMisunderstanding" => Ok(Sheet::SeizaMisunderstanding),
            "SeizaNormal" => Ok(Sheet::SeizaNormal),
            "SeizaPlaying" => Ok(Sheet::SeizaPlaying),
            "SeizaShocked" => Ok(Sheet::SeizaShocked),
            "SeizaSleepy" => Ok(Sheet::SeizaSleepy),
            "SeizaSpeechless" => Ok(Sheet::SeizaSpeechless),
            "SeizaSurprised" => Ok(Sheet::SeizaSurprised),
            name => Err(SheetError::UnknownSheet(name.to_string())),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match *self {
            Sheet::None => "None",
            Sheet::BustAngry => "BustAngry",
            Sheet::BustHappy => "BustHappy",
            Sheet::BustLove => "BustLove",
            Sheet::BustMalicious => "BustMalicious",
            Sheet::BustMisunderstanding => "BustMisunderstanding",
            Sheet::BustNormal => "BustNormal",
            Sheet::BustPlaying => "BustPlaying",
            Sheet::BustShocked => "BustShocked",
            Sheet::BustSleepy => "BustSleepy",
            Sheet::BustSpeechless => "BustSpeechless",
            Sheet::BustSurprised => "BustSurprised",
            Sheet::LyingAngry => "LyingAngry",
            Sheet::LyingHappy => "LyingHappy",
            Sheet::LyingLove => "LyingLove",
            Sheet::LyingMalicious => "LyingMalicious",
            Sheet::LyingMisunderstanding => "LyingMisunderstanding",
            Sheet::LyingNormal => "LyingNormal",
            Sheet::LyingPlaying => "LyingPlaying",
            Sheet::LyingShocked => "LyingShocked",
            Sheet::LyingSleepy => "LyingSleepy",
            Sheet::LyingSpeechless => "LyingSpeechless",
            Sheet::LyingSurprised => "LyingSurprised",
            Sheet::SeizaAngry => "SeizaAngry",
            Sheet::SeizaHappy => "SeizaHappy",
            Sheet::SeizaLove => "SeizaLove",
            Sheet::SeizaMalicious => "SeizaMalicious",
            Sheet::SeizaMisunderstanding => "SeizaMisunderstanding",
            Sheet::SeizaNormal => "SeizaNormal",
            Sheet::SeizaPlaying => "SeizaPlaying",
            Sheet::SeizaShocked => "SeizaShocked",
            Sheet::SeizaSleepy => "SeizaSleepy",
            Sheet::SeizaSpeechless => "SeizaSpeechless",
            Sheet::SeizaSurprised => "SeizaSurprised",
        }
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl Default for Sheet {
    fn default() -> Sheet {
        Sheet::None
    }
}
