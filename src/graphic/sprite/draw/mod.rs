mod err;

pub use self::err::{DrawError, Result};
use std::fmt;
use std::mem;

pub const SPEC_MAX_X: usize = 7;
pub const SPEC_MAX_Y: usize = 10;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;
pub const SPEC_MAX_PRE_XY: usize = SPEC_MAX_XY-1;

pub use super::{Emotion, EmotionError};
pub use super::{Position, PositionError};
pub use super::Texel;

/// Position is like the Posture of the drawned persona.
#[derive(Copy)]
pub struct Draw {
    posture: Position,
    board: [(Emotion, Texel); SPEC_MAX_XY],
    position: usize,
}

impl Draw {
    pub fn new(position: &Position, buf: &[(Emotion, Texel)]) -> Result<Self> {
        if SPEC_MAX_XY.eq(&buf.len()) {
            unsafe {
                let mut line: [(Emotion, Texel); SPEC_MAX_XY] = mem::uninitialized();

                line.copy_from_slice(buf);
                Ok(Draw {
                    posture: *position,
                    board: line,
                    position: 0,
                })
            }
        } else {
            Err(DrawError::OutOfSize)
        }
    }

    /// The mutator method `add_position` changes the position of
    /// the file sprite cursor.
    pub fn add_position(&mut self, position: usize) {
        if let Some(pos @ 0...SPEC_MAX_PRE_XY) = self.position.checked_add(position) {
            self.position = pos;
        }
    }

    /// The mutator method `sub_position` changes the position of
    /// the file sprite cursor.
    pub fn sub_position(&mut self, position: usize) {
        if let Some(pos) = self.position.checked_sub(position) {
            self.position = pos;
        }
    }

    /// The accessor method `current` returns the pointed cell.
    pub fn current(&self) -> Option<(&Emotion, &Texel)> {
        self.board
            .get(self.position)
            .and_then(|&(ref emotion, ref texel)| Some((emotion, texel)))
    }
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let position: (usize, usize) = (
            self.position.checked_rem(SPEC_MAX_X).unwrap_or(0),
            self.position/SPEC_MAX_X
        );
        write!(f, "{}",
               self.board
               .chunks(SPEC_MAX_X)
               .enumerate()
               .map(|(y, cells)|
                   format!("{} {} {}\n\r",
                           cells.iter()
                           .enumerate()
                           .map(|(x, &(_, texel))|
                                format_cell!(texel.get_glyph(), position, x, y))
                           .collect::<String>(),
                           cells.iter()
                           .enumerate()
                           .map(|(x, &(_, texel))|
                                format_cell!(texel.get_part(), position, x, y))
                           .collect::<String>(),
                           cells.iter()
                           .enumerate()
                           .map(|(x, &(emotion, _))|
                                format_cell!(emotion, position, x, y))
                           .collect::<String>()))
               .collect::<String>()
        )
    }
}

impl fmt::Debug for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Position: {:?}, sprite: {:?})",
               self.posture,
               self.board.iter().collect::<Vec<&(Emotion, Texel)>>())
    }
}

impl Clone for Draw {
    fn clone(&self) -> Draw {
        *self
    }
}
