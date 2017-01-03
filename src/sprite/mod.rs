pub mod texel;
pub mod draw;
mod err;

use std::collections::HashMap;
use std::io;
use std::usize;
use std::mem;

pub use self::draw::SPEC_MAX_XY;

use self::draw::Draw;
pub use self::texel::Texel;

pub use self::err::{SpriteError, Result};
pub use super::tuple::Tuple;
pub use super::Part;
pub use super::emotion::{Emotion, EmotionError};
pub use super::position::{Posture, PostureError};

/// The limit of draws by sprite.
pub const SPEC_MAX_DRAW: usize = 16;

#[derive(Debug)]
pub struct Sprite {
    texel: HashMap<(Part, Emotion), Vec<Texel>>,
    sheet: io::Cursor<[Draw; SPEC_MAX_DRAW]>,
    count: usize,
}

impl Sprite {

    pub fn explicite_emotion(&mut self,
        change: &[[Tuple; SPEC_MAX_XY]; SPEC_MAX_DRAW]
    ) {
        let board: Vec<Vec<(Emotion, Vec<Texel>)>> =
            change.iter().map(|tuples: &[Tuple; SPEC_MAX_XY]| {
                 tuples.iter().filter_map(|tuple| {
                      self.texel.get(&(tuple.part, tuple.emotion))
                          .and_then(|texels| Some((tuple.emotion, texels.clone())))
                 })
                 .collect::<Vec<(Emotion, Vec<Texel>)>>()
            })
            .collect::<Vec<Vec<(Emotion, Vec<Texel>)>>>();

        self.sheet.get_mut()
            .iter_mut()
            .zip(board.iter())
            .all(|(draw, tuple): (&mut Draw, &Vec<(Emotion, Vec<Texel>)>)|
                tuple.iter()
                     .all(|&(emotion, ref texels): &(Emotion, Vec<Texel>)|
                        texels.iter()
                              .enumerate()
                              .all(|(index, texel): (usize, &Texel)| {
                                    draw.set_cell_at(index, texel, &emotion);
                                    true
                              })
                     ));
    }

    /// The function `insert_list` push a new draw from a list of
    /// tuple of emotion by part.
    pub fn insert_list(&mut self,
        duration: i64,
        posture: &Posture,
        source: &[(Part, Emotion)],
    ) {
        let mut draw: Vec<(Emotion, Texel)> = Vec::with_capacity(SPEC_MAX_XY);
    
        source.iter().all(|&(part, emotion): &(Part, Emotion)| {
           self.texel.get(&(part, emotion)).and_then(|texels: &Vec<Texel>| {
                let index: usize = draw.iter().filter(|&&(_, ref texel)| {
                    texel.get_part().eq(&part)
                }).count();
                Some(draw.push((emotion, *texels.get(index).unwrap())))
            }).is_some()
        });
        if let Ok(draw) = Draw::new(posture, duration, draw.as_slice()) {
            unsafe {
                *self.sheet.get_mut()
                     .get_unchecked_mut(self.count) = draw;
                self.count += 1;
            }
        }
    }

    /// The function `extend` extends the local dictionary of texel.
    pub fn extend(&mut self,
                 texels: &HashMap<(Part, Emotion), Vec<Texel>>
    ) {
        texels.iter()
               .all(|(&(part, emotion), value):
                     (&(Part, Emotion), &Vec<Texel>)|
                    self.texel.insert((part, emotion),
                                      value.clone())
                              .is_none());
    }

    pub fn current(&self) -> Option<(&Emotion, &Texel)> {
        self.sheet
            .get_ref()
            .get(self.get_position())
            .and_then(|draw| draw.current())
    }

    pub fn set_current(&mut self, cell: (&Emotion, &Vec<Texel>)) -> Option<()> {
        let position: usize = self.get_position();
        self.sheet
            .get_mut()
            .get_mut(position)
            .and_then(|board| Some(board.set_current(cell)))
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

impl Clone for Sprite {
       fn clone(&self) -> Sprite {
            unsafe {
                let mut sheet: [Draw; SPEC_MAX_DRAW] = mem::uninitialized();

                sheet.clone_from_slice(self.sheet.get_ref());
                Sprite {
                    texel: self.texel.clone(),
                    sheet: io::Cursor::new(sheet),
                    count: self.count,
                }
            }
       }
}

impl<'a> IntoIterator for &'a Sprite {
    type Item = &'a Draw;
    type IntoIter = ::std::slice::Iter<'a, Draw>;

    fn into_iter(self) -> Self::IntoIter {
        self.sheet.get_ref().split_at(self.count).0.into_iter()
    }
}

impl Default for Sprite {
    fn default() -> Sprite {
        unsafe {
            let mut sheet: [Draw; SPEC_MAX_DRAW] = mem::uninitialized();

            assert!(sheet.iter_mut().all(|mut draw| {
                *draw = Draw::default();
                true
            }));
            Sprite {
                texel: HashMap::with_capacity(SPEC_MAX_XY),
                sheet: io::Cursor::new(sheet),
                count: 0,
            }
        }
    }
}
