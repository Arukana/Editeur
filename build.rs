#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![allow(dead_code)]

pub mod graphic {
    /// The sub-directory texel.
    pub const SPEC_SUBD_NCT: &'static str = "texels";
    /// The sub-directory sprite.
    pub const SPEC_SUBD_NCS: &'static str = "sprites";
    /// The sub-directory font.
    pub const SPEC_SUBD_NCF: &'static str = "fonts";
    /// The first directory.
    pub const SPEC_ROOT: &'static str = "NEKO_PATH";
}

const SPEC_ROOT: &'static str = "etc";

use std::env;
use std::fs;
use std::ffi::OsStr;
use std::path::PathBuf;

fn copy<S: AsRef<OsStr>>(
    mut source: PathBuf,
    mut destination: PathBuf,
    sub: S
) -> Option<()> {
    source.push(sub.as_ref());
    destination.push(sub.as_ref());
    fs::create_dir_all(&destination).ok()
       .and_then(|()|
                 fs::read_dir(&source).ok()
                 .and_then(|entry|
                           entry.filter_map(|is| is.ok())
                           .filter_map(|source| {
                                    fs::copy(
                                        source.path(),
                                        destination.join(source.file_name())
                                    ).err()
                           }).next().and_then(|_| None)
                                   .unwrap_or(Some(()))
                 )
       )
}

fn main() {
    env::current_dir().ok()
        .and_then(|mut source| 
            env::var_os(graphic::SPEC_ROOT).and_then(|path| {
                  let destination = PathBuf::from(path);
                  source.push(SPEC_ROOT);
                  copy(source.clone(), destination.clone(), graphic::SPEC_SUBD_NCT)
                      .and(copy(source.clone(), destination.clone(), graphic::SPEC_SUBD_NCS))
                      .and(copy(source.clone(), destination.clone(), graphic::SPEC_SUBD_NCF))
                  }));
}
