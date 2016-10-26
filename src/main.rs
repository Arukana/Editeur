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

fn main() {
  println!("Nya! Let's start!");
}
