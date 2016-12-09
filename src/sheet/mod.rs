mod err;

use std::fmt;

pub use self::err::{SheetError, Result};

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Sheet {
    None = b'_',
    Bust = b'b',
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
        write!(f, "{}", match *self {
            Sheet::None => '_',
            Sheet::Bust => 'B',
        })
    }
}

impl Default for Sheet {
    fn default() -> Sheet {
        Sheet::None
    }
}
