use std::fs::{File, self};
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
  let games_acc = vec![];

  let game_files_wildcard = "./completed_game_*.json";
  // let dir = fs::read_dir(game_files_wildcard)?;
  let games_glob = glob(game_files_wildcard)?;
  let v: Result<Vec<_>,_> = games_glob.collect();
  for path in v? {
    // let path = entry?.path();
    println!("path: {:?}", path);
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    println!("buf {:?}", buffer);
    // games_acc.push(value);
  }
  
  Ok(games_acc)
}
