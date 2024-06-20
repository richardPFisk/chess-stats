pub mod username_colour;
pub mod result;
pub mod game_opening;
pub mod group_openings;

use crate::models::CompletedGame;

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


pub fn count_results(
    games: &[CompletedGame],
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

pub fn count_by_grouped_openings(
    grouped_games: Vec<(String, Vec<CompletedGame>)>,
) -> Vec<(String, usize)> {
    grouped_games
        .iter()
        .map(|(opening, games)| (opening.clone(), games.len()))
        .collect::<Vec<_>>()
}
