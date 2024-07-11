use dioxus::html::DragEvent;
use dioxus::prelude::*;
use dioxus_elements::geometry::Coordinates;
use shakmaty::Board;

use crate::components::PieceComponent;

#[derive(PartialEq, Clone, Props)]
pub struct ChessBoardComponentProps {
    #[props(!optional)]
    pub board: Option<Board>,
    pub is_white: bool,
}

#[component]
pub fn ChessBoard(props: ChessBoardComponentProps) -> Element {
    let is_white = props.is_white;
    let board = props.board;
    let board = board.map(|mut b| {
        if !is_white {
            b.flip_vertical();
            b.flip_horizontal();
        }
        b
    });

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
                    PieceComponent { board: board.clone(), rank, file }
                }
              }
            }
          }
          }
        }
    }
}
