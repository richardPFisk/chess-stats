use dioxus::prelude::*;
use chesscom::models::CompletedGame;

#[derive(PartialEq, Clone, Props)]
pub struct ChessGameComponentProps {
    #[props(!optional)]
    last_move: Option<CompletedGame>,
}

#[component]
pub fn ChessGame() -> Element {
  rsx! {
  }
}

#[component]
pub fn ChessGameContainer() -> Element {
  rsx! {
  }
}