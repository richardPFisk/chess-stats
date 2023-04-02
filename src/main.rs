// #[macro_use]
// extern crate serde_derive;

use crate::{apis::get_games, game_storage::write_games};

// extern crate serde;
// extern crate serde_json;
// extern crate url;
// extern crate reqwest;
pub mod apis;
pub mod date_iter;
pub mod features;
pub mod game_storage;
pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let p = get_profile("richardfisk").await?;
    let gs = get_games("richardfisk").await?;
    let flattened_games = gs.iter().map(|g| g.games.clone()).collect::<Vec<_>>();

    println!("length{}", gs.len());
    for g in flattened_games {
        write_games(g)?;
    }
    // let game_option = gs.into_iter().next_back();
    // match game_option {
    //     Some(game) => {
    //         println!("{:#?}", game.games.clone());

    //     },
    //     _ => println!("No game. Game over.")
    // }

    Ok(())
}
