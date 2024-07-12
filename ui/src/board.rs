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
    let mut drag_position: Signal<Option<(f64, f64)>> = use_signal(|| None);

  //   let style_attr = use_memo(move || {
  //     let dragged = dragged_piece.read();
  //     let position = drag_position.read();
  //     if let (Some((r, f)), Some((x, y))) = (*dragged, *position) {
  //         if r == rank && f == file {
  //             format!("transform: translate({}px, {}px);", x, y)
  //         } else {
  //             String::new()
  //         }
  //     } else {
  //         String::new()
  //     }
  // });

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
                        div {
                            class: if (rank + file) % 2 == 1 { "piece-square light-square" } else { "piece-square dark-square" },
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
                                    drag_position.set(None);
                                },
                                ondragover: move |_| {
                                    dropped_piece.set(Some((rank, file)));
                                },
                                ondragstart: move |event: DragEvent| {
                                    dragged_piece.set(Some((rank, file)));
                                    let coords: Coordinates = event.coordinates();
                                    drag_position.set(Some((coords.client().x as f64, coords.client().y as f64)));
                                },
                                ondrag: move |event: DragEvent| {
                                    let coords: Coordinates = event.coordinates();
                                    drag_position.set(Some((coords.client().x as f64, coords.client().y as f64)));
                                },
                                style: use_memo(move || {
                                  let dragged = dragged_piece.read();
                                  let position = drag_position.read();
                                  if let (Some((r, f)), Some((x, y))) = (*dragged, *position) {
                                      if r == rank && f == file {
                                          format!("transform: translate({}px, {}px);", x, y)
                                      } else {
                                          String::new()
                                      }
                                  } else {
                                      String::new()
                                  }
                              }),
                                PieceComponent { board: board.clone(), rank, file }
                            }
                        }
                    }
                }
            }
        }
    }
}