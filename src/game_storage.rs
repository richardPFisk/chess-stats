use std::fs::File;
use std::io::prelude::*;

use crate::models::CompletedGame;

pub fn write_game(
    month: u32,
    year: i32,
    game: CompletedGame,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("completed_game_{}{}.json", year, month);
    let mut file = File::create(path)?;

    let game_str = serde_json::to_string(&game)?;
    file.write_all(game_str.as_bytes())?;
    Ok(())
}
