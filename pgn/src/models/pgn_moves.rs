use pgn_reader::{San, SanPlus, Skip, Visitor};

pub struct PgnMoves {
  moves: Vec<San>,
}

impl Default for PgnMoves {
    fn default() -> Self {
        Self::new()
    }
}

impl PgnMoves {
  pub fn new() -> PgnMoves {
    PgnMoves { moves: vec![] }
  }
}

impl Visitor for PgnMoves {
  type Result = Vec<San>;

  fn san(&mut self, san_plus: SanPlus) {
      self.moves.push(san_plus.san);
  }

  fn begin_variation(&mut self) -> Skip {
      Skip(true) // stay in the mainline
  }

  fn end_game(&mut self) -> Self::Result {
      self.moves.clone()
  }
}
