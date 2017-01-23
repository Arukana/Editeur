use std::error::Error;
use std::fmt;
use std::io;

use ::editeur::GraphicError;

pub type Result<T> = ::std::result::Result<T, EditeurError>;

/// The enum `EditeurError` defines the possible errors from constructor Editeur.
#[derive(Debug)]
pub enum EditeurError {
    /// Can't write on the output.
    Write(io::Error),
    /// Can't enter in Raw Mode.
    Raw(io::Error),
    /// The Graphic Graphic has meet an error.
    Graphic(GraphicError),
}


impl fmt::Display for EditeurError {
    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for EditeurError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            EditeurError::Write(_) => "Can't write on the output.",
            EditeurError::Raw(_) => "Can't enter in Raw Mode.",
            EditeurError::Graphic(_) => "The Graphic Graphic has meet an error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            EditeurError::Write(ref err) |
            EditeurError::Raw(ref err) => Some(err),
            EditeurError::Graphic(ref err) => Some(err),
        }
    }
}
