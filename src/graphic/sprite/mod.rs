pub mod texel;
pub mod draw;
mod err;

use std::fmt;

use self::draw::Draw;
pub use self::err::{SpriteError, Result};
pub use self::texel::Texel;
pub use super::emotion::{Emotion, EmotionError};
pub use super::position::{Position, PositionError};

const SPEC_CAPACITY_SHEET: usize = 5;

#[derive(Clone, Debug)]
pub struct Sprite {
    sheet: Vec<Draw>,
    position: usize,
}

impl Sprite {
    pub fn insert(&mut self, draw: Draw) {
        self.sheet.push(draw);
    }

    pub fn current(&self) -> Option<(&Emotion, &Texel)> {
        self.sheet.get(self.position).and_then(|draw| draw.current())
    }

    pub fn set_current(
        &mut self,
        cell: (&Emotion, &Texel)
    ) -> Option<()> {
        self.sheet
            .get_mut(self.position)
            .and_then(|board|
                      board.set_current(cell))
    }

    pub fn get_posture(&self) -> Option<&Position> {
        self.sheet.get(self.position).and_then(|draw| Some(draw.get_posture()))
    }

    /// The mutator method `add_position_draw` changes the position of
    /// the cell board cursor.
    pub fn add_position_draw(&mut self, position: usize) {
        if let Some(ref mut draw) = self.sheet.get_mut(self.position) {
            draw.add_position(position);
        }
    }

    /// The mutator method `sub_position_draw` changes the position of
    /// the cell board cursor.
    pub fn sub_position_draw(&mut self, position: usize) {
        if let Some(ref mut draw) = self.sheet.get_mut(self.position) {
            draw.sub_position(position);
        }
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sheet.get(self.position) {
            Some(sheet) => write!(f, "{}", sheet),
            None => write!(f, "\n\r"),
        }
    }
}

impl Default for Sprite {
    fn default() -> Sprite {
        Sprite {
            sheet: Vec::with_capacity(SPEC_CAPACITY_SHEET),
            position: 0,
        }
    }
}
