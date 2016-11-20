mod err;

pub use self::err::{DrawError, Result};
use std::fmt;
use std::io;
use std::mem;

pub const SPEC_MAX_X: usize = 7;
pub const SPEC_MAX_Y: usize = 10;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;
pub const SPEC_MAX_PRE_XY: usize = SPEC_MAX_XY - 1;

pub use super::{Emotion, EmotionError};
pub use super::{Posture, PostureError};
pub use super::Texel;

/// Posture is like the Posture of the drawned persona.
pub struct Draw {
    posture: Posture,
    board: io::Cursor<[(Emotion, Texel); SPEC_MAX_XY]>,
}

impl Draw {
    pub fn new(position: &Posture,
               buf: &[(Emotion, Texel)])
               -> Result<Self> {
        if SPEC_MAX_XY.eq(&buf.len()) {
            unsafe {
                let mut line: [(Emotion, Texel); SPEC_MAX_XY] =
                    mem::uninitialized();

                line.copy_from_slice(buf);
                Ok(Draw {
                    posture: *position,
                    board: io::Cursor::new(line),
                })
            }
        } else {
            Err(DrawError::OutOfSize)
        }
    }

    /// The accessor method `current` returns the pointed cell.
    pub fn current(&self) -> Option<(&Emotion, &Texel)> {
        self.board
            .get_ref()
            .get(self.get_position())
            .and_then(|&(ref emotion, ref texel)| Some((emotion, texel)))
    }

    pub fn set_current(&mut self,
                       (emotion, texel): (&Emotion, &Texel))
                       -> Option<()> {
        let position: usize = self.get_position();
        self.board
            .get_mut()
            .get_mut(position)
            .and_then(|&mut (ref mut cell_emotion, ref mut cell_texel)| {
                cell_emotion.clone_from(emotion);
                cell_texel.clone_from(texel);
                Some(())
            })
    }

    pub fn get_posture(&self) -> &Posture {
        &self.posture
    }

    /// The accessor method `get_position` returns the position of
    /// the file sprite cursor.
    fn get_position(&self) -> usize {
        self.board.position() as usize
    }

    /// The mutator method `set_position` changes the position of
    /// the file sprite cursor.
    fn set_position(&mut self, position: usize) {
        self.board.set_position(position as u64);
    }

    /// The mutator method `add_position` increments the position of
    /// the file sprite cursor.
    pub fn add_position(&mut self, position: usize) -> Option<()> {
        if let Some(pos @ 0...SPEC_MAX_PRE_XY) = self.get_position()
            .checked_add(position) {
            Some(self.set_position(pos))
        } else {
            None
        }
    }

    /// The mutator method `sub_position` decrements the position of
    /// the file sprite cursor.
    pub fn sub_position(&mut self, position: usize) -> Option<()> {
        self.get_position()
            .checked_sub(position)
            .and_then(|pos| Some(self.set_position(pos)))
    }
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let position: (usize, usize) =
            (self.get_position().checked_rem(SPEC_MAX_X).unwrap_or(0),
             self.get_position() / SPEC_MAX_X);
        write!(f,
               "{}",
               self.board
                   .get_ref()
                   .chunks(SPEC_MAX_X)
                   .enumerate()
                   .map(|(y, cells)| {
                format!("{} {} {}\n\r",
                        cells.iter()
                            .enumerate()
                            .map(|(x, &(_, texel))| {
                                format_cell!(texel.get_glyph(),
                                             position,
                                             x,
                                             y)
                            })
                            .collect::<String>(),
                        cells.iter()
                            .enumerate()
                            .map(|(x, &(_, texel))| {
                                format_cell!(texel.get_part(), position, x, y)
                            })
                            .collect::<String>(),
                        cells.iter()
                            .enumerate()
                            .map(|(x, &(emotion, _))| {
                                format_cell!(emotion, position, x, y)
                            })
                            .collect::<String>())
            })
                   .collect::<String>())
    }
}

impl fmt::Debug for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(Posture: {:?}, sprite: {:?})",
               self.posture,
               self.board
                   .get_ref()
                   .iter()
                   .filter(|&&(ref emotion, _)| emotion.is_none())
                   .collect::<Vec<&(Emotion, Texel)>>())
    }
}

impl<'a> IntoIterator for &'a Draw {
    type Item = &'a (Emotion, Texel);
    type IntoIter = ::std::slice::Iter<'a, (Emotion, Texel)>;

    fn into_iter(self) -> Self::IntoIter {
        self.board.get_ref().into_iter()
    }
}

impl Clone for Draw {
    fn clone(&self) -> Draw {
        Draw {
            posture: self.posture,
            board: io::Cursor::new(*self.board.get_ref()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.posture = source.posture;
        self.board.get_mut().copy_from_slice(source.board.get_ref());
    }
}
