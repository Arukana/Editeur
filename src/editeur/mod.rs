#[macro_use]
mod macros;
mod menu;
mod err;


#[cfg(feature = "clipboard")]
use clipboard::ClipboardContext;
use graphic::emotion::Emotion;

use graphic::sprite::Sprite;
use graphic::sprite::draw::Draw;
use graphic::sprite::texel::Texel;

pub use self::err::{EditeurError, Result};
use self::graphic::Graphic;
pub use self::graphic::sprite::draw::SPEC_MAX_X;

use self::menu::Menu;
use std::fmt::{self, Display};
use std::io;
use std::path::{Path, PathBuf};
pub use super::graphic;

use super::termion;

use termion::event::{Event, MouseEvent, Key};
use termion::input::{self, TermRead};
use termion::raw::{self, IntoRawMode};

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
                use std::io::Write;
                let mut output = input::MouseTerminal::from(stdout);
                if let Some(why) = write!(output, "{}",
                    termion::clear::All,
                )
                    .err()
                    .or(output.flush().err()) {
                    Err(EditeurError::Write(why))
                } else {
                    Ok(editeur_new!(graphic, output))
                }
            }
        }
    }

    /// The printer method `write_filename` writes the file name on
    /// the current line.
    fn write_filename(&self, f: &mut fmt::Formatter,
                      path: &Path) -> fmt::Result {
        path.file_stem().unwrap_or_default().to_str().unwrap_or_default().fmt(f)
            .and(termion::clear::AfterCursor.fmt(f)
                 .and("\n\r".fmt(f)))
    }

    /// The printer method `write_draw_line` writes the line by
    /// glyph, part and emotion.
    fn write_draw_line(&self, f: &mut fmt::Formatter,
                       line: &[(Emotion, Texel)]) -> fmt::Result {
        line.iter().map(|&(_, texel): &(Emotion, Texel)|
                        texel.get_glyph())
            .collect::<String>().fmt(f).and(" ".fmt(f))
            .and(line.iter().map(|&(_, texel): &(Emotion, Texel)|
                                 texel.get_part().fmt(f))
                 .find(|d| d.is_err())
                 .unwrap_or_else(|| " ".fmt(f))
                 .and(line.iter().map(|&(emotion, _): &(Emotion, Texel)|
                                      emotion.fmt(f))
                      .find(|d| d.is_err())
                      .unwrap_or_else(|| "\n\r".fmt(f))))
    }

    /// The printer method `write_draw` writes the draw
    /// line by line.
    fn write_current_draw(&self, f: &mut fmt::Formatter,
                          draw: &Draw) -> fmt::Result {
        draw.get_posture().fmt(f)
            .and("\n\r".fmt(f))
            .and(draw.into_iter()
                 .as_slice()
                 .chunks(SPEC_MAX_X)
                 .map(|line: &[(Emotion, Texel)]|
                      self.write_draw_line(f, line))
                 .find(|f| f.is_err())
                 .unwrap_or_else(|| Ok(()))
                 .and("\n\r".fmt(f)))
    }

    /// The printer method `write_sprite` writes all the draw and the command.
    fn write_sprite(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.graphic
            .get_current_sprite()
            .and_then(|&(ref path, ref sprite): &(PathBuf, Sprite)|
                      Some(self.write_filename(f, path.as_path())
                           .and(sprite.into_iter()
                                .map(|draw| self.write_current_draw(f, &draw))
                                .find(|f| f.is_err())
                                .unwrap_or_else(|| "\n\r".fmt(f)))))
            .unwrap_or_else(|| "there is not a current draw".fmt(f))
    }
}

impl io::Write for Editeur {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl Display for Editeur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}\n\r",
               termion::cursor::Goto(1, 1),
               self.menu)
        .and(self.write_sprite(f))
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
        self.input
            .next()
            .and_then(|event| event.ok())
            .and_then(|event| match event {
                Event::Key(Key::Ctrl('q')) |
                Event::Key(Key::Char('q')) |
                Event::Mouse(MouseEvent::Release(0...8, 1)) => None,
                event => {
                    match event {
                        #[cfg(feature = "clipboard")]
                    Event::Key(Key::Ctrl('c')) |
                        Event::Key(Key::Char('c')) |
                        Event::Mouse(MouseEvent::Release(10...18, 1)) => {
                            self.kopimism
                                .set_contents(format!("{:?}", self.graphic))
                                .ok()
                        }
                        Event::Key(Key::Home) => {
                            Some(self.graphic.start_position(0))
                        }
                        Event::Key(Key::End) => {
                            Some(self.graphic.end_position(0))
                        }
                        Event::Key(Key::Char('H')) |
                        Event::Key(Key::PageUp) => {
                            Some(self.graphic.sub_position(1))
                        }
                        Event::Key(Key::Char('L')) |
                        Event::Key(Key::PageDown) => {
                            Some(self.graphic.add_position(1))
                        }
                        Event::Key(Key::Char('h')) |
                        Event::Key(Key::Left) => {
                            Some(self.graphic.sub_position_sprite_draw(1))
                        }
                        Event::Key(Key::Char('k')) |
                        Event::Key(Key::Up) => {
                            Some(self.graphic
                                .sub_position_sprite_draw(SPEC_MAX_X))
                        }
                        Event::Key(Key::Char('j')) |
                        Event::Key(Key::Down) => {
                            Some(self.graphic
                                .add_position_sprite_draw(SPEC_MAX_X))
                        }
                        Event::Key(Key::Char('l')) |
                        Event::Key(Key::Right) => {
                            Some(self.graphic.add_position_sprite_draw(1))
                        }
                        Event::Key(Key::Char(nbr @ '0'...'9')) => {
                            Some(self.graphic
                                .set_cell_draw(nbr as usize - '0' as usize))
                        }
                        _ => Some(()),
                    }
                }
            })
    }
}
