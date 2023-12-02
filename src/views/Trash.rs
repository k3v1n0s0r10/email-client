use dioxus::prelude::*;

#[inline_props]
pub fn Trash(cx: Scope) -> Element {
    render! {
      section{
        h1{ "Trash" }
      }
    }
}
