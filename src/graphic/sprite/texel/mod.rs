pub mod err;
pub mod part;

pub use self::err::{TexelError, Result};

use super::SPEC_MAX_XY;

use self::part::Part;
use std::borrow::BorrowMut;
use std::ops::{Add, Sub};
use std::fmt;

#[derive(Copy)]
pub struct Texel {
    part: Part, 
    count: usize,
    position: usize,
    glyph: [char; SPEC_MAX_XY],
}

impl Texel {
    pub fn new(part: &str, glyph: char) -> Result<Self> {
        if let '\u{e000}'...'\u{efff}' = glyph {
            match Part::new(part) {
                Err(why) => Err(TexelError::PartFail(why)),
                Ok(part) => Ok(
                    Texel {
                        part: part,
                        count: 1,
                        position: 0,
                        glyph: [glyph; SPEC_MAX_XY],
                    }
                ),
            }
        } else {
            Err(TexelError::ForbiddenGlyph(glyph))
        }
    }

    /// The accessor method `get_part` returns the Texel Part.
    pub fn get_part(&self) -> &Part {
        &self.part
    }

    /// The accessor method `get_glyph` returns the Texel Glyph.
    pub fn get_glyph(&self) -> char {
        *self.glyph.get(self.get_position()).unwrap()
    }

    fn get_position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }
}

impl ExactSizeIterator for Texel {
    fn len(&self) -> usize {
        self.count
    }
}


impl PartialEq for Texel {
    fn eq(&self, rhs: &Texel) -> bool {
        self.part.eq(&rhs.part)
    }
}

impl fmt::Display for Texel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_glyph())
    }
}

impl fmt::Debug for Texel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texel {{ part: {:?}, count: {}, position: {}, glyph: {:?} }}",
               self.part,
               self.count,
               self.position,
               &self.glyph[..8])
    }
}

impl Iterator for Texel {
        type Item = char;

        fn next(&mut self) -> Option<char> {
            let position: usize = self.get_position();
            if position.lt(&self.len()) {
                self.set_position(position.add(&1));
            } else {
                self.set_position(0);
            }
            self.glyph.get(position)
                      .and_then(|&glyph|
                                Some(glyph))
        }
}

impl Clone for Texel {
    fn clone(&self) -> Self {
        let mut glyph: [char; SPEC_MAX_XY] = [' '; SPEC_MAX_XY];

        glyph.copy_from_slice(&self.glyph);
        Texel {
            part: self.part, 
            count: self.count,
            position: self.position,
            glyph: glyph,
        }
    }

    fn clone_from(&mut self, source: &Texel) {
        let start: usize = self.count;
        let end: usize = SPEC_MAX_XY.sub(&start);

        self.glyph[start..].borrow_mut().copy_from_slice(&source.glyph[..end]);
        self.count = end;
    }
}
