mod err;

use std::fmt;

pub use self::err::{PostureError, Result};

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Posture {
    None = 0,
    Talk = 1,
    NotTalk = 2,
    Bust = 3,
    Lying = 4,
    Seiza = 5,
}

impl Posture {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "Talk" => Ok(Posture::Talk),
            "NotTalk" => Ok(Posture::NotTalk),
            "Bust" => Ok(Posture::Bust),
            "Lying" => Ok(Posture::Lying),
            "Seiza" => Ok(Posture::Seiza),
            name => Err(PostureError::UnknownPosture(name.to_string())),
        }
    }
}

impl fmt::Display for Posture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Posture::Talk => "Talk",
            Posture::NotTalk => "NotTalk",
            Posture::Bust => "Bust",
            Posture::Lying => "Lying",
            Posture::Seiza => "Seiza",
            Posture::None => "None",
        })
    }
}

impl Default for Posture {
    fn default() -> Posture {
        Posture::None
    }
}
