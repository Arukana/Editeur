#[macro_use]
mod macros;
mod menu;
mod err;

use std::fmt::{self, Display};
use std::io;
use std::path::{Path, PathBuf};
use std::ops::{BitAnd, Div, Rem};

#[cfg(feature = "clipboard")]
use clipboard::ClipboardContext;

pub use self::err::{EditeurError, Result};

use self::menu::Menu;

use graphic::Graphic;
use graphic::emotion::Emotion;
use graphic::sprite::Sprite;
use graphic::sprite::draw::{Draw, SPEC_MAX_X};
use graphic::sprite::texel::Texel;
use graphic::sprite::texel::part::Part;

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

    #[cfg(feature = "clipboard")]
    fn kopimism_command_draw(&self, draw: &Draw) -> String {
        draw.into_iter()
            .filter_map(|&(ref emotion, ref texel): &(Emotion, Texel)|
                        texel.is_first()
                        .and_then(|part: &Part|
                                            emotion.not_empty()
                                            .and_then(|ref emotion: &Emotion|
                                                      Some(format!("{}{}",
                                                                   part,
                                                                   emotion)))))
            .collect::<Vec<String>>()
            .concat()
    }

    #[cfg(feature = "clipboard")]
    fn kopimism_command(&mut self) -> Option<()> {
        self.graphic
            .get_current_sprite()
            .and_then(|&(_, ref sprite): &(PathBuf, Sprite)|
                      Some(sprite.into_iter()
                           .map(|draw: &Draw|
                                format!("{}{}",
                                        draw.get_posture(),
                                        self.kopimism_command_draw(draw)))
                           .collect::<Vec<String>>()
                           .join(" ")))
            .and_then(|command: String|
                      self.kopimism
                      .set_contents(command).ok())
    }

    /// The printer method `write_filename` writes the file name on
    /// the current line.
    fn write_filename(&self, f: &mut fmt::Formatter,
                      path: &Path) -> fmt::Result {
        path.file_stem().unwrap_or_default().to_str().unwrap_or_default().fmt(f)
            .and(termion::clear::AfterCursor.fmt(f)
                 .and("\n\r".fmt(f)))
    }

    fn draw_cell(&self, f: &mut fmt::Formatter,
                 part: &Part, current: bool) -> fmt::Result {
        if current {
            termion::style::Bold.fmt(f)
                .and(part.fmt(f)
                     .and(termion::style::Reset.fmt(f)))
        } else {
            part.fmt(f)
        }
    }

    /// The printer method `write_draw_line` writes the line by
    /// glyph, part and emotion.
    fn write_draw_line(&self, f: &mut fmt::Formatter,
                       line: &[(Emotion, Texel)],
                       is_y: bool, current_x: usize) -> fmt::Result {
        line.iter().map(|&(_, texel): &(Emotion, Texel)|
                        texel.get_glyph())
            .collect::<String>().fmt(f).and(" ".fmt(f))
            .and(line.iter().enumerate()
                 .map(|(x, &(_, texel)): (usize, &(Emotion, Texel))|
                      self.draw_cell(f, texel.get_part(), is_y.bitand(&current_x.eq(&x))))
                 .find(|d| d.is_err())
                 .unwrap_or_else(|| " ".fmt(f))
                 .and(line.iter().map(|&(emotion, _): &(Emotion, Texel)|
                                      emotion.fmt(f))
                      .find(|d| d.is_err())
                      .unwrap_or_else(|| "\n\r".fmt(f))))
    }

    fn write_draw_emotion_list(&self, f: &mut fmt::Formatter,
                               draw: &Draw) -> fmt::Result {
        draw.get_current_part()
            .and_then(|part: &Part|
                      self.graphic.get_emotion_list(draw.get_posture(), part)
                      .and_then(|emotions| Some(
                          emotions.iter()
                              .map(|emotion|
                                          emotion.fmt(f))
                              .find(|e| e.is_err())
                      )).unwrap_or_else(|| Some(Ok(())))
            ).unwrap_or_else(|| Ok(()))
    }

    /// The printer method `write_draw` writes the draw
    /// line by line.
    fn write_draw(&self, f: &mut fmt::Formatter,
                  draw: &Draw) -> fmt::Result {
        let current_position: usize = draw.get_position();
        let (current_x, current_y): (usize, usize) = (
            current_position.rem(SPEC_MAX_X),
            current_position.div(SPEC_MAX_X),
        );

        draw.get_posture().fmt(f)
            .and("\n\r".fmt(f))
            .and(draw.into_iter()
                 .as_slice()
                 .chunks(SPEC_MAX_X)
                 .enumerate()
                 .map(|(y, line): (usize, &[(Emotion, Texel)])|
                      self.write_draw_line(f, line, current_y.eq(&y), current_x))
                 .find(|f| f.is_err())
                 .unwrap_or_else(|| Ok(()))
                 .and("\n\r".fmt(f)))
            .and(self.write_draw_emotion_list(f, draw)
                 .and("\n\r".fmt(f)))
    }

    /// The printer method `write_draw_command` writes the all
    /// the non-none (part, emotions) command of this draw.
    fn write_draw_command(&self, f: &mut fmt::Formatter,
                          draw: &Draw) -> fmt::Result {
        draw.into_iter()
            .filter_map(|&(ref emotion, ref texel): &(Emotion, Texel)|
                        texel.is_first()
                        .and_then(|part: &Part|
                                  emotion.not_empty()
                                  .and_then(|emotion: &Emotion| Some(
                                      format!(" {:?}:{:?}",
                                              part,
                                              emotion).fmt(f)))))
            .find(|d| d.is_err())
            .unwrap_or_else(|| Ok(()))
    }

    /// The printer method `write_sprite_command` writes the all
    /// the command's draws.
    fn write_sprite_command(&self, f: &mut fmt::Formatter,
                            sprite: &Sprite) -> fmt::Result {
        sprite.into_iter()
            .map(|draw: &Draw|
                 "--".fmt(f)
                 .and(draw.get_posture().fmt(f))
                 .and(self.write_draw_command(f, draw))
                 .and(" ".fmt(f)))
            .find(|s| s.is_err())
            .unwrap_or_else(|| Ok(()))
            .and(termion::clear::AfterCursor.fmt(f)
                 .and("\n\r".fmt(f)))
    }

    /// The printer method `write_sprite` writes all the draw and the command.
    fn write_sprite(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.graphic
            .get_current_sprite()
            .and_then(|&(ref path, ref sprite): &(PathBuf, Sprite)|
                Some(self.write_filename(f, path.as_path())
                    .and(sprite.into_iter()
                        .enumerate()
                        .map(|(index, draw)|
                             index.fmt(f)
                             .and(" - ".fmt(f))
                             .and(draw.get_duration().fmt(f))
                             .and(": ".fmt(f))
                             .and(self.write_draw(f, &draw))
                        )
                        .find(|f| f.is_err())
                        .unwrap_or_else(||
                                self.write_sprite_command(f, sprite)))))
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
                           self.kopimism_command()
                        },
                        Event::Key(Key::Home) => {
                            Some(self.graphic.start_position(0))
                        },
                        Event::Key(Key::End) => {
                            Some(self.graphic.end_position(0))
                        },
                        Event::Key(Key::Char('H')) |
                        Event::Key(Key::PageUp) => {
                            Some(self.graphic.sub_position(1))
                        },
                        Event::Key(Key::Char('L')) |
                        Event::Key(Key::PageDown) => {
                            Some(self.graphic.add_position(1))
                        },
                        Event::Key(Key::Char('{')) |
                        Event::Key(Key::Char('[')) => {
                            Some(self.graphic.sub_position_sprite(1))
                        },
                        Event::Key(Key::Char('}')) |
                        Event::Key(Key::Char(']')) => {
                            Some(self.graphic.add_position_sprite(1))
                        },
                        Event::Key(Key::Char('h')) |
                        Event::Key(Key::Left) => {
                            Some(self.graphic.sub_position_sprite_draw(1))
                        },
                        Event::Key(Key::Char('k')) |
                        Event::Key(Key::Up) => {
                            Some(self.graphic.sub_position_sprite_draw(SPEC_MAX_X))
                        },
                        Event::Key(Key::Char('j')) |
                        Event::Key(Key::Down) => {
                            Some(self.graphic.add_position_sprite_draw(SPEC_MAX_X))
                        },
                        Event::Key(Key::Char('l')) |
                        Event::Key(Key::Right) => {
                            Some(self.graphic.add_position_sprite_draw(1))
                        },
                        Event::Key(Key::Char(nbr @ '0'...'9')) => {
                            Some(self.graphic
                                .set_current_emotion(nbr as usize - '0' as usize))
                        },
                        _ => Some(()),
                    }
                }
            })
    }
}
