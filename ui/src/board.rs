
use dioxus::html::DragEvent;
use dioxus::prelude::*;
use dioxus_elements::geometry::Coordinates;
use shakmaty::{Board, Move};

use crate::components::PieceComponent;

#[derive(PartialEq, Clone, Props)]
pub struct ChessBoardComponentProps {
    #[props(!optional)]
    last_move: Option<Move>,
    #[props(!optional)]
    board: Option<Board>,
}

#[component]
pub fn ChessBoard(chess: ChessBoardComponentProps) -> Element {
    let board = chess.board;

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

          for rank in 0usize..8 {
            for file in 0usize..8 {
                div { class: if (rank + file) % 2 == 0 { "piece-square light-square" } else { "piece-square dark-square" },
                span {
                  class: "chess-piece",
                  key: "{file},{rank}",
                  prevent_default: "ondragover ondrop",
                  draggable: true,
                    ondrop: move |event| {
                      tracing::info!("####### Drop #######");
                      event.stop_propagation();
                    },
                    ondragover: move |event| {
                      dropped_piece.set(Some((rank, file)));
                      // tracing::info!("Drag ondragover for  ({rank},{file})", );
                      event.stop_propagation();
                      // tracing::info!("ondragover");
                    },
                    ondragend: move |event| {
                      tracing::info!("Drag end for  ({rank},{file})", );

                      event.stop_propagation();
                      // tracing::info!("Drop end...");
                    },
                    ondragstart: move |event: DragEvent| {
                      dragged_piece.set(Some((rank, file)));
                      let coords: Coordinates = event.coordinates();
                      // tracing::info!("Drag started at: {:?}", coords);
                      event.stop_propagation();
                    },
                    PieceComponent { board: board.clone(), rank, file }
                }
              }
            }
          }
          }
        }
    }
}
