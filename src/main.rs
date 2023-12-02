#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod components;
pub mod layout;
pub mod routes;
pub mod views;

use routes::Router::Route;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()),
    )
    // dioxus_web::launch(App)
}

pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
