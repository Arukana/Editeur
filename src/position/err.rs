use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, PostureError>;

/// The enum `PostureError` defines the possible errors
/// from constructor Posture.
#[derive(Clone, Debug)]
pub enum PostureError {
    UnknownPosture(String),
}

impl fmt::Display for PostureError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for PostureError {
    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            PostureError::UnknownPosture(ref name) => name,
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
