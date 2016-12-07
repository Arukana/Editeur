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
            "Bust" => Ok(Sheet::Bust),
            "None" => Ok(Sheet::None),
            name => Err(SheetError::UnknownSheet(name.to_string())),
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
