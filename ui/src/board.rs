use dioxus::html::DragEvent;
use dioxus::prelude::*;
use dioxus_elements::geometry::Coordinates;
use shakmaty::Board;

use crate::components::PieceComponent;

#[derive(PartialEq, Clone, Props)]
pub struct ChessBoardComponentProps {
    #[props(!optional)]
    pub board: Option<Board>,
    pub is_white_perspective: bool,
}

#[component]
pub fn ChessBoard(props: ChessBoardComponentProps) -> Element {
    let is_white_perspective = props.is_white_perspective;
    let board = props.board;
    let board = board.map(|mut b| {
        if !is_white_perspective {
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
                    ondrop: move |_| {
                      if let (Some(from), Some(to)) = (dragged_piece.read().clone(), dropped_piece.read().clone()) {
                        tracing::info!("####### Drop ####### ({from:?},{to:?})");
                        // make_move(from, to);
                      }
                    },
                    ondragover: move |_| {
                      dropped_piece.set(Some((rank, file)));
                      // tracing::info!("Drag ondragover for  ({rank},{file}) {event:#?} ", );
                      // tracing::info!("ondragover");
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
