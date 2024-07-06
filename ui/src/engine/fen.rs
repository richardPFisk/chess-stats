use std::error::Error;

use shakmaty::{fen::Fen, CastlingMode, Chess};

pub fn get_board(fen: &str) -> Result<Chess, Box<dyn Error>>  {
  let fen: Fen = fen.parse()?;
  let pos: Chess = fen.into_position(CastlingMode::Standard)?;

  Ok(pos)
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn fen() {
    let fen = "R7/6p1/7p/8/1p3P2/kr4PK/3R4/8 b - -";
      let r = get_board(fen);
      println!("{r:#?}");
  }
}