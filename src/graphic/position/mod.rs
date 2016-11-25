mod err;

pub use self::err::{PostureError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Posture {
    Bust,
    Lying,
    Seiza,
    None,
}

impl Posture {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "Bust" => Ok(Posture::Bust),
            "Lying" => Ok(Posture::Lying),
            "Seiza" => Ok(Posture::Seiza),
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
