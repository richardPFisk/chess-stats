#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
mod app;
mod route;
mod board;

use route::Route;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    // tracing_wasm::set_as_global_default();
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
