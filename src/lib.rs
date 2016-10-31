// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/Arukana/Editor
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # editeur

#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
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

extern crate neko;
extern crate termion;

mod err;

const SPEC_CLEAR: [u8; 4] = [b'\x1B', b'\x5B', b'\x32', b'\x4A'];

pub use err::{EditeurError, Result};

use std::io::{self, Write};
use std::fmt;

use termion::input;
use termion::raw::{self, IntoRawMode};
use neko::prelude::*;

pub struct Editeur {
    stdout: input::MouseTerminal<raw::RawTerminal<io::Stdout>>,
    graphic: Manager,
}

impl Editeur {
    pub fn new() -> Result<Self> {
        match (io::stdout().into_raw_mode(), Manager::new()) {
            (Err(why), _) => Err(EditeurError::Raw(why)),
            (_, Err(why)) => Err(EditeurError::Graphic(why)),
            (Ok(mut stdout), Ok(graphic)) => {
                if let Some(why) = stdout.write(&SPEC_CLEAR).err() {
                    Err(EditeurError::Write(why))
                } else {
                    Ok(Editeur {
                        stdout: input::MouseTerminal::from(stdout),
                        graphic: graphic,
                    })
                }
            },
        }
    }
}

impl Write for Editeur {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl fmt::Debug for Editeur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Editeur ({:?})", self.graphic)
    }
}
