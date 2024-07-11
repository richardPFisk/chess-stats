use shakmaty::{Chess, File, Move, Position, Rank, Square};

pub fn get_square(is_white_perspective: bool, rank_int: usize, file_int: usize) -> Square {
  let (file, rank) = if is_white_perspective {
      (file_int, 7 - rank_int)
  } else {
      (7 - file_int, rank_int)
  };
  Square::from_coords(File::new(file as u32), Rank::new(rank as u32))
}

pub fn make_move(chess: Chess, to: (usize, usize), from: (usize, usize), is_white_perspective: bool) -> Option<Chess> {
    let from_square = get_square(is_white_perspective, from.0, from.1);
    let to_square = get_square(is_white_perspective, to.0, to.1);
    
    let current_position = chess;
    let moves = current_position.legal_moves();
    
    if let Some(chess_move) = moves.into_iter().find(|m| {
        match m {
            Move::Normal { from, to, .. } => *from == from_square && *to == to_square,
            Move::EnPassant { from, to } => *from == from_square && *to == to_square,
            Move::Castle { king, rook } => *king == from_square || *rook == from_square,
            Move::Put { .. } => false,
        }
    }) {
        let new_position = current_position.clone();
        
        return new_position.play(&chess_move).ok();
    }
    else {
        None
    }
}