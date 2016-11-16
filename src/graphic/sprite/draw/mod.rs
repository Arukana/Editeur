mod err;

pub use self::err::{DrawError, Result};
use std::fmt;
use std::mem;

use ::termion::color;

pub const SPEC_MAX_X: usize = 7;
pub const SPEC_MAX_BX: usize = SPEC_MAX_X-1;
pub const SPEC_MAX_Y: usize = 10;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;

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
                Ok(Draw{
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
        if let Some(pos @ 0...SPEC_MAX_XY) = self.position.checked_add(position) {
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
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(why) = self.board.iter().enumerate()
                                            .filter_map(|(index, &(_, ref texel))|
            match (self.position.eq(&index),
                   index.checked_rem(SPEC_MAX_X).eq(&Some(SPEC_MAX_BX))) {
                (true, true) => write!(f, "{}{}{}\n\r",
                                       color::Bg(color::Cyan),
                                       texel,
                                       color::Bg(color::Reset)),
                (true, false) => write!(f, "{}{}{}",
                                       color::Bg(color::Cyan),
                                       texel,
                                       color::Bg(color::Reset)),
                (false, true) => write!(f, "{}\n\r", texel),
                (false, false) => write!(f, "{}", texel),
            }.err()
        ).next() {
            Err(why)
        } else {
            Ok(())
        }
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
