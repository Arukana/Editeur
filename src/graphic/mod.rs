pub mod sprite;
pub mod emotion;
pub mod position;
mod err;

use self::emotion::Emotion;
use self::position::Position;
use self::sprite::Sprite;

pub use self::err::{GraphicError, Result};

use self::sprite::draw::{Draw, SPEC_MAX_XY, SPEC_MAX_X};
use self::sprite::texel::Texel;
use self::sprite::texel::part::Part;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::fmt;
use std::io;
use std::env;
use std::io::prelude::*;
use std::ops::Not;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use ::SPEC_ROOT;

/// The default capacity of Position dictionary.
const SPEC_CAPACITY_POSITION: usize = 25;
/// The default capacity of Sprite dictionary by Position dictionary.
const SPEC_CAPACITY_SPRITE: usize = 5;
/// The sub-directory texel.
const SPEC_SUBD_NCT: &'static str = "nct";
/// The sub-directory sprite.
const SPEC_SUBD_NCS: &'static str = "ncs";

#[derive(Clone, Debug)]
pub struct Graphic {
    /// Dictionary of texel.
    texel: HashMap<Position, HashMap<(Part, Emotion), Texel>>,
    /// Dictionary of sprite.
    sprite: Vec<(PathBuf, (Position, Sprite))>,
    /// Index of file.
    position: usize,
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
    pub fn nct_with_ncs (
        &mut self,
    ) -> Result<(PathBuf, PathBuf)> {
        match (self.get_nct(), self.get_ncs()) {
            (Err(why), _) | (_, Err(why)) => Err(why),
            (Ok(nct), Ok(ncs)) => Ok((nct, ncs)),
        }
    }

    fn get_texel(&self,
                 position: &Position,
                 tuple: &(Part, Emotion))
                 -> Option<Texel> {
        self.texel.get(position).and_then(|sprite|
            sprite.get(tuple).and_then(|texel| Some(*texel))
        )
    }

    /// The function `insert_texel` insert a texel.
    fn insert_texel(&mut self,
                    (position, part, emotion): (Position, Part, Emotion),
                    val: Texel)
                    -> Option<Texel> {
        self.texel.entry(position)
            .or_insert_with(|| HashMap::with_capacity(SPEC_CAPACITY_SPRITE))
            .insert((part, emotion), val)
    }

    /// The function `insert_sprite` insert a sprite.
    fn insert_sprite(&mut self, (file, sprite): (&Path, (Position, Sprite))) {
        self.sprite.push((file.to_path_buf(), sprite))
    }

    fn line_with_character(
        &mut self, content: &str, pt: &str, emotion: &str, character: &str
    ) -> Result<()> {
        match (Position::new(content),
               Part::new(pt),
               Emotion::new(emotion),
               character.chars().next()) {
            (Err(why), _, _, _) => Err(GraphicError::Position(why)),
            (_, Err(why), _, _) => Err(GraphicError::Part(why)),
            (_, _, Err(why), _) => Err(GraphicError::Emotion(why)),
            (_, _, _, None) => Err(GraphicError::Glyph),
            (Ok(position), Ok(part), Ok(emotion), Some(glyph)) => {
                match Texel::new(pt, glyph) {
                    Err(why) => Err(GraphicError::Texel(why)),
                    Ok(texel) => {
                        self.insert_texel((position, part, emotion), texel);
                        Ok(())
                    },
                }
            },
        }
    }

    fn texel_with_line(
        &mut self, line: &str
    ) -> Result<()> {
        let words: Vec<&str> = line.split(|c| "('): [,]".contains(c))
            .filter(|x| x.is_empty().not())
            .collect::<Vec<&str>>();
        match &words[..] {
            &[pt, character, emotion, ref positions..] => {
                if let Some(why) = positions.iter()
                                            .filter_map(|content: &&str|
                        self.line_with_character(content, pt, emotion, character).err()
                ).next() {
                    Err(why)
                } else {
                    Ok(())
                }
            }
            _ => unimplemented!(),
        }
    }

    /// The function `from_file_texel` insert a texel from a file.
    pub fn insert_from_texelfile<S: AsRef<Path>>(&mut self, source: S) -> Result<()> {
        match fs::OpenOptions::new().read(true).open(source.as_ref()) {
            Err(why) => Err(GraphicError::OpenFile(why)),
            Ok(buffer) => {
                let reader = io::BufReader::new(buffer).lines();
                if let Some(why) = reader.filter_map(|line: io::Result<String>|
                       match line {
                           Err(why) => Err(GraphicError::ReadFile(why)),
                           Ok(line) => self.texel_with_line(&line),
                       }.err()
                ).next() {
                    Err(why)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn sprite_with_draw(
        &self, sprite: &mut Sprite, position: &Position, words: &VecDeque<&str>,
    ) -> Result<()> {
        let mut draw: Vec<(Emotion, Texel)> = Vec::with_capacity(SPEC_MAX_XY);
        let pairs = words.as_slices().0.iter().take(
            SPEC_MAX_XY * 2
        ).collect::<Vec<&&str>>();

        if let Some(why) = pairs.chunks(2).filter_map(|pair: &[&&str]|
            match (
                Part::new(pair[0]),
                Emotion::new(pair[1])
            ) {
                (Err(why), _) => Err(GraphicError::Part(why)),
                (_, Err(why)) => Err(GraphicError::Emotion(why)),
                (Ok(part), Ok(emotion)) => {
                    match self.get_texel(position, &(part, emotion)) {
                        Some(texel) => Ok(draw.push((emotion, texel))),
                        None => Err(GraphicError::FoundTexel),
                    }
                },
            }.err()
        ).next() {
            Err(why)
        } else {
            match Draw::new(position, draw.as_slice()) {
                Ok(draw) => Ok(sprite.insert(draw)),
                Err(why) => Err(GraphicError::Draw(why)),
            }
        }
    }

    /// The function `from_file_sprite` insert a sprite from a file.
    pub fn insert_from_spritefile<S: AsRef<OsStr> + AsRef<Path>>(
        &mut self, source: S
    ) -> Result<()> {
        let mut sprite: Sprite = Sprite::default();
        let mut buffer = String::new();

        match fs::OpenOptions::new().read(true).open(&source) {
            Err(why) => Err(GraphicError::OpenFile(why)),
            Ok(mut file) => {
                if let Some(why) = file.read_to_string(&mut buffer).err() {
                    Err(GraphicError::ReadFile(why))
                } else {
                    let mut words: VecDeque<&str> = buffer
                        .split(|c| " \n:".contains(c))
                        .filter(|x| x.is_empty().not())
                        .collect::<VecDeque<&str>>();
                    match Position::new(words.pop_front().unwrap()) {
                        Err(why) => Err(GraphicError::Position(why)),
                        Ok(position) => {
                            self.sprite_with_draw(
                                &mut sprite, &position, &words
                            ).and_then(|()|
                                Ok(self.insert_sprite((source.as_ref(), (position, sprite))))
                            )
                        },
                    }
                }
            }
        }
    }

    /// The mutator method `add_position` changes the position of
    /// the file sprite cursor.
    pub fn add_position(&mut self, position: usize) {
        if let Some(pos) = self.position.checked_add(position) {
                if pos < self.sprite.len() {
                self.position = pos;
            }
        }
    }

    /// The mutator method `sub_position` changes the position of
    /// the file sprite cursor.
    pub fn sub_position(&mut self, position: usize) {
        if let Some(pos) = self.position.checked_sub(position) {
            self.position = pos;
        }
    }

    /// The mutator method `add_position_sprite_draw` changes the position of
    /// the cell board cursor.
    pub fn add_position_sprite_draw(&mut self, position: usize) {
        if let Some(&mut (_, (_, ref mut sprite))) = self.sprite.get_mut(self.position) {
            sprite.add_position_draw(position);
        }
    }

    /// The mutator method `sub_position_sprite_draw` changes the position of
    /// the cell board cursor.
    pub fn sub_position_sprite_draw(&mut self, position: usize) {
        if let Some(&mut (_, (_, ref mut sprite))) = self.sprite.get_mut(self.position) {
            sprite.sub_position_draw(position);
        }
    }

    pub fn get_cell(&self, position: usize) -> Option<(&(Part, Emotion), &Texel)> {
        self.sprite.get(self.position).and_then(|&(_, (ref posture, _))|
            self.texel.get(posture).and_then(|texels|
                texels.iter().collect::<Vec<_>>().get(position).and_then(|&cell|
                    Some(cell)
                )
            )
        )
    }

    pub fn set_cell_draw(&mut self, position: usize) {
    }
}

impl fmt::Display for Graphic {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self.sprite.get(self.position) {
            None => "\n\r".to_string(),
            Some(&(ref path, (ref posture, ref sprite))) => {
                let current: Option<(&Emotion, &Texel)> = sprite.current();
                format!("{:?}\n\r{}{}",
                        path.file_stem().unwrap_or_default(),
                        sprite,
                        self.texel.get(posture)
                        .and_then(|texels|
                                  Some(texels.iter()
                                       .collect::<Vec<(&(Part, Emotion), &Texel)>>()
                                       .as_slice()
                                       .chunks(SPEC_MAX_X)
                                       .map(|line|
                                           format!("{}{}{}",
                                              line.iter()
                                                   .map(|&(&(_, ref emotion), ref texel)|
                                                        format_cell!(texel, current, Some((&emotion, &texel)))
                                                   ).collect::<String>(),
                                              line.iter()
                                                   .map(|&(&(_, ref emotion), ref texel)|
                                                        format_cell!(texel, current, Some((&emotion, &texel)))
                                                   ).collect::<String>(),
                                              line.iter()
                                                  .map(|&(&(_, ref emotion), ref texel)|
                                                        format_cell!(texel, current, Some((&emotion, &texel)))
                                                   ).collect::<String>()
                                           )
                                       )
                                       .collect::<String>())
                        ).unwrap_or_default())
            },
        })
    }
}

/// A trait for giving a type a useful default value.
impl Default for Graphic {

    /// The constructor `default` returns a empty Graphic.
    fn default() -> Graphic {
        Graphic {
            texel: HashMap::with_capacity(SPEC_CAPACITY_POSITION),
            sprite: Vec::with_capacity(SPEC_CAPACITY_SPRITE),
            position: 0,
        }
    }
}
