mod err;

use std::fmt;
use std::char;

pub use self::err::{SheetError, Result};

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Sheet {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'b'.
    Bust = 0x62,
}

impl Sheet {
    pub fn new(content: &str) -> Result<Self> {
        match content {
            "bust" => Ok(Sheet::Bust),
            "none" => Ok(Sheet::None),
            name => Err(SheetError::UnknownSheet(name.to_string())),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match *self {
            Sheet::None => "None",
            Sheet::Bust => "Bust",
        }
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe {
            char::from_u32_unchecked(*self as u32)
        })
    }
}

impl Default for Sheet {
    fn default() -> Sheet {
        Sheet::None
    }
}
