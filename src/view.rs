use super::state;
use crate::colors::Colors;
use std::io::{Read, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::{color, cursor, get_tty, terminal_size_fd};

use unicode_width::UnicodeWidthStr;

pub struct View<'a> {
  state: &'a mut state::State<'a>,
  skip: usize,
  multi: bool,
  contrast: bool,
  position: &'a str,
  matches: Vec<state::Match<'a>>,
  colors: Colors,
  chosen: Vec<(String, bool)>,
}

enum CaptureEvent {
  Exit,
  Hint,
}

impl<'a> View<'a> {
  pub fn new(
    state: &'a mut state::State<'a>,
    multi: bool,
    reverse: bool,
    unique: bool,
    contrast: bool,
    position: &'a str,
    colors: Colors,
  ) -> View<'a> {
    let matches = state.matches(reverse, unique);
    let skip = if reverse { matches.len() - 1 } else { 0 };

    View {
      state,
      skip,
      multi,
      contrast,
      position,
      matches,
      colors,
      chosen: vec![],
    }
  }

  pub fn prev(&mut self) {
    if self.skip > 0 {
      self.skip -= 1;
    }
  }

  pub fn next(&mut self) {
    if self.skip < self.matches.len() - 1 {
      self.skip += 1;
    }
  }

  fn make_hint_text(&self, hint: &str) -> String {
    if self.contrast {
      format!("[{hint}]")
    } else {
      hint.to_string()
    }
  }

  fn render(&self, stdout: &mut dyn Write, typed_hint: &str) {
    let (columns, rows) = terminal_size_fd(&get_tty().unwrap()).unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();
    let mut line_row: u16 = 0;
    let mut line_rows = Vec::new();

    for line in self.state.lines.iter() {
      let clean = line.trim_end_matches(|c: char| c.is_whitespace());

      line_rows.push(line_row);
      line_row += match clean.width() {
        l if l >= 1 => (l as u16 - 1) / columns + 1,
        _ => 1,
      }
    }

    let mut line_start = 0;
    for (index, line) in self.state.lines.iter().enumerate() {
      if line_row - 1 - line_rows[index] > rows as u16 {
        line_start = line_rows[index + 1];
        continue;
      }
      let clean = line.trim_end_matches(|c: char| c.is_whitespace());

      if !clean.is_empty() {
        write!(stdout, "{goto}{text}", goto = cursor::Goto(1, line_rows[index] - line_start + 1), text = line).unwrap();
      }
    }

    let selected = self.matches.get(self.skip);

    for mat in self.matches.iter() {
      if line_rows[mat.y as usize] < line_start {
        continue;
      };
      let chosen_hint = self.chosen.iter().any(|(hint, _)| hint == mat.text);

      let selected_color = if chosen_hint {
        self.colors.multi_foreground
      } else if selected == Some(mat) {
        self.colors.select_foreground
      } else {
        self.colors.foreground
      };
      let selected_background_color = if chosen_hint {
        self.colors.multi_background
      } else if selected == Some(mat) {
        self.colors.select_background
      } else {
        self.colors.background
      };

      // Find long utf sequences and extract it from mat.x
      let line = &self.state.lines[mat.y as usize];
      let prefix = &line[0..mat.x as usize];
      let extra = prefix.width_cjk() - prefix.chars().count();
      let offset = (mat.x as u16) - (extra as u16);
      let text = self.make_hint_text(mat.text);

      write!(
        stdout,
        "{goto}{background}{foregroud}{text}{resetf}{resetb}",
        goto = cursor::Goto(offset + 1, line_rows[mat.y as usize] - line_start + 1),
        foregroud = color::Fg(selected_color),
        background = color::Bg(selected_background_color),
        resetf = color::Fg(color::Reset),
        resetb = color::Bg(color::Reset),
        text = &text
      ).unwrap();

      if let Some(ref hint) = mat.hint {
        let extra_position = match self.position {
          "right" => text.width_cjk() - hint.len(),
          "off_left" => 0 - hint.len() - if self.contrast { 2 } else { 0 },
          "off_right" => text.width_cjk(),
          _ => 0,
        };

        let text = self.make_hint_text(hint.as_str());
        let final_position = std::cmp::max(offset as i16 + extra_position as i16, 0);

        write!(
          stdout,
          "{goto}{background}{foregroud}{text}{resetf}{resetb}",
          goto = cursor::Goto(final_position as u16 + 1, line_rows[mat.y as usize] - line_start + 1),
          foregroud = color::Fg(self.colors.hint_foreground),
          background = color::Bg(self.colors.hint_background),
          resetf = color::Fg(color::Reset),
          resetb = color::Bg(color::Reset),
          text = &text
        ).unwrap();

        if hint.starts_with(typed_hint) {
          write!(
            stdout,
            "{goto}{background}{foregroud}{text}{resetf}{resetb}",
            goto = cursor::Goto(final_position as u16 + 1, line_rows[mat.y as usize] - line_start + 1),
            foregroud = color::Fg(self.colors.multi_foreground),
            background = color::Bg(self.colors.multi_background),
            resetf = color::Fg(color::Reset),
            resetb = color::Bg(color::Reset),
            text = &typed_hint
          ).unwrap();
        }
      }
    }

    stdout.flush().unwrap();
  }

  fn listen(&mut self, stdin: &mut dyn Read, stdout: &mut dyn Write) -> CaptureEvent {
    if self.matches.is_empty() {
      return CaptureEvent::Exit;
    }

    let mut typed_hint: String = "".to_owned();
    let longest_hint = self
      .matches
      .iter()
      .filter_map(|m| m.hint.clone())
      .max_by(|x, y| x.len().cmp(&y.len()))
      .unwrap()
      .clone();

    self.render(stdout, &typed_hint);

    loop {
      match stdin.keys().next() {
        Some(key) => {
          match key {
            Ok(key) => {
              match key {
                Key::Esc => {
                  if self.multi && !typed_hint.is_empty() {
                    typed_hint.clear();
                  } else {
                    break;
                  }
                }
                Key::Up => {
                  self.prev();
                }
                Key::Down => {
                  self.next();
                }
                Key::Left => {
                  self.prev();
                }
                Key::Right => {
                  self.next();
                }
                Key::Backspace => {
                  typed_hint.pop();
                }
                Key::Char(ch) => {
                  match ch {
                    '\n' => match self.matches.iter().enumerate().find(|&h| h.0 == self.skip) {
                      Some(hm) => {
                        self.chosen.push((hm.1.text.to_string(), false));

                        if !self.multi {
                          return CaptureEvent::Hint;
                        }
                      }
                      _ => panic!("Match not found?"),
                    },
                    ' ' => {
                      if self.multi {
                        // Finalize the multi selection
                        return CaptureEvent::Hint;
                      } else {
                        // Enable the multi selection
                        self.multi = true;
                      }
                    }
                    key => {
                      let key = key.to_string();
                      let lower_key = key.to_lowercase();

                      typed_hint.push_str(lower_key.as_str());

                      let selection = self.matches.iter().find(|mat| mat.hint == Some(typed_hint.clone()));

                      match selection {
                        Some(mat) => {
                          self.chosen.push((mat.text.to_string(), key != lower_key));

                          if self.multi {
                            typed_hint.clear();
                          } else {
                            return CaptureEvent::Hint;
                          }
                        }
                        None => {
                          if !self.multi && typed_hint.len() >= longest_hint.len() {
                            break;
                          }
                        }
                      }
                    }
                  }
                }
                _ => {
                  // Unknown key
                }
              }
            }
            Err(err) => panic!("{}", err),
          }

          stdin.keys().for_each(|_| { /* Skip the rest of stdin buffer */ })
        }
        _ => {
          // Nothing in the buffer. Wait for a bit...
          std::thread::sleep(std::time::Duration::from_millis(50));
          continue; // don't render again if nothing new to show
        }
      }

      self.render(stdout, &typed_hint);
    }

    CaptureEvent::Exit
  }

  pub fn present(&mut self) -> Vec<(String, bool)> {
    let mut stdin = async_stdin();
    let mut stdout = get_tty().unwrap().into_raw_mode().unwrap().into_alternate_screen().unwrap();

    let hints = match self.listen(&mut stdin, &mut stdout) {
      CaptureEvent::Exit => vec![],
      CaptureEvent::Hint => self.chosen.clone(),
    };

    write!(stdout, "{}", cursor::Show).unwrap();

    hints
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::colors;

  fn split(output: &str) -> Vec<&str> {
    output.split("\n").collect::<Vec<&str>>()
  }

  #[test]
  fn hint_text() {
    let lines = split("lorem 127.0.0.1 lorem");
    let custom = [].to_vec();
    let mut state = state::State::new(&lines, "abcd", &custom);
    let default = colors::parse_color("default");
    let mut view = View {
      state: &mut state,
      skip: 0,
      multi: false,
      contrast: false,
      position: "",
      matches: vec![],
      colors: Colors {
        foreground: default,
        background: default,
        hint_foreground: default,
        hint_background: default,
        select_foreground: default,
        select_background: default,
        multi_foreground: default,
        multi_background: default,
      },
      chosen: vec![],
    };

    let result = view.make_hint_text("a");
    assert_eq!(result, "a".to_string());

    view.contrast = true;
    let result = view.make_hint_text("a");
    assert_eq!(result, "[a]".to_string());
  }
}
