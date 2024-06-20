
use lazy_static::lazy_static;
use regex::Regex;

use crate::models::CompletedGame;

use super::username_colour::username_colour;

pub fn opening(game: &CompletedGame) -> Option<String> {
  _opening(&game.pgn, false)
}

pub fn get_all_openings(username: &str, game: &[CompletedGame]) -> Vec<String> 
{
    let game_to_opening_name: Box<dyn Fn(&CompletedGame) -> Option<String>> = Box::new(|game: &CompletedGame| {
      let my_colour = username_colour(username, game);
      let original_opening = opening(game).unwrap_or_else(|| "Unknown".to_string());
      
      Some(format!("{} ({})", original_opening, my_colour.as_str()))
    });
    game.iter()
        .filter_map(game_to_opening_name)
        .collect::<Vec<_>>()
}

lazy_static! {
  static ref SHORT_OPENING_RE: Regex =
      Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"0-9]*)"#).unwrap();
}
lazy_static! {
  static ref LONG_OPENING_RE: Regex =
      Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"]*)"#).unwrap();
}

fn _opening(input: &str, use_short_opening: bool) -> Option<String> {
  if use_short_opening {
      let re = &SHORT_OPENING_RE;
      let captures = re.captures_iter(input);
      let first = captures.into_iter().next();

      return first
          .as_ref()
          .and_then(|c| c.name("opening").map(|m| m.as_str().to_owned()));
  } else {
      let captures = LONG_OPENING_RE.captures_iter(input);
      let first = &captures.into_iter().next();

      return first
          .as_ref()
          .and_then(|c| c.name("opening").map(|m| m.as_str().to_owned()));
  }
}