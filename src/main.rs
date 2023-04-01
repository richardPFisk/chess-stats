// #[macro_use]
// extern crate serde_derive;

use crate::apis::get_games;

// extern crate serde;
// extern crate serde_json;
// extern crate url;
// extern crate reqwest;
pub mod apis;
pub mod date_iter;
pub mod features;
pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let p = get_profile("richardfisk").await?;
    let g = get_games("richardfisk").await?;
    println!("{:#?}", g.games.into_iter().next_back());
    Ok(())
}
