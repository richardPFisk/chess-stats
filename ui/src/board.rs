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
    let mut drag_offset: Signal<Option<(f64, f64)>> = use_signal(|| None);
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
                                  let client_point = event.client_coordinates();
                                  let element_point = event.element_coordinates();
                                  // Calculate the offset as the difference between client and element coordinates
                                  let offset = (
                                      client_point.x as f64 - element_point.x as f64,
                                      client_point.y as f64 - element_point.y as f64
                                  );
                                  drag_offset.set(Some(offset));
                                  drag_position.set(Some(client_point.into()));
                              },
                              ondrag: move |event: DragEvent| {
                                  let client_point = event.client_coordinates();
                                  drag_position.set(Some(client_point.into()));
                              },
                              style: use_memo(move || {
                                let dragged = dragged_piece.read();
                                let position = drag_position.read();
                                let offset = drag_offset.read();
                                if let (Some((r, f)), Some(pos), Some(off)) = (*dragged, *position, *offset) {
                                    if r == rank && f == file {
                                        format!("transform: translate({}px, {}px);", 
                                                pos.0 as f64 - off.0 as f64, 
                                                pos.1 as f64 - off.1 as f64)
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