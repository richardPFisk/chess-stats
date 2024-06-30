// use dioxus::prelude::*;
// use pleco::Board;

// // use crate::components::{ChessBoard, EvaluationBar, MoveHistory, NavigationButtons};

// #[derive(Props)]
// pub struct AppProps {}

// pub fn App(cx: Scope<AppProps>) -> Element {
//     let board = use_state(cx, || Board::start_pos());
//     let move_history = use_state(cx, Vec::<String>::new);
//     let evaluation = use_state(cx, || 0.0);

//     rsx! {
//         div { class: "chess-app",
//             ChessBoard { 
//                 board: board.clone(),
//                 on_move: move |from, to| {
//                     // Implement move logic
//                 }
//             }
//             div { class: "sidebar",
//                 EvaluationBar { evaluation: *evaluation.get() }
//                 MoveHistory { moves: move_history.get().clone() }
//                 NavigationButtons {
//                     on_navigate: move |step| {
//                         // Implement navigation logic
//                     }
//                 }
//             }
//         }
//     }
// }