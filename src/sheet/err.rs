use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, SheetError>;


/// The enum `SheetError` defines the possible errors
/// from constructor Sheet.
#[derive(Clone, Debug)]
pub enum SheetError {
    UnknownSheet(String),
}

impl fmt::Display for SheetError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for SheetError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
      match *self {
         SheetError::UnknownSheet(ref name) => name,
      }
  }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
