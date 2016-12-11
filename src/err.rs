use std::error::Error;
use std::fmt;
use std::io;
use std::env;

use super::sheet::SheetError;
use super::position::PostureError;
use super::emotion::EmotionError;
use super::sprite::draw::DrawError;
use super::sprite::texel::TexelError;
use super::sprite::texel::part::PartError;

pub type Result<T> = ::std::result::Result<T, GraphicError>;

/// The enum `GraphicError` defines the possible errors
/// from constructor Graphic.
#[derive(Debug)]
pub enum GraphicError {
    /// Can't read the sub-directory.
    ReadDir(io::Error),
    /// Can't open the file.
    OpenFile(io::Error),
    /// Can't read the file.
    ReadFile(io::Error),
    /// Can't create the texel sub-directory.
    MkDirTexel(io::Error),
    /// Can't create the sprite sub-directory.
    MkDirSprite(io::Error),
    /// The Posture interface has meet an error.
    Posture(PostureError),
    /// The Draw interface has meet an error.
    Draw(DrawError),
    /// The Emotion interface has meet an error.
    Emotion(EmotionError),
    /// The Texel interface has meet an error.
    Texel(TexelError),
    /// Can't split the chunk of sprite.
    SpriteSplitFirst(String),
    /// The Part interface has meet an error.
    Part(PartError),
    /// The Sheet interface has meet an error.
    Sheet(SheetError),
    /// Can't found the NEKO_PATH environement variable.
    NekoPath,
    /// Can't found the glyph of texel.
    Glyph,
    /// Can't found the texel.
    FoundTexel(String),
    /// Unvalid texel syntax.
    SyntaxTexel(String),
}

impl fmt::Display for GraphicError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
    }
}

impl Error for GraphicError {
    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            GraphicError::ReadDir(_) => "Can't read the sub-directory.",
            GraphicError::OpenFile(_) => "Can't open the file.",
            GraphicError::ReadFile(_) => "Can't read the file.",
            GraphicError::MkDirTexel(_) => "Can't create the texel sub-directory.",
            GraphicError::MkDirSprite(_) => "Can't create the sprite sub-directory.",
            GraphicError::Posture(_) => "The Posture interface has meet an error.",
            GraphicError::Draw(_) => "The Draw interface has meet an error.",
            GraphicError::Emotion(_) => "The Emotion interface has meet an error.",
            GraphicError::Texel(_) => "The Texel interface has meet an error.",
            GraphicError::Part(_) => "The Part interface has meet an error.",
            GraphicError::Sheet(_) => "The Sheet interface has meet an error.",
            GraphicError::NekoPath => "Can't found the $NEKO_PATH environement variable.",
            GraphicError::Glyph => "Can't found the glyph of texel.",
            GraphicError::SpriteSplitFirst(ref name) => name,
            GraphicError::FoundTexel(ref name) => name,
            GraphicError::SyntaxTexel(ref name) => name,
        }
    }

  /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            GraphicError::ReadDir(ref why) |
            GraphicError::OpenFile(ref why) |
            GraphicError::ReadFile(ref why) |
            GraphicError::MkDirTexel(ref why) |
            GraphicError::MkDirSprite(ref why) => Some(why),
            GraphicError::Posture(ref why) => Some(why),
            GraphicError::Draw(ref why) => Some(why),
            GraphicError::Emotion(ref why) => Some(why),
            GraphicError::Texel(ref why) => Some(why),
            GraphicError::Part(ref why) => Some(why),
            _ => None,
        }
    }
}

impl From<env::VarError> for GraphicError {
    fn from(_: env::VarError) -> GraphicError {
        GraphicError::NekoPath
    }
}
