
use lazy_static::lazy_static;
use regex::Regex;

use crate::models::CompletedGame;

pub fn opening(game: &CompletedGame) -> Option<String> {
  _opening(&game.pgn, false)
}

pub fn get_all_openings<F>(game: Vec<CompletedGame>, game_to_opening_name: &F) -> Vec<String> 
where
    F: Fn(&CompletedGame) -> Option<String>,    
{
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