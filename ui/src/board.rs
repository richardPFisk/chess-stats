use dioxus::prelude::*;
use dioxus::html::{MouseEvent};
use dioxus_elements::geometry::{ClientPoint};
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
    let mut drag_position: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let mut drag_offset: Signal<Option<(f64, f64)>> = use_signal(|| None);

    let piece_size = 80.0; // Adjust this value to match your actual piece size

    let mut on_mouse_down = move |event: MouseEvent, rank: usize, file: usize| {
        dragged_piece.set(Some((rank, file)));
        let client_point = event.client_coordinates();
        let offset = (
            client_point.x as f64 - (file as f64 * piece_size),
            client_point.y as f64 - ((7_f64 - rank as f64) * piece_size)
        );
        drag_offset.set(Some(offset));
        drag_position.set(Some((client_point.x as f64, client_point.y as f64)));
    };

    let on_mouse_move = move |event: MouseEvent| {
        if dragged_piece.read().is_some() {
            let client_point = event.client_coordinates();
            drag_position.set(Some((client_point.x as f64, client_point.y as f64)));
        }
    };

    let on_mouse_up = move |_| {
        if let (Some(from), Some(to)) = (dragged_piece.read().clone(), calculate_drop_position()) {
            tracing::info!("####### Drop ####### ({from:?},{to:?})");
            // make_move(from, to);
        }
        dragged_piece.set(None);
        drag_position.set(None);
        drag_offset.set(None);
    };

    rsx! {
        style { {include_str!("./assets/drag-drop.css")} }
        style { {include_str!("./assets/board.css")} }
        div {
            class: "chess-board",
            onmousemove: on_mouse_move,
            onmouseup: on_mouse_up,

            div {
                class: "board",

                for rank in (0..8).rev() {
                    for file in 0..8 {
                        div {
                            class: if (rank + file) % 2 == 1 { "piece-square light-square" } else { "piece-square dark-square" },
                            span {
                                class: "chess-piece",
                                key: "{file},{rank}",
                                onmousedown: move |e| on_mouse_down(e, rank, file),
                                style: use_memo(move || {
                                    let dragged = dragged_piece.read();
                                    if let Some((r, f)) = *dragged {
                                        if r == rank && f == file {
                                            "visibility: hidden;".to_string()
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
            // Ghost piece
            div {
                style: use_memo(move || {
                    let position = drag_position.read();
                    let offset = drag_offset.read();
                    if let (Some(pos), Some(off), Some(_)) = (*position, *offset, *dragged_piece.read()) {
                        format!("position: fixed; left: {}px; top: {}px; width: {}px; height: {}px; pointer-events: none; z-index: 1000;", 
                                pos.0 - off.0, 
                                pos.1 - off.1,
                                piece_size,
                                piece_size)
                    } else {
                        "display: none;".to_string()
                    }
                }),
                PieceComponent { 
                    board: board.clone(),
                    rank: dragged_piece.read().map(|(r, _)| r).unwrap_or(0),
                    file: dragged_piece.read().map(|(_, f)| f).unwrap_or(0)
                }
            }
        }
    }
}

fn calculate_drop_position() -> Option<(usize, usize)> {
    // Implement this function to calculate the square where the piece is dropped
    // based on the current drag_position
    None
}