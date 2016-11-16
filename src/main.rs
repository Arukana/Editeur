// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/Arukana/Editor
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # editeur

extern crate editeur;

use std::io::Write;

fn main() {
    let mut editeur: editeur::Editeur = editeur::Editeur::new().unwrap();

    print!("{}", editeur);
    while let Some(()) = editeur.flush().ok().and(
        editeur.next()
    ) {
        print!("{}", editeur);
    }
}
