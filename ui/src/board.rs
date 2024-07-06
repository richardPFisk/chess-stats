
use dioxus::html::DragEvent;
use dioxus::prelude::*;
use dioxus_elements::geometry::Coordinates;
use shakmaty::{Board, Position};

use crate::components::PieceComponent;
use crate::engine::fen::get_board;

#[component]
pub fn ChessBoard() -> Element {
    let fen = "R7/6p1/7p/8/1p3P2/kr4PK/3R4/8 b - -";
    let board: Option<Board> = get_board(fen).ok().map(|b| b.board().clone());
    
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
                    
                    // if dropped_piece.read().map(|d| file != d.1 || rank != d.0).and_then(|_|dragged_piece.read().map(|d| Some(file != d.1 || rank != d.0)).unwrap_or(Some(true))).unwrap_or(true) {
                    //   WhiteKing {}
                    // }
                    // else {
                    //   BlackBishop {}
                    // }
                }
              }
            }
          }
          }
        }
    }
}
