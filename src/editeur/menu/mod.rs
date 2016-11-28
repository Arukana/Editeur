use std::fmt;

use ::termion::color;

pub struct Menu {

}

impl fmt::Display for Menu {
    #[cfg(not(feature = "clipboard"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}Quit <q>{}",
               color::Bg(color::Cyan),
               color::Bg(color::Reset))
    }

    #[cfg(feature = "clipboard")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}Quit <q> Copy <c> Past <v>{}",
               color::Bg(color::Cyan),
               color::Bg(color::Reset))
    }
}

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu {{ _: {} }}", 0)
    }
}

impl Default for Menu {
    fn default() -> Menu {
        Menu {}
    }
}
