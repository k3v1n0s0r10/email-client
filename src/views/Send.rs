use dioxus::prelude::*;

pub fn Send(cx: Scope) -> Element {
    cx.render(rsx! {
      section{
        h1{ "Send" }
      }
    })
}
