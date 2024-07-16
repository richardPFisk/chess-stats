#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod route;
mod board;
pub mod components;
mod engine;
mod convert;
mod game;
mod moves;
mod eval;

use route::Route;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
