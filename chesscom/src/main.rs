use game_storage::read_games;
use tokio::sync::mpsc::error;

use crate::{
    features::{
        count_by_grouped_openings, count_openings, count_results, game_opening::get_all_openings,
        group_openings::group_by_opening_by_root_opening,
    },
    string_util::get_parent_child_strings,
    ml::tree::get_linfa_tree, models::CompletedGame,
};


pub mod get_profile;
mod csv_models;
pub mod date_iter;
pub mod features;
pub mod game_storage;
pub mod models;
mod string_util;
mod ml;
mod engine;
mod games_source;

use games_source::{chess_com_source::ChessCommGamesSource, GamesSource};


async fn api_games() -> Result<Vec<CompletedGame>, Box<dyn std::error::Error>> {
    let username = "Richardfisk".into();
    let from_month_string = "2024-05-01T23:59:60.234567+05:00".into();

    let gs = ChessCommGamesSource { username, from_month_string };
    let wrapped_games = gs.get_games().await?;
    // println!("{wrapped_games:#?}");
    let games = wrapped_games.into_iter().flat_map(|g|g.games).collect::<Vec<_>>();
    Ok(games)
}

fn results_by_openings<'a>(username: &'a str, games: &'a [CompletedGame]) -> Vec<(String, Vec<(chesscom_openapi::models::player_result::Result, usize)>)> {
    let openings = get_all_openings(username, games);
    println!("{openings:#?}");
    // let parent_child_openings = get_parent_child_strings(openings.clone());
    // println!("{parent_child_openings:#?}");
    // let mut openings_count = count_openings(openings);
    // openings_count.sort_by(|(_, a), (_, b)| b.cmp(a));

    // let results_by_count = count_results(games);

    // let games_by_opening = group_by_opening_by_root_opening(username, games);
    // let _c = count_by_grouped_openings(games_by_opening.clone());
    // let _results_by_opening = games_by_opening
    //     .iter()
    //     .map(|(opening, games)| (opening.to_string(), count_results(games)))
    //     .collect::<Vec<_>>();
    // // println!("{_results_by_opening:#?}");
    // _results_by_opening
    vec![]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let games_path = "./recent-games";
    // let games: Vec<CompletedGame> = read_games(games_path)?;

    let username  = "Richardfisk";

    let games = api_games().await?;
    println!("{games:#?}");
    let games_count = games.len();
    println!("===> games_count: {games_count}");
    let black_games: Vec<CompletedGame> = games.clone().into_iter().filter(|g| g.black.username == username).collect::<Vec<_>>();
    let black_openings = results_by_openings(username, &black_games.as_slice());
    let black_openings_count: usize = black_openings.into_iter().map(|(a, b)| b.len()).into_iter().sum();
    println!("{black_openings_count:#?}");

    let white_games = games.clone().into_iter().filter(|g| g.white.username == username).collect::<Vec<_>>();
    let white_openings = results_by_openings(username, &white_games.as_slice());
    let white_openings_count: usize = white_openings.into_iter().map(|(a, b)| b.len()).into_iter().sum();
    println!("{white_openings_count:#?}");

    // get_linfa_tree("richardfisk", &g)?;
    Ok(())
}
