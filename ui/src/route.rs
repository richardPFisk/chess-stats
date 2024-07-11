use dioxus::prelude::*;

use crate::game::ChessGameContainer;
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/home")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/")]
    ChessGameContainer,
}


#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        
    }
}


#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::ChessGameContainer {},
            "Go to board"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
