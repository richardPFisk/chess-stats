use dioxus::html::MouseEvent;
use dioxus::prelude::*;

use crate::{components::PieceComponent, eval::board_eval, game::GameState};

#[component]
pub fn ChessBoard(mut game_state: Signal<GameState>) -> Element {
    let mut mounted_text_div: Signal<Option<MountedEvent>> = use_signal(|| None);
    let mut rendered_size: Signal<f64> = use_signal(|| 0_f64);

    let position = game_state.read().current_position().cloned();
    let board_value = board_eval(position).map(|v| v as f64 / 100.0_f64);

    let board = game_state.read().current_board().cloned();
    let board = board.map(|mut b| {
        if !game_state.read().is_white_perspective {
            b.flip_vertical();
            b.flip_horizontal();
        }
        b
    });

    let mut dragged_piece: Signal<Option<(usize, usize)>> = use_signal(|| None);
    let mut dragged_over_piece: Signal<Option<(usize, usize)>> = use_signal(|| None);
    let mut drag_position: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let mut drag_offset: Signal<Option<(f64, f64)>> = use_signal(|| None);

    let piece_size = *rendered_size.read();

    let mut on_mouse_down = move |event: MouseEvent, rank: usize, file: usize| {
        dragged_piece.set(Some((rank, file)));
        let client_point = event.client_coordinates();
        let element_point = event.element_coordinates();

        let offset = (
            client_point.x - (file as f64 * piece_size) - element_point.x,
            client_point.y - ((7_f64 - rank as f64) * piece_size) - element_point.y,
        );
        drag_offset.set(Some(offset));
        drag_position.set(Some((client_point.x, client_point.y)));
    };

    let on_mouse_move = move |event: MouseEvent| {
        if dragged_piece.read().is_some() {
            let client_point = event.client_coordinates();
            drag_position.set(Some((client_point.x, client_point.y)));
        }
    };
    let onmouseup_handler = move |_| {
      let to = dragged_over_piece.read().unwrap();
      let from = dragged_piece.read().unwrap();

      let _ = game_state.write().move_piece(from, to);
    };
    let reset_dragged = move |_| {
        dragged_piece.set(None);
        drag_position.set(None);
        drag_offset.set(None);
    };

    use_effect(move || {
        if let Some(div) = mounted_text_div() {
            spawn(async move {
                let bounding_box = div.get_client_rect().await;
                rendered_size.set(bounding_box.ok().map(|bb| bb.size.width).unwrap_or(0_f64));
            });
        }
    });

    rsx! {
        style { {include_str!("./assets/drag-drop.css")} }
        style { {include_str!("./assets/board.css")} }
        div {
            class: "chess-board",
            onmousemove: on_mouse_move,
            onmouseup: reset_dragged,
            div {
              "{board_value:#?}"
            }
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
                                onmounted: move |element| {
                                  mounted_text_div.set(Some(element.clone()));
                                },
                                onmouseup: onmouseup_handler,
                                onmousedown: move |e| on_mouse_down(e, rank, file),
                                onmouseover: move |_| {
                                  dragged_over_piece.set(Some((rank, file)));
                                },
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
