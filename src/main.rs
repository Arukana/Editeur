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

extern crate editeur;
extern crate termion;

use termion::event::*;
use termion::cursor;
use termion::input::TermRead;
use std::io::{self, Write};

fn main() {
    let mut editeur: editeur::Editeur = editeur::Editeur::new().unwrap();

    let stdin = io::stdin();
    stdin.events().filter_map(|c| c.ok())
                  .all(|evt| {
        match evt {
            Event::Key(Key::Char('q')) => false,
            evt => {
                match evt {
                    Event::Mouse(me) => {
                        match me {
                            MouseEvent::Press(_, a, b) |
                            MouseEvent::Release(a, b) |
                            MouseEvent::Hold(a, b) => {
                                editeur.write(format!("{}ss", cursor::Goto(a, b)).as_bytes()).unwrap();
                            }
                        }
                    }
                    _ => {}
                }
                editeur.flush().unwrap();
                true
            }
        }
    });
}
