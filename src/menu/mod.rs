use std::fmt;

use ::termion::color;

pub struct Menu {

}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}Exit <q>{}",
               color::Bg(color::Cyan),
               color::Bg(color::Reset)
        )
    }
}

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu {{ _: {} }}", 0)
    }
}

impl Default for Menu {
    fn default() -> Menu {
        Menu {
        }
    }
}
