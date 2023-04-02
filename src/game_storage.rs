use std::fs::File;
use std::io::prelude::*;

use chrono::Datelike;

use crate::models::CompletedGame;

pub fn write_games(games: Vec<CompletedGame>) -> Result<(), Box<dyn std::error::Error>> {
    let month = games
        .first()
        .map(|g_unwrapped| {
            let dt = g_unwrapped.end_time;
            dt.month()
        })
        .unwrap_or(0);

    let year = games
        .first()
        .map(|g_unwrapped| {
            let dt = g_unwrapped.end_time;
            dt.year()
        })
        .unwrap_or(0);

    let path = format!("completed_game_{}_{}.json", year, month);
    let mut file = File::create(path)?;

    let game_str = serde_json::to_string(&games)?;
    file.write_all(game_str.as_bytes())?;
    Ok(())
}
