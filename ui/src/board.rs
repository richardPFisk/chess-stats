use dioxus::html::DragEvent;
use dioxus::prelude::*;
use dioxus_elements::geometry::Coordinates;

#[component]
pub fn King() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 45 45",
            width: "100%",
            height: "100%",
            g {
                fill: "none",
                fill_rule: "evenodd",
                stroke: "#000",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    d: "M22.5 11.63V6M20 8h5",
                    stroke_linejoin: "miter",
                }
                path {
                    d: "M22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5",
                    fill: "#fff",
                    stroke_linecap: "butt",
                    stroke_linejoin: "miter",
                }
                path {
                    d: "M11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4V27v-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10V37z",
                    fill: "#fff",
                }
                path {
                    d: "M11.5 30c5.5-3 15.5-3 21 0m-21 3.5c5.5-3 15.5-3 21 0m-21 3.5c5.5-3 15.5-3 21 0",
                }
            }
        }
    }
}


#[component]
pub fn Queen() -> Element {
  rsx! {
    svg {
        xmlns: "http://www.w3.org/2000/svg",
        view_box: "0 0 45 45",
        width: "100%",
        height: "100%",
        g {
            fill: "#fff",
            stroke: "#000",
            stroke_width: "1.5",
            stroke_linejoin: "round",
            path {
                d: "M8 12a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM24.5 7.5a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM41 12a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM16 8.5a2 2 0 1 1-4 0 2 2 0 1 1 4 0zM33 9a2 2 0 1 1-4 0 2 2 0 1 1 4 0z",
            }
            path {
                d: "M9 26c8.5-1.5 21-1.5 27 0l2-12-7 11V11l-5.5 13.5-3-15-3 15-5.5-14V25L7 14l2 12z",
                stroke_linecap: "butt",
            }
            path {
                d: "M9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 .5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5.5 2.5.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 .5-1.5-1-2.5-.5-2.5-.5-2 .5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z",
                stroke_linecap: "butt",
            }
            path {
                d: "M11.5 30c3.5-1 18.5-1 22 0M12 33.5c6-1 15-1 21 0",
                fill: "none",
            }
        }
      }
  }
}

#[component]
pub fn ChessBoard() -> Element {
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
                    if dropped_piece.read().map(|d| file != d.1 || rank != d.0).and_then(|_|dragged_piece.read().map(|d| Some(file != d.1 || rank != d.0)).unwrap_or(Some(true))).unwrap_or(true) {
                      Queen {}
                    }
                    else {
                      King {}
                    }
                }
              }
            }
          }
          }
        }
    }
}
