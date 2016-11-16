pub mod err;
pub mod part;

pub use self::err::{TexelError, Result};

use self::part::Part;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Texel(Part, char);

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
