use dioxus::prelude::*;

#[inline_props]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    println!("Route not found: {:?}", route);
    cx.render(rsx! {
      section{
        h1{ "404" }
        p { "Page not found" }
      }
    })
}
