use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::NavBar::NavBar;
use crate::routes::Router::Route;

#[inline_props]
pub fn MainLayout(cx: Scope) -> Element {
    render! {
        div {
            class: "app flex flex-col h-screen max-h-screen overflow-hidden lg:flex-row-reverse",
            main {
              class: "flex-grow overflow-y-auto bg-[#242434] p-10",
              Outlet::<Route> {}
            }
            div {
              class: "sticky bottom-0 lg:left-0",
              NavBar{}
            }
        }
    }
}
