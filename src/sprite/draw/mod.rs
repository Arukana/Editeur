mod err;

pub use self::err::{DrawError, Result};
use std::fmt;
use std::mem;

pub const SPEC_MAX_X: usize = 10;
pub const SPEC_MAX_Y: usize = 5;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;
pub const SPEC_MAX_PRE_XY: usize = SPEC_MAX_XY - 1;

use ::Cursor;

pub use super::{Emotion, EmotionError};
pub use super::{Sheet, SheetError};
pub use super::Texel;
pub use super::texel::part::Part;

use ::time;

/// Sheet is like the Sheet of the drawned persona.

#[derive(Copy)]
pub struct Draw {
    posture: Sheet,
    duration: time::Duration,
    board: Cursor<[(Emotion, Texel); SPEC_MAX_XY]>,
}

impl Draw {
    pub fn new(position: &Sheet,
               duration: i64,
               buf: &[(Emotion, Texel)])
               -> Result<Self> {
        let len: usize = buf.len();
        if len.eq(&SPEC_MAX_XY) {
            unsafe {
                let mut line: [(Emotion, Texel); SPEC_MAX_XY] =
                    mem::uninitialized();

                line.copy_from_slice(buf);
                Ok(Draw {
                    posture: *position,
                    duration: time::Duration::milliseconds(duration),
                    board: Cursor::new(line),
                })
            }
        } else {
            Err(DrawError::OutOfSize(format!("{}/{}", len, SPEC_MAX_XY)))
        }
    }

    /// The accessor method `current` returns the pointed cell.
    pub fn current(&self) -> Option<(&Emotion, &Texel)> {
        self.board
            .get_ref()
            .get(self.get_position())
            .and_then(|&(ref emotion, ref texel)| Some((emotion, texel)))
    }

    pub fn get_current_part(&self) -> Option<&Part> {
        self.current()
            .and_then(|(_, ref texel)|
                      Some(texel.get_part()))
    }

    pub fn set_current(&mut self,
                       (emotion, texels): (&Emotion, &Vec<Texel>)) {
        if let Some(texel) = texels.first() {
            let part: &Part = texel.get_part();

            self.board
                .get_mut()
                .iter_mut()
                .filter(|&&mut (_, cell_texel)| cell_texel.get_part().eq(part))
                .zip(texels.iter())
                .all(|(&mut (ref mut cell_emotion,
                             ref mut cell_texel),
                       texel):
                      (&mut (Emotion, Texel), &Texel)| {
                    cell_emotion.clone_from(emotion);
                    cell_texel.set_glyph(texel.get_glyph());
                    true
                });
        }
    }

    pub fn get_posture(&self) -> &Sheet {
        &self.posture
    }

    /// The accessor method `get_position` returns the position of
    /// the file sprite cursor.
    pub fn get_position(&self) -> usize {
        self.board.position()
    }

    /// The mutator method `set_position` changes the position of
    /// the file sprite cursor.
    pub fn set_position(&mut self, position: usize) {
        self.board.set_position(position);
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

    pub fn get_duration(&self) -> &time::Duration {
        &self.duration
    }

    pub fn set_cell_at(&mut self,
        index: usize, texel: &Texel, emotion: &Emotion
    ) {
        self.board.get_mut()
                  .iter_mut()
                  .filter(|&&mut (_, ref cur_texel)| cur_texel.eq(&texel))
                  .nth(index)
                  .and_then(|&mut (ref mut cur_emotion, ref mut cur_texel)| {
                        cur_emotion.clone_from(emotion);
                        cur_texel.clone_from(texel);
                        Some(())
                  });
    }

    pub fn next(&mut self) -> Option<(&Emotion, &Texel)> {
        let position: usize = self.get_position();
    
        self.board.set_position(position+1);
        self.board
            .get_ref()
            .get(position)
            .and_then(|&(ref emotion, ref texel)| Some((emotion, texel)))
    }
}

impl<'a> IntoIterator for &'a Draw {
    type Item = &'a (Emotion, Texel);
    type IntoIter = ::std::slice::Iter<'a, (Emotion, Texel)>;

    fn into_iter(self) -> Self::IntoIter {
        self.board.get_ref().into_iter()
    }
}

impl fmt::Debug for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Sheet: {:?}, sprite: {:?})",
               self.posture,
               self.board
                   .get_ref()
                   .iter()
                   .filter(|&&(ref emotion, _)| emotion.is_none())
                   .collect::<Vec<&(Emotion, Texel)>>())
    }
}

impl Clone for Draw {
    fn clone(&self) -> Draw {
        Draw {
            posture: self.posture,
            duration: self.duration,
            board: Cursor::new(*self.board.get_ref()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.posture.clone_from(&source.posture);
        self.duration.clone_from(&source.duration);
        self.board.get_mut().copy_from_slice(source.board.get_ref());
    }
}

impl Default for Draw {
    fn default() -> Draw {
        unsafe {
            let mut board: [(Emotion, Texel); SPEC_MAX_XY] =
                mem::uninitialized();

            assert!(board.iter_mut().all(|mut tuple| {
                *tuple = (Emotion::default(), Texel::default());
                true
            }));
            Draw {
                posture: Sheet::default(),
                duration: time::Duration::milliseconds(0),
                board: Cursor::new(board),
            }
        }
    }
}
