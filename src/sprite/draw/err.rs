use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, DrawError>;


/// The enum `DrawError` defines the possible errors
/// from constructor Position.
#[derive(Clone, Debug)]
pub enum DrawError {
    OutOfSize(String),
}

impl fmt::Display for DrawError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for DrawError {
    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            DrawError::OutOfSize(ref size) => size,
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
