use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, PartError>;

/// The enum `PartError` defines the possible errors
/// from constructor Part.
#[derive(Clone, Debug)]
pub enum PartError {
    UnknownPart(String),
}

impl fmt::Display for PartError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for PartError {
    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            PartError::UnknownPart(ref name) => name,
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
