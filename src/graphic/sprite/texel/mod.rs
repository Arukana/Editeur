pub mod err;
pub mod part;

pub use self::err::{TexelError, Result};

use self::part::Part;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Texel(pub Part, pub char);

impl Texel {
    pub fn new(part: &str, glyph: char) -> Result<Self> {
        if let '\u{e000}'...'\u{efff}' = glyph {
            match Part::new(part) {
                Ok(part) => Ok(Texel(part, glyph)),
                Err(why) => Err(TexelError::PartFail(why)),
            }
        } else {
            Err(TexelError::ForbiddenGlyph(glyph))
        }
    }

    /// The accessor method `get_part` returns the Texel Part.
    pub fn get_part(&self) -> &Part {
        &self.0
    }

    /// The accessor method `get_glyph` returns the Texel Glyph.
    pub fn get_glyph(&self) -> &char {
        &self.1
    }
}

impl PartialEq for Texel {
    fn eq(&self, rhs: &Texel) -> bool {
        self.0.eq(&rhs.0)
    }
}

impl fmt::Display for Texel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}
