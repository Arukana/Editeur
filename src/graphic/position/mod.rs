mod err;

pub use self::err::{PostureError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Posture {
    LotusHandsOnFloor,
    LyingOnSomething,
    Lotus,
    Lying,
    Joke,
    None,
}

impl Posture {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "LotusHandsOnFloor" => Ok(Posture::LotusHandsOnFloor),
            "LyingOnSomething" => Ok(Posture::LyingOnSomething),
            "Lotus" => Ok(Posture::Lotus),
            "Lying" => Ok(Posture::Lying),
            "Joke" => Ok(Posture::Joke),
            "None" => Ok(Posture::None),
            _ => Err(PostureError::UnknownPosture),
        }
    }
}

impl Default for Posture {
    fn default() -> Posture {
        Posture::None
    }
}
