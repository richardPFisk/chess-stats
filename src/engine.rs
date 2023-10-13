use pleco::Board;
// use pleco::board::

pub fn read_fen(fen: &str) -> Board {
  Board::from_fen(fen).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_board() {
      let fen = "k2r3r/8/1Q1p3p/pp1Bp3/4P1n1/P2P1q2/1PPKN3/R7 b - -";
      let r = read_fen(fen);
      assert_eq!(r, Board::start_pos());
    }

    #[test]
    fn test_pgn() {
      let pgn = "k2r3r/8/1Q1p3p/pp1Bp3/4P1n1/P2P1q2/1PPKN3/R7 b - -";
      
      read_fen(pgn);
    }
}