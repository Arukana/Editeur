pub mod texel;
pub mod draw;
mod err;

pub use self::draw::SPEC_MAX_XY;
use self::draw::Draw;
pub use self::err::{SpriteError, Result};
pub use self::texel::Texel;
use std::io;
use std::usize;
pub use super::emotion::{Emotion, EmotionError};
pub use super::position::{Posture, PostureError};

const SPEC_CAPACITY_SHEET: usize = 5;

#[derive(Clone, Debug)]
pub struct Sprite {
    sheet: io::Cursor<Vec<Draw>>,
}

impl Sprite {
    pub fn insert(&mut self, draw: Draw) {
        self.sheet.get_mut().push(draw);
    }

    pub fn current(&self) -> Option<(&Emotion, &Texel)> {
        self.sheet
            .get_ref()
            .get(self.get_position())
            .and_then(|draw| draw.current())
    }

    pub fn set_current(&mut self, cell: (&Emotion, &Texel)) -> Option<()> {
        let position: usize = self.get_position();
        self.sheet
            .get_mut()
            .get_mut(position)
            .and_then(|board| board.set_current(cell))
    }

    pub fn get_posture(&self) -> Option<&Posture> {
        self.sheet
            .get_ref()
            .get(self.get_position())
            .and_then(|draw| Some(draw.get_posture()))
    }

    pub fn get_current_draw(&self) -> Option<&Draw> {
        self.sheet.get_ref().get(self.get_position())
    }

    /// The accessor method `get_position` returns the position of
    /// the draw sheet cursor.
    fn get_position(&self) -> usize {
        self.sheet.position() as usize
    }

    /// The mutator method `set_position` changes the position of
    /// the file sprite cursor.
    fn set_position(&mut self, position: usize) {
        self.sheet.set_position(position as u64);
    }

    /// The mutator method `add_position_draw` increments the position of
    /// the draw sheet cursor.
    pub fn add_position(&mut self, position: usize) -> Option<()> {
        match (self.get_position().checked_add(position),
               self.sheet.get_ref().len()) {
            (Some(pos), len) if pos < len => Some(self.set_position(pos)),
            _ => None,
        }
    }

    /// The mutator method `sub_position` decrements the position of
    /// the draw sheet cursor.
    pub fn sub_position(&mut self, position: usize) -> Option<()> {
        self.get_position()
            .checked_sub(position)
            .or_else(|| self.sheet.get_ref().len().checked_sub(1))
            .and_then(|pos| Some(self.set_position(pos)))
    }

    /// The mutator method `add_position_draw` increments the position of
    /// the cell board cursor.
    pub fn add_position_draw(&mut self, position: usize) -> Option<()> {
        let current_position: usize = self.get_position();
        self.sheet
            .get_mut()
            .get_mut(current_position)
            .and_then(|ref mut draw|
                      draw.add_position(position))
            .or_else(|| self.add_position(1))
    }

    /// The mutator method `sub_position_draw` decrements the position of
    /// the cell board cursor.
    pub fn sub_position_draw(&mut self, position: usize) -> Option<()> {
        let current_position: usize = self.get_position();
        self.sheet
            .get_mut()
            .get_mut(current_position)
            .and_then(|ref mut draw| draw.sub_position(position))
            .or_else(|| self.sub_position(1))
    }
}

impl<'a> IntoIterator for &'a Sprite {
    type Item = &'a Draw;
    type IntoIter = ::std::slice::Iter<'a, Draw>;

    fn into_iter(self) -> Self::IntoIter {
        self.sheet.get_ref().into_iter()
    }
}

impl Default for Sprite {
    fn default() -> Sprite {
        Sprite {
            sheet: io::Cursor::new(Vec::with_capacity(SPEC_CAPACITY_SHEET)),
        }
    }
}
