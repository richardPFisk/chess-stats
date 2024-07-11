use dioxus::html::DragEvent;
use dioxus::prelude::*;
use dioxus_elements::geometry::Coordinates;
use shakmaty::{Board, CastlingMode, Chess, File, Move, Piece, Rank, Role, Square};
use shakmaty::Position;
use crate::components::PieceComponent;

#[derive(PartialEq, Clone, Props)]
pub struct ChessBoardComponentProps {
    #[props(!optional)]
    pub chess: Option<Chess>,
    pub is_white: bool,
}

#[derive(Clone)]
struct BoardState {
    chess: Chess,
    board: Board,
    is_white_perspective: bool,
    dragged_piece: Option<(usize, usize)>,
    dropped_piece: Option<(usize, usize)>,
}

impl BoardState {
  fn new(chess: Chess, is_white_perspective: bool) -> Self {
      let mut board = chess.board().clone();
      if !is_white_perspective {
          board.flip_vertical();
          board.flip_horizontal();
      }
      Self {
          chess,
          board,
          is_white_perspective,
          dragged_piece: None,
          dropped_piece: None,
      }
  }

  fn set_dragged(&mut self, piece: Option<(usize, usize)>) {
      self.dragged_piece = piece;
  }

  fn set_dropped(&mut self, piece: Option<(usize, usize)>) {
      self.dropped_piece = piece;
  }

  fn get_square(&mut self, rank_int: usize, file_int: usize) -> Square {
    let file = File::new(file_int as u32);
    let rank = Rank::new(rank_int as u32);
    Square::from_coords(file, rank)
  }

  fn make_move(&mut self) -> Option<Move> {
      if let (Some((from_rank, from_file)), Some((to_rank, to_file))) = (self.dragged_piece, self.dropped_piece) {
        
        let from_square = self.get_square(from_rank, from_file);
        let to_square = self.get_square(to_rank, to_file);
        let from_piece = self.board.piece_at(from_square);
        // let to_piece = self.board.piece_at(to_square);
        let moves = self.chess.legal_moves();
        let chess_move = moves.into_iter().find(|m| {
            match m {
                Move::Normal { from, to, promotion, role , capture } => {
                    *from == from_square && *to == to_square && 
                    (promotion.is_some() == (from_piece.unwrap().role == Role::Pawn && (to_rank == 0 || to_rank == 7)))
                }
                Move::EnPassant { from, to } => *from == from_square && *to == to_square,
                Move::Castle { king, rook } => *king == from_square || *rook == from_square,
                Move::Put { role: _role, to: _to } => false,
            }
        });
        return chess_move;
      }
      None
  }
}

#[component]
pub fn ChessBoard(props: ChessBoardComponentProps) -> Element {
    let chess  = props.chess.clone();
    let is_white = props.is_white;
    // let board = chess.map(|c|c.board().clone());
    // let board = board.map(|mut b| {
    //     if !is_white {
    //         b.flip_vertical();
    //         b.flip_horizontal();
    //     }
    //     b
    // });

    let mut dragged_piece: Signal<Option<(usize, usize)>> = use_signal(|| None);
    let mut dropped_piece: Signal<Option<(usize, usize)>> = use_signal(|| None);

    rsx! {
        style { {include_str!("./assets/drag-drop.css")} }
        style { {include_str!("./assets/board.css")} }
        div {
          class: "chess-board",
          "{dragged_piece:?}{dropped_piece:?}"

        div {
          class: "board",

          for rank in (0..8).rev() {
            for file in 0..8 {
                div { class: if (rank + file) % 2 == 1 { "piece-square light-square" } else { "piece-square dark-square" },
                span {
                  class: "chess-piece",
                  key: "{file},{rank}",
                  prevent_default: "ondragover ondrop",
                  draggable: true,
                    ondrop: move |event| {
                      if let (Some(from), Some(to)) = (dragged_piece.read().clone(), dropped_piece.read().clone()) {
                        tracing::info!("####### Drop ####### {event:#?} ({from:?},{to:?})");
                        // make_move(from, to);
                      }
                    },
                    ondragover: move |event| {
                      dropped_piece.set(Some((rank, file)));
                      // tracing::info!("Drag ondragover for  ({rank},{file}) {event:#?} ", );
                      // tracing::info!("ondragover");
                    },
                    ondragend: move |event| {
                      // tracing::info!("Drag end for  ({rank},{file}) {event:#?} ", );
                    },
                    ondragstart: move |event: DragEvent| {
                      dragged_piece.set(Some((rank, file)));
                      let coords: Coordinates = event.coordinates();
                      // tracing::info!("Drag started at: {:?}", coords);

                    },
                    PieceComponent { chess: chess.clone(), rank, file }
                }
              }
            }
          }
          }
        }
    }
}
