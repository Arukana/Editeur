// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/Arukana/Editor
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # editeur

#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]

#![crate_type="lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]

#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file="clippy.toml")))]

#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]

#![deny(
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications
)]

extern crate termion;

mod graphic;
mod menu;
mod err;

pub use err::{EditeurError, Result};

use std::io::{self, Write};
use std::fmt;

use termion::input::{self, TermRead};
use termion::raw::{self, IntoRawMode};
use termion::event::{Event, Key};

use graphic::Graphic;

use graphic::sprite::draw::SPEC_MAX_X;

use menu::Menu;

/// The first directory.
const SPEC_ROOT: &'static str = ".neko";

pub struct Editeur {
    graphic: Graphic,
    input: input::Events<std::io::Stdin>,
    output: input::MouseTerminal<raw::RawTerminal<io::Stdout>>,
    menu: Menu,
}

impl Editeur {
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
                    Ok(Editeur {
                        graphic: graphic,
                        input: io::stdin().events(),
                        output: output,
                        menu: Menu::default(),
                    })
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
                 self.graphic
        )
    }
}

impl fmt::Debug for Editeur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Editeur ({:?})", self.graphic)
    }
}

impl Iterator for Editeur {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        self.input.next()
            .and_then(|event| event.ok())
            .and_then(|event|
                      match event {
                          Event::Key(Key::Ctrl('q')) |
                          Event::Key(Key::Char('q')) => None,

                          Event::Key(Key::Ctrl('j')) => Some(self.graphic.sub_position(1)),
                          Event::Key(Key::Ctrl('k')) => Some(self.graphic.add_position(1)),
                          Event::Key(Key::Ctrl('h')) => Some(self.graphic.sub_position_sprite(1)),
                          Event::Key(Key::Ctrl('l')) => Some(self.graphic.add_position_sprite(1)),

                          Event::Key(Key::Char('h')) => Some(self.graphic.sub_position_sprite_draw(1)),
                          Event::Key(Key::Char('j')) => Some(self.graphic.sub_position_sprite_draw(SPEC_MAX_X)),
                          Event::Key(Key::Char('k')) => Some(self.graphic.add_position_sprite_draw(SPEC_MAX_X)),
                          Event::Key(Key::Char('l')) => Some(self.graphic.add_position_sprite_draw(1)),
                          e => {
                              println!("{:?}", e);
                              Some(())
                          },
                      }
            )
    }
}
