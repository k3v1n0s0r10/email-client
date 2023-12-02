use dioxus::prelude::*;

use crate::components::Mail::Mail;

pub fn Home(cx: Scope) -> Element {
    render! {
      section{
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
        Mail{}
      }
    }
}
