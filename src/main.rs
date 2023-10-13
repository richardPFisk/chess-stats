use game_storage::read_games;

use crate::{
    apis::get_games,
    csv_models::to_csv,
    features::{
        count_by_grouped_openings, count_openings, count_results, get_all_openings,
        group_by_opening,
    },
    game_storage::write_games,
    string_util::get_parent_child_strings,
    tree::get_linfa_tree,
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
    // let g = read_games()?;
    let wrapped_games = get_games(&"Richardfisk").await?;

    let g: Vec<models::CompletedGame> = wrapped_games
        .iter()
        .flat_map(|g| g.games.clone())
        .collect::<Vec<_>>();
    print!("num games {}", &g.len());
    write_games(&g, "./recent-games")?;
    println!("finished reading games");
    // to_csv(&g)?;
    let openings = get_all_openings(g.clone());
    println!("{:#?}", openings);
    let parent_child_openings = get_parent_child_strings(openings.clone());
    println!("{:#?}", parent_child_openings);
    let mut openings_count = count_openings(openings);
    openings_count.sort_by(|(_, a), (_, b)| b.cmp(a));
    // println!("{:#?}", openings_count);
    let results = count_results(g.clone());
    println!("{:#?}", results);

    let games_by_opening = group_by_opening(g.clone());
    // println!("{:#?}", games_by_opening);
    let c = count_by_grouped_openings(games_by_opening.clone());
    println!("{:#?}", c);
    let results_by_opening = games_by_opening
        .iter()
        .map(|(opening, games)| (opening, count_results(games.clone())))
        .collect::<Vec<_>>();
    println!("finished");
    println!("{:#?}", results_by_opening);

    // results_by_opening.sort_by(|(_, a), (_, b)| {
    //     b.iter().map.cmp(a)
    // });

    // let p = get_profile("richardfisk").await?;
    // let gs = get_games("richardfisk").await?;
    // let flattened_games = gs.iter().map(|g| g.games.clone()).collect::<Vec<_>>();

    // for g in flattened_games {
    //     write_games(g)?;
    // }
    get_linfa_tree(&"richardfisk", &g)?;
    Ok(())
}
