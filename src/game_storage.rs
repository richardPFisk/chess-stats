use std::fs::{File};
use std::io::prelude::*;
extern crate glob;
use self::glob::glob;

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

pub fn read_games() -> Result<Vec<CompletedGame>, Box<dyn std::error::Error>> {
  let mut games_acc = vec![];

  let game_files_wildcard = "./completed_game_*.json";
  let games_glob = glob(game_files_wildcard)?;

  let games_paths: Result<Vec<_>,_> = games_glob.collect();
  for path in games_paths? {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let games: Vec<CompletedGame> = serde_json::from_slice(&buffer)?;
    games_acc.push(games);
  }
  
  Ok(games_acc.into_iter().flatten().collect())
}
