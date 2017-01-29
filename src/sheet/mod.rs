mod err;

use std::fmt;

pub use self::err::{SheetError, Result};

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Sheet {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'A'.
    BustAngry = 0x41,
    /// Symbol 'H'.
    BustHappy = 0x48,
    /// Symbol 'L'.
    BustLove = 0x4c,
    /// Symbol 'M'.
    BustMalicious = 0x4d,
    /// Symbol 'm'.
    BustMisunderstanding = 0x6d,
    /// Symbol 'n'.
    BustNormal = 0x6e,
    /// Symbol 'P'.
    BustPlaying = 0x50,
    /// Symbol 'S'.
    BustShocked = 0x53,
    /// Symbol 's'.
    BustSleepy = 0x73,
    /// Symbol 'p'.
    BustSpeechless = 0x70, 
    /// Symbol 'U'.
    BustSurprised = 0x55,
    /// Symbol 'a'.
    LyingAngry = 0x61,
    /// Symbol 'h'.
    LyingHappy = 0x68,
    /// Symbol 'l'.
    LyingLove = 0x6c,
    /// Symbol 'I'.
    LyingMalicious = 0x49,
    /// Symbol 'i'.
    LyingMisunderstanding = 0x69,
    /// Symbol 'N'.
    LyingNormal = 0x4e,
    /// Symbol 'R'.
    LyingPlaying = 0x52,
    /// Symbol 'D'.
    LyingShocked = 0x44,
    /// Symbol 'E'.
    LyingSleepy = 0x45,
    /// Symbol 'C'.
    LyingSpeechless = 0x43,
    /// Symbol 'u'.
    LyingSurprised = 0x75,
    /// Symbol 'G'.
    SeizaAngry = 0x47,
    /// Symbol 'Y'.
    SeizaHappy = 0x59,
    /// Symbol 'V'.
    SeizaLove = 0x56,
    /// Symbol 'c'.
    SeizaMalicious = 0x63,
    /// Symbol 'e'.
    SeizaMisunderstanding = 0x65,
    /// Symbol 'o'.
    SeizaNormal = 0x6f,
    /// Symbol 'g'.
    SeizaPlaying = 0x67,
    /// Symbol 'K'.
    SeizaShocked = 0x4b,
    /// Symbol 'y'.
    SeizaSleepy = 0x79,
    /// Symbol 'Z'.
    SeizaSpeechless = 0x5a,
    /// Symbol 'd'.
    SeizaSurprised = 0x64,
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
