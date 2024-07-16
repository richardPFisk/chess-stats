pub mod black_knight;
pub mod black_rook;
pub mod black_bishop;
pub mod black_queen;
pub mod black_king;
pub mod black_pawn;
pub mod white_king;
pub mod white_rook;
pub mod white_pawn;
pub mod white_bishop;
pub mod white_queen;
pub mod white_knight;

use dioxus::prelude::*;
use shakmaty::{Board, File, Rank, Square};

use crate::convert::piece_to_component;

#[derive(PartialEq, Clone, Props)]
pub struct PieceComponentProps {
    rank: usize, 
    file: usize,
    #[props(!optional)]
    board: Option<Board>,
}

#[component]
pub fn PieceComponent(props: PieceComponentProps) -> Element {
  let file = File::new(props.file as u32);
  let rank = Rank::new(props.rank as u32);
  let square = Square::from_coords(file, rank);
  
  let piece = props.board.and_then(|b| b.piece_at(square));

  let element: Option<VNode>  = if let Some(p) = piece { piece_to_component(p) } else { None };
  rsx!{
    {element}
  }
}
