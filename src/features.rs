use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};

use crate::{
    models::CompletedGame,
    string_util::{get_child_to_parent_map, get_parent_child_strings},
};

pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn as_str(&self) -> &'static str {
        match self {
            Colour::White => "white",
            Colour::Black => "black"
        }
    }
}

pub fn result(
    username: &str,
    game: CompletedGame,
) -> chesscom_openapi::models::player_result::Result {
    if game.white.username == username {
        return game.white.result;
    }
    game.black.result
}

pub fn username_colour(
    username: &str,
    game: &CompletedGame,
) -> Colour {
    if game.white.username == username {
        return Colour::White;
    }
    Colour::Black
}

pub fn count_results(
    games: Vec<CompletedGame>,
) -> Vec<(chesscom_openapi::models::player_result::Result, usize)> {
    let mut results_count = vec![];
    for game in games {
        let mut found = false;
        for (_i, (result, count)) in results_count.iter_mut().enumerate() {
            if result == &game.white.result {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            results_count.push((game.white.result, 1));
        }
    }
    results_count
}

pub fn opening(game: &CompletedGame) -> Option<String> {
    _opening(&game.pgn, false)
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

pub fn get_all_openings<F>(game: Vec<CompletedGame>, game_to_opening_name: &F) -> Vec<String> 
where
    F: Fn(&CompletedGame) -> Option<String>,    
{
    game.iter()
        .filter_map(|g| game_to_opening_name(&g))
        .collect::<Vec<_>>()
}

pub fn count_openings(openings: Vec<String>) -> Vec<(String, usize)> {
    let mut openings_count = vec![];
    for opening in openings {
        let mut found = false;
        for (_i, (opening_name, count)) in openings_count.iter_mut().enumerate() {
            if opening_name == &opening {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            openings_count.push((opening, 1));
        }
    }
    openings_count
}

pub fn group_by_opening(username: &str, games: Vec<CompletedGame>) -> Vec<(String, Vec<CompletedGame>)> {
    
    let game_to_opening_name: Box<dyn Fn(&CompletedGame) -> Option<String>> = Box::new(|game: &CompletedGame| {
        let my_colour = username_colour(username, &game);
        let original_opening = opening(&game).unwrap_or_else(|| "Unknown".to_string());
        
        Some(format!("{} ({})", original_opening, my_colour.as_str()))
    });

    let mut grouped_games: HashMap<String, Vec<CompletedGame>> = HashMap::new();
    let openings = get_all_openings(games.clone(), &game_to_opening_name);
    let parent_child_openings: BTreeMap<String, Vec<String>> =
        get_parent_child_strings(openings.clone());

    for (opening_parent, _openings) in parent_child_openings {
        grouped_games.insert(opening_parent, vec![]);
    }
    let child_to_parent_map = get_child_to_parent_map(openings);

    for game in games {
        let opening_name = game_to_opening_name(&game).unwrap();
        if let Some(games_in_group) = grouped_games.get_mut(&opening_name.clone()) {
            games_in_group.push(game.clone());
        }
    }

    let mut gg: Vec<(String, Vec<CompletedGame>)> = grouped_games.into_iter().collect();
    gg.sort_by(|g1, g2| g1.0.len().cmp(&g2.0.len()));
    gg
}

pub fn count_by_grouped_openings(
    grouped_games: Vec<(String, Vec<CompletedGame>)>,
) -> Vec<(String, usize)> {
    grouped_games
        .iter()
        .map(|(opening, games)| (opening.clone(), games.len()))
        .collect::<Vec<_>>()
}
