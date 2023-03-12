// #[macro_use]
// extern crate serde_derive;

// extern crate serde;
// extern crate serde_json;
// extern crate url;
// extern crate reqwest;

use std::error::Error;

use chesscom_openapi::{
    apis::default_api::{get_chess_games_for_month, get_player_profile, get_player_stats},
    models::{InlineResponse2004, PlayerStats},
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerStatsContainer {
    stats: PlayerStats,
}

type Games = InlineResponse2004;

async fn get_games(username: &str) -> Result<Games, Box<dyn Error>> {
    let conf = chesscom_openapi::apis::configuration::Configuration::default();
    let games = get_chess_games_for_month(&conf, &username, "2023", "02").await?;

    Ok(games)
}

async fn get_profile(username: &str) -> Result<PlayerStats, Box<dyn Error>> {
    let conf = chesscom_openapi::apis::configuration::Configuration::default();
    let profile = get_player_profile(&conf, &username).await?;
    println!("Username: {}", profile.username);
    println!("Status: {}", profile.status);
    println!("Name: {}", profile.name.as_deref().unwrap_or("<Unknown>"));
    println!("Joined: {}", profile.joined.format("%Y-%m-%d"));

    let time_since_online = Utc::now() - profile.last_online;

    if time_since_online < Duration::hours(1) {
        println!("Last Online: {} mins ago", time_since_online.num_minutes());
    } else if time_since_online < Duration::days(1) {
        println!("Last Online: {} hours ago", time_since_online.num_hours());
    } else {
        println!("Last Online: {} days ago", time_since_online.num_days());
    }

    let stats = get_player_stats(&conf, &username).await?;
    Ok(stats)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let p = get_profile("richardfisk").await?;
    let g = get_games("richardfisk").await?;
    println!("{:#?}", g);
    Ok(())
}
