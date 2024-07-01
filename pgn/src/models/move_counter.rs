use pgn_reader::{SanPlus, Skip, Visitor};

pub struct MoveCounter {
  moves: usize,
}

impl MoveCounter {
  pub fn new() -> MoveCounter {
      MoveCounter { moves: 0 }
  }
}

impl Visitor for MoveCounter {
  type Result = usize;

  fn begin_game(&mut self) {
      self.moves = 0;
  }

  fn san(&mut self, _san_plus: SanPlus) {
      self.moves += 1;
  }

  fn begin_variation(&mut self) -> Skip {
      Skip(true) // stay in the mainline
  }

  fn end_game(&mut self) -> Self::Result {
      self.moves
  }
}
