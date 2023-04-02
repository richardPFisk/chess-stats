use game_storage::read_games;

use crate::{apis::get_games, game_storage::write_games};

pub mod date_iter;
pub mod features;
pub mod game_storage;
pub mod models;
pub mod apis;
mod tree;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let g = read_games();
    println!("{:?}", g);
    // let p = get_profile("richardfisk").await?;
    // let gs = get_games("richardfisk").await?;
    // let flattened_games = gs.iter().map(|g| g.games.clone()).collect::<Vec<_>>();

    // for g in flattened_games {
    //     write_games(g)?;
    // }

    Ok(())
}
