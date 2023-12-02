use dioxus::prelude::*;

#[inline_props]
pub fn Profile(cx: Scope) -> Element {
    render! {
      section{
        h1{ "Profile" }
      }
    }
}
