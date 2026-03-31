use termion::color;

#[derive(Clone, Copy)]
pub struct Colors {
  pub foreground: color::Rgb,
  pub background: color::Rgb,
  pub hint_foreground: color::Rgb,
  pub hint_background: color::Rgb,
  pub select_foreground: color::Rgb,
  pub select_background: color::Rgb,
  pub multi_foreground: color::Rgb,
  pub multi_background: color::Rgb,
}

pub fn parse_color(color_name: &str) -> color::Rgb {
  match color_name {
    "black" | "default" => color::Rgb(0, 0, 0),
    "red" => color::Rgb(205, 0, 0),
    "green" => color::Rgb(0, 205, 0),
    "yellow" => color::Rgb(205, 205, 0),
    "blue" => color::Rgb(0, 0, 238),
    "magenta" => color::Rgb(205, 0, 205),
    "cyan" => color::Rgb(0, 205, 205),
    "white" => color::Rgb(229, 229, 229),
    _ => {
      if let Some(hex) = color_name.strip_prefix('#')
        && hex.len() == 6
        && let (Ok(r), Ok(g), Ok(b)) = (
          u8::from_str_radix(&hex[0..2], 16),
          u8::from_str_radix(&hex[2..4], 16),
          u8::from_str_radix(&hex[4..6], 16),
        )
      {
        return color::Rgb(r, g, b);
      }
      panic!("Unknown color: {color_name}");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn match_color() {
    let c = parse_color("green");
    assert_eq!((c.0, c.1, c.2), (0, 205, 0));
  }

  #[test]
  fn parse_rgb() {
    let c = parse_color("#1b1cbf");
    assert_eq!((c.0, c.1, c.2), (27, 28, 191));
  }

  #[test]
  #[should_panic]
  fn parse_invalid_rgb() {
    parse_color("#1b1cbj");
  }

  #[test]
  #[should_panic]
  fn no_match_color() {
    parse_color("wat");
  }
}
