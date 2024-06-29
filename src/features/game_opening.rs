use fp_core::chain::Chain;
use fp_core::pure::Pure;
use lazy_static::lazy_static;
use regex::Regex;

use crate::models::CompletedGame;
use super::username_colour::username_colour;

pub fn opening(game: &CompletedGame) -> Option<String> {
    _opening(&game.pgn, false)
}

pub fn get_all_openings(username: &str, games: &[CompletedGame]) -> Vec<String> {
    games.iter()
        .filter_map(|game| {
            let my_colour = username_colour(username, game);
            opening(game).map(|original_opening| {
                format!("{} ({})", original_opening, my_colour.as_str())
            })
        })
        .collect()
}

lazy_static! {
    static ref SHORT_OPENING_RE: Regex =
        Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"0-9]*)"#).unwrap();
    static ref LONG_OPENING_RE: Regex =
        Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"]*)"#).unwrap();
}

fn _opening(input: &str, use_short_opening: bool) -> Option<String> {
    let re: &Regex = if use_short_opening { &SHORT_OPENING_RE } else { &LONG_OPENING_RE };
    
    re.captures(input)
        .chain(|captures| 
            captures.name("opening")
                .map(|m| m.as_str().to_owned())
        )
}