#![feature(slice_patterns)]
#![allow(unused_attributes, unknown_lints)]

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]

#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file="clippy.toml")))]

#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    blacklisted_name,
    unused_qualifications
)]

extern crate time;

#[macro_use]
mod macros;
pub mod sheet;
pub mod sprite;
pub mod emotion;
pub mod position;
mod err;

pub mod prelude;

use self::emotion::Emotion;
use self::position::Posture;
use self::sprite::Sprite;

pub use self::err::{GraphicError, Result};

use self::sprite::draw::{Draw, SPEC_MAX_XY};
use self::sprite::texel::Texel;
use self::sprite::texel::part::Part;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::Not;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};


/// The limit of draws by sprite.
pub const SPEC_MAX_DRAW: usize = 16;

/// The default capacity of Posture dictionary.
pub const SPEC_CAPACITY_POSITION: usize = 25;
/// The default capacity of Sprite dictionary by Posture dictionary.
pub const SPEC_CAPACITY_SPRITE: usize = 5;

/// The sub-directory texel.
pub const SPEC_SUBD_NCT: &'static str = "texel";
/// The sub-directory sprite.
pub const SPEC_SUBD_NCS: &'static str = "sprite";
/// The first directory.
pub const SPEC_ROOT: &'static str = ".neko";

#[derive(Clone, Debug)]
pub struct Graphic {
    /// Dictionary of texel.
    texel: HashMap<Posture, HashMap<(Part, Emotion), Texel>>,
    /// Dictionary of primitive's sprite.
    sprite: io::Cursor<Vec<(PathBuf, Sprite)>>,
}

impl Graphic {

    /// The constructor `new` returns a Graphic prepared with
    /// the texel and sprite root.
    pub fn new() -> Result<Self> {
        let mut manager = Graphic::default();

        manager.nct_with_ncs()
            .and_then(|(texel, sprite)|
                match (fs::read_dir(texel), fs::read_dir(sprite)) {
                    (Err(why), _) | (_, Err(why)) => Err(GraphicError::ReadDir(why)),
                    (Ok(entry_nct), Ok(entry_ncs)) => {
                        if let Some(why) = entry_nct.filter_map(|texel| texel.ok())
                                                    .filter_map(|entry|
                            manager.insert_from_texelfile(&entry.path()).err()
                        ).next() {
                                Err(why)
                        } else if let Some(why) = entry_ncs.filter_map(|sprite|
                                                               sprite.ok())
                                                           .filter_map(|entry|
                            manager.insert_from_spritefile(&entry.path()).err()
                        ).next() {
                            Err(why)
                        } else {
                            Ok(manager)
                        }
                    },
                }
            )
    }

    /// The accessor method `get_nct` returns the texel sub-directory.
    pub fn get_nct(&self) -> Result<PathBuf> {
        if let Some(mut path) = env::home_dir() {
            path.push(SPEC_ROOT);
            path.push(SPEC_SUBD_NCT);
            if let Some(why) = fs::create_dir_all(&path).err() {
                if why.kind() == io::ErrorKind::AlreadyExists {
                    Ok(path)
                } else {
                    Err(GraphicError::MkDirTexel(why))
                }
            } else {
                Ok(path)
            }
        } else {
            Err(GraphicError::Home)
        }
    }

    /// The accessor method `get_ncs` returns the sprite sub-directory.
    pub fn get_ncs(&self) -> Result<PathBuf> {
        if let Some(mut path) = env::home_dir() {
            path.push(SPEC_ROOT);
            path.push(SPEC_SUBD_NCS);
            if let Some(why) = fs::create_dir_all(&path).err() {
                if why.kind() == io::ErrorKind::AlreadyExists {
                    Ok(path)
                } else {
                    Err(GraphicError::MkDirSprite(why))
                }
            } else {
                Ok(path)
            }
        } else {
            Err(GraphicError::Home)
        }
    }

    /// The accessor method `nct_with_ncs` returns a couple
    /// of texel and sprite sub-repositories.
    pub fn nct_with_ncs(
        &mut self,
    ) -> Result<(PathBuf, PathBuf)> {
        match (self.get_nct(), self.get_ncs()) {
            (Err(why), _) | (_, Err(why)) => Err(why),
            (Ok(nct), Ok(ncs)) => Ok((nct, ncs)),
        }
    }


    /// The accessor method `get_emotion_list` returns a list of available emotions
    /// for the Posture key and Part sub-key.
    pub fn get_emotion_list(&self,
                            posture_key: &Posture,
                            part_key: &Part,
    ) -> Option<Vec<&Emotion>> {
        self.texel.get(posture_key)
            .and_then(|part_by_emotion|
                      Some(part_by_emotion.keys()
                           .filter(|&&(part, _)| part.eq(part_key))
                           .map(|&(_, ref emotion)|
                                emotion).collect::<Vec<&Emotion>>()))
    }

    pub fn get_cell_list(&self,
                            posture_key: &Posture,
                            part_key: &Part,
    ) -> Option<Vec<(&Emotion, &Texel)>> {
        self.texel.get(posture_key)
            .and_then(|part_by_emotion|
                      Some(part_by_emotion.iter()
                           .filter(|&(&(part, _), _)| part.eq(part_key))
                           .map(|(&(_, ref emotion), texel)|
                                (emotion, texel)).collect::<Vec<(&Emotion, &Texel)>>()))
    }

    /// The accessor method `get_texel` returns a reference on texel.
    pub fn get_texel(&self,
                 position: &Posture,
                 tuple: &(Part, Emotion)
    ) -> Option<&Texel> {
        self.texel.get(position).and_then(|sprite|
                      sprite.get(tuple).and_then(|texel| Some(texel)))
    }

    /// The accessor method `get_sprite` returns a reference on sprite.
    pub fn get_sprite<S: AsRef<OsStr>>(&self, name: S) -> Option<&Sprite> {
        self.sprite.get_ref().iter()
            .find(|&&(ref path, _)| path.file_stem().eq(&Some(name.as_ref())))
            .and_then(|&(_, ref sprite)| Some(sprite))
    }

    /// The function `insert_texel` insert a texel.
    fn insert_texel(&mut self,
                    (position, part, emotion): (Posture, Part, Emotion),
                    mut val: Texel,
    ) -> Option<Texel> {
        if let Some(ref texel) = self.get_texel(&position, &(part, emotion)) {
            val.clone_from(&texel);
        }
        self.texel.entry(position)
            .or_insert_with(|| HashMap::with_capacity(SPEC_CAPACITY_SPRITE))
                .insert((part, emotion), val)
    }

    /// The function `insert_sprite` insert a sprite.
    fn insert_sprite(&mut self, (file, sprite): (&Path, Sprite)) {
        self.sprite.get_mut().push((file.to_path_buf(), sprite))
    }

    fn line_with_character(
        &mut self, posture: &str, part: &str, emotion: &str, character: char
    ) -> Result<()> {
        match (Posture::new(posture),
               Texel::new(part, character),
               Emotion::new(emotion)) {
            (Err(why), _, _) => Err(GraphicError::Posture(why)),
            (_, Err(why), _) => Err(GraphicError::Texel(why)),
            (_, _, Err(why)) => Err(GraphicError::Emotion(why)),
            (Ok(posture), Ok(texel), Ok(emotion)) => {
                self.insert_texel((posture, *texel.get_part(), emotion), texel);
                Ok(())
            },
        }
    }

    fn texel_with_line(
        &mut self, line: &str
    ) -> Result<()> {
        if let Some(position) = line.find(':') {
            let (part_for_characters, emotion_and_postures) = line.split_at(position);
            let (part_for_characters, emotion_and_postures) = (
                part_for_characters.split(|c| "('\",) ".contains(c))
                    .filter(|x| x.is_empty().not())
                    .collect::<Vec<&str>>(),
                emotion_and_postures.split(|c| ":[,] ".contains(c))
                    .filter(|x| x.is_empty().not())
                    .collect::<Vec<&str>>(),
            );

            match (&part_for_characters[..], &emotion_and_postures[..]) {
                (&[part, ref characters..], &[emotion, ref postures..]) => {
                    postures.iter()
                        .filter_map(|posture: &&str|
                            characters.iter()
                                 .filter_map(|character: &&str|
                                     character.chars()
                                         .filter_map(|glyph|
                                              self.line_with_character(
                                                  posture,
                                                  part,
                                                  emotion,
                                                  glyph).err())
                                         .next())
                                 .next().and_then(|why| Some(Err(why))))
                        .next().unwrap_or_else(|| Ok(()))
                },
                _ => Err(GraphicError::SyntaxTexel(line.to_string())),
            }
        } else {
            Err(GraphicError::SyntaxTexel(line.to_string()))
        }
    }

    /// The function `from_file_texel` insert a texel from a file.
    pub fn insert_from_texelfile<S: AsRef<Path>>(&mut self, source: S) -> Result<()> {
        match fs::OpenOptions::new().read(true).open(source.as_ref()) {
            Err(why) => Err(GraphicError::OpenFile(why)),
            Ok(buffer) => {
                let reader = io::BufReader::new(buffer).lines();
                reader.map(|line: io::Result<String>|
                           match line {
                               Err(why) => Err(GraphicError::ReadFile(why)),
                               Ok(ref line) if line.is_empty() => Ok(()),
                               Ok(line) => self.texel_with_line(&line),
                           })
                    .find(|f| f.is_err())
                    .unwrap_or_else(|| Ok(()))
            }
        }
    }

    fn sprite_with_draw(
        &self, sprite: &mut Sprite, duration: &str, posture: &Posture, pairs: &&[&str],
    ) -> Result<()> {
        let mut draw: Vec<(Emotion, Texel)> = Vec::with_capacity(SPEC_MAX_XY);

        pairs.into_iter().as_slice().chunks(2).map(|pair: &[&str]|
            match (
                Part::new(pair[0]),
                Emotion::new(pair[1])
            ) {
                (Err(why), _) => Err(GraphicError::Part(why)),
                (_, Err(why)) => Err(GraphicError::Emotion(why)),
                (Ok(part), Ok(emotion)) => {
                    match self.get_texel(posture, &(part, emotion)) {
                        None => Err(GraphicError::FoundTexel(
                            format!("{}:{}", pair[0], pair[1]))
                        ),
                        Some(texel) => {
                            let position: Option<usize> =
                                draw.iter()
                                .rev()
                                .find(|&&(ref current_emotion,
                                          ref current_texel)|
                                      (current_emotion,
                                       current_texel.get_part()
                                      ).eq(&(&emotion, &part)))
                                .and_then(|&(_, ref texel)|
                                          Some(texel.get_position()));
                            if let &Some(position) = &position {
                                let mut texel: Texel = *texel;
                                texel.set_position(position+1);
                                Ok(draw.push((emotion, texel)))
                            } else {
                                Ok(draw.push((emotion, *texel)))
                            }
                        },
                    }
                },
            }
        ).find(|p| p.is_err()).unwrap_or_else(||
            match Draw::new(posture, duration.parse::<i64>().unwrap(), draw.as_slice()) {
                Ok(draw) => Ok(sprite.insert(draw)),
                Err(why) => Err(GraphicError::Draw(why)),
            })
    }

    /// The function `from_file_sprite` insert a sprite from a file.
    pub fn insert_from_spritefile<S: AsRef<OsStr> + AsRef<Path>>(
        &mut self, source: S
    ) -> Result<()> {
        let mut buffer: String = String::new();
        let mut sprite: Sprite = Sprite::default();

        match fs::OpenOptions::new().read(true).open(&source) {
            Err(why) => Err(GraphicError::OpenFile(why)),
            Ok(mut file) => {
                if let Some(why) = file.read_to_string(&mut buffer).err() {
                    Err(GraphicError::ReadFile(why))
                } else {
                    buffer.split(|c| " \n:".contains(c))
                        .filter(|x| x.is_empty().not())
                        .collect::<Vec<&str>>()
                        .as_slice()
                        .chunks(SPEC_MAX_XY*2+2)
                        .map(|sprite_and_draw|
                             sprite_and_draw.split_first()
                             .and_then(|(sprite_name, duration_and_draw)|
                                       duration_and_draw.split_first()
                                       .and_then(|(duration, draw)| Some(
                                            match Posture::new(sprite_name) {
                                                Err(why) => Err(GraphicError::Posture(why)),
                                                Ok(posture) => {
                                                    self.sprite_with_draw(
                                                        &mut sprite, duration, &posture, &draw
                                                    )
                                                },
                                            })))
                             .unwrap())
                        .find(|anim| anim.is_err())
                        .unwrap_or_else(|| Ok(
                            self.insert_sprite((source.as_ref(), sprite))))
                }
            },
        }
    }

    /// The accessor method `get_position` returns the position of
    /// the file sprite cursor.
    fn get_position(&self) -> usize {
        self.sprite.position() as usize
    }

    /// The mutator method `set_position` changes the position of
    /// the file sprite cursor.
    fn set_position(&mut self, position: usize) {
        self.sprite.set_position(position as u64);
    }

    /// The mutator method `add_position` changes the position of
    /// the file sprite cursor.
    pub fn add_position(&mut self, position: usize) {
        match (self.get_position().checked_add(position),
               self.sprite.get_ref().len()) {
            (Some(pos), len) if pos < len => self.set_position(pos),
            _ => self.set_position(0),
        }
    }

    /// The mutator method `sub_position` changes the position of
    /// the file sprite cursor.
    pub fn sub_position(&mut self, position: usize) {
        if let (Some(pos), _) = (self.get_position().checked_sub(position),
                                 self.sprite.get_ref().len()) {
            self.set_position(pos);
        }
    }

    pub fn start_position(&mut self, position: usize) {
        self.set_position(0);
        self.add_position(position);
    }

    pub fn end_position(&mut self, position: usize) {
        let len: usize = self.sprite.get_ref().len()-1;
        self.set_position(len);
        self.sub_position(position);
    }

    pub fn add_position_sprite(&mut self, position: usize) {
        let current_position: usize = self.get_position();
        self.sprite.get_mut()
            .get_mut(current_position)
            .and_then(|&mut (_, ref mut sprite)|
                      sprite.add_position(position));
    }

    pub fn sub_position_sprite(&mut self, position: usize) {
        let current_position: usize = self.get_position();
        self.sprite.get_mut()
            .get_mut(current_position)
            .and_then(|&mut (_, ref mut sprite)|
                      sprite.sub_position(position));
    }

    /// The mutator method `add_position_sprite_draw` changes the position of
    /// the cell board cursor.
    pub fn add_position_sprite_draw(&mut self, position: usize) {
        let current_position: usize = self.get_position();
        self.sprite.get_mut()
            .get_mut(current_position)
            .and_then(|&mut (_, ref mut sprite)|
                      sprite.add_position_draw(position));
    }

    /// The mutator method `sub_position_sprite_draw` changes the position of
    /// the cell board cursor.
    pub fn sub_position_sprite_draw(&mut self, position: usize) {
        let current_position: usize = self.get_position();
        self.sprite.get_mut()
            .get_mut(current_position)
            .and_then(|&mut (_, ref mut sprite)|
                      sprite.sub_position_draw(position));
    }

    pub fn get_current_cell_number(&self, index: usize) -> Option<(Emotion, Texel)> {
        self.get_current_sprite()
            .and_then(|&(_, ref sprite)|
                      sprite.get_posture()
                      .and_then(|posture|
                                sprite.current()
                                .and_then(|(_, texel)|
                                          self.get_cell_list(posture, texel.get_part())
                                          .and_then(|emotions|
                                                    emotions.get(index)
                                                    .and_then(|&(emotion, texel)|
                                                              Some((*emotion, *texel)))))))
    }

    pub fn set_current_emotion(&mut self, index: usize) {
        let position: usize = self.get_position();
        let cell: Option<(Emotion, Texel)> = self.get_current_cell_number(index);

        cell.and_then(|(ref emotion, ref texel)|
                      self.sprite.get_mut()
                      .get_mut(position)
                      .and_then(|&mut (_, ref mut sprite)|
                                sprite.set_current((emotion, texel))));
    }


    /// The accessor method `get_sprite` returns a reference on sprite.
    pub fn get_current_sprite(&self) -> Option<&(PathBuf, Sprite)> {
        self.sprite.get_ref().get(self.get_position())
    }
}

/// A trait for giving a type a useful default value.
impl Default for Graphic {

    /// The constructor `default` returns a empty Graphic.
    fn default() -> Graphic {
        Graphic {
            texel: HashMap::with_capacity(SPEC_CAPACITY_POSITION),
            sprite: io::Cursor::new(Vec::with_capacity(SPEC_CAPACITY_SPRITE)),
        }
    }
}
