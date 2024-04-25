use game_storage::read_games;

use crate::{
    features::{
        count_by_grouped_openings, count_openings, count_results, get_all_openings,
        group_by_opening, username_colour, opening,
    },
    string_util::get_parent_child_strings,
    tree::get_linfa_tree, models::CompletedGame,
};

pub mod apis;
mod csv_models;
pub mod date_iter;
pub mod features;
pub mod game_storage;
pub mod models;
mod string_util;
mod tree;
mod engine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let games_path = "./recent-games";
    let g = read_games(games_path)?;
    let username = &"Richardfisk";
    // let wrapped_games = get_games(&username).await?;

    // let g: Vec<models::CompletedGame> = wrapped_games
    //     .iter()
    //     .flat_map(|g| g.games.clone())
    //     .collect::<Vec<_>>();
    print!("num games {}", &g.len());
    // write_games(&g, games_path)?;
    println!("finished writing games");
    // to_csv(&g)?;
    let game_to_opening_name: Box<dyn Fn(&CompletedGame) -> Option<String>> = Box::new(|game: &CompletedGame| {
        let my_colour = username_colour(username, game);
        let original_opening = opening(game).unwrap_or_else(|| "Unknown".to_string());
        
        Some(format!("{} ({})", original_opening, my_colour.as_str()))
    });
    let openings = get_all_openings(g.clone(), &game_to_opening_name);
    println!("{:#?}", openings);
    let parent_child_openings = get_parent_child_strings(openings.clone());
    println!("{:#?}", parent_child_openings);
    let mut openings_count = count_openings(openings);
    openings_count.sort_by(|(_, a), (_, b)| b.cmp(a));
    // println!("{:#?}", openings_count);
    let results = count_results(g.clone());
    println!("{:#?}", results);

    let games_by_opening = group_by_opening(username, g.clone());
    println!("games_by_opening");
    println!("{:#?}", games_by_opening);
    let _c = count_by_grouped_openings(games_by_opening.clone());
    // println!("{:#?}", c);
    let _results_by_opening = games_by_opening
        .iter()
        .map(|(opening, games)| (opening, count_results(games.clone())))
        .collect::<Vec<_>>();
    println!("finished");
    // println!("{:#?}", results_by_opening);

    // results_by_opening.sort_by(|(_, a), (_, b)| {
    //     b.iter().map.cmp(a)
    // });

    // let p = get_profile("richardfisk").await?;
    // let gs = get_games("richardfisk").await?;
    // let flattened_games = gs.iter().map(|g| g.games.clone()).collect::<Vec<_>>();

    // for g in flattened_games {
    //     write_games(g)?;
    // }
    get_linfa_tree("richardfisk", &g)?;
    Ok(())
}
