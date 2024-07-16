// use pleco::{core::score::Value, BitBoard, Board};
// use pleco_engine::search::eval::Evaluation;

use std::fmt::{self};

use shakmaty::fen::Fen;
use shakmaty::Bitboard;
use shakmaty::Position;
use timecat::Board;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Value(i32);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}", self.0)
    }
}

pub fn board_eval(position: Option<shakmaty::Chess>) -> Option<i32> {
    if let Some(position) = position {
        let fen = Fen::from_position(position.clone(), shakmaty::EnPassantMode::Legal).to_string();
        // let fen = input_board. (Bitboard(0)).to_string();
        // tracing::info!("{fen:#?}");
        let board = Board::from_fen(&fen);
        // tracing::info!("{board:#?}");
        let mut board = board.ok()?;
        let evaluation = board.evaluate();
        // println!("Current Evaluation: {}\n", evaluation);

        return Some(evaluation as i32)
    }

    return None;
}
