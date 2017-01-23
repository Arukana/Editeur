// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/Arukana/Editor
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # interface

#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]

#![crate_type="bin"]
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

#[cfg(feature = "clipboard")]
extern crate clipboard;
extern crate termion;
extern crate editeur;

mod interface;

use std::io::Write;

fn main() {
    let mut editeur: interface::Editeur = interface::Editeur::new().unwrap();

    loop {
        print!("{}", editeur);
        if editeur.flush().ok().and(editeur.next()).is_none() {
            break ;
        }
    }
}
