#[macro_use]
mod macros;
mod menu;
mod err;

use std::io::{self, Write};
use std::fmt;

pub use self::err::{EditeurError, Result};
pub use super::graphic;

use super::termion;

use termion::input::{self, TermRead};
use termion::raw::{self, IntoRawMode};
use termion::event::{Event, MouseEvent, Key};

#[cfg(feature = "clipboard")]
use clipboard::ClipboardContext;

use self::graphic::Graphic;

pub use self::graphic::sprite::draw::SPEC_MAX_X;

use self::menu::Menu;

pub struct Editeur {
    graphic: Graphic,
    input: input::Events<io::Stdin>,
    output: input::MouseTerminal<raw::RawTerminal<io::Stdout>>,
    #[cfg(feature = "clipboard")]
    kopimism: ClipboardContext,
    menu: Menu,
}

impl Editeur {

    /// The constructor method `new` returns a Editeur interface.
    pub fn new() -> Result<Self> {
        match (io::stdout().into_raw_mode(), Graphic::new()) {
            (Err(why), _) => Err(EditeurError::Raw(why)),
            (_, Err(why)) => Err(EditeurError::Graphic(why)),
            (Ok(stdout), Ok(graphic)) => {
                let mut output = input::MouseTerminal::from(stdout);
                if let Some(why) = write!(output, "{}",
                    termion::clear::All,
                ).err().or(
                    output.flush().err()
                ) {
                    Err(EditeurError::Write(why))
                } else {
                    Ok(editeur_new!(graphic, output))
                }
            },
        }
    }
}

impl Write for Editeur {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl fmt::Display for Editeur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}\n\r{}\n\r",
               termion::cursor::Goto(1, 1),
               self.menu,
               self.graphic)
    }
}

impl fmt::Debug for Editeur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.graphic)
    }
}

impl Iterator for Editeur {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        self.input.next()
            .and_then(|event| event.ok())
            .and_then(|event| match event {
                Event::Key(Key::Ctrl('q')) |
                Event::Key(Key::Char('q')) |
                Event::Mouse(MouseEvent::Release(0...8, 1)) => None,
                event => match event {
                    #[cfg(feature = "clipboard")]
                    Event::Key(Key::Ctrl('c')) |
                    Event::Key(Key::Char('c')) |
                    Event::Mouse(MouseEvent::Release(10...18, 1)) => {
                        self.kopimism.set_contents(
                            format!("{:?}", self.graphic)
                        ).ok()
                    },
                    Event::Key(Key::Home) => Some(
                        self.graphic.start_position(0)
                    ),
                    Event::Key(Key::End) => Some(
                        self.graphic.end_position(0)
                    ),
                    Event::Key(Key::Char('H')) |
                    Event::Key(Key::PageUp) => Some(
                        self.graphic.sub_position(1)
                    ),
                    Event::Key(Key::Char('L')) |
                    Event::Key(Key::PageDown) => Some(
                        self.graphic.add_position(1)
                    ),
                    Event::Key(Key::Char('h')) |
                    Event::Key(Key::Left) => Some(
                        self.graphic.sub_position_sprite_draw(1)
                    ),
                    Event::Key(Key::Char('k')) |
                    Event::Key(Key::Up) => Some(
                        self.graphic.sub_position_sprite_draw(SPEC_MAX_X)
                    ),
                    Event::Key(Key::Char('j')) |
                    Event::Key(Key::Down) => Some(
                        self.graphic.add_position_sprite_draw(SPEC_MAX_X)
                    ),
                    Event::Key(Key::Char('l')) |
                    Event::Key(Key::Right) => Some(
                        self.graphic.add_position_sprite_draw(1)
                    ),
                    Event::Key(Key::Char(nbr @ '0' ... '9')) => Some(
                        self.graphic.set_cell_draw(nbr as usize - '0' as usize)
                    ),
                    _ => Some(()),
                },
            })
    }
}

impl Drop for Editeur {
    fn drop(&mut self) {
        assert!(write!(self.output, "{}{}",
                       termion::cursor::Goto(1, 1), termion::clear::All).is_ok());
    }
}
