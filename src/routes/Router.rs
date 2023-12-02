use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::layout::MainLayout::MainLayout;
use crate::views::{
    Home::Home, PageNotFound::PageNotFound, Profile::Profile, Send::Send, Trash::Trash,
};

#[derive(Routable, PartialEq, Debug, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
      #[route("/")]
      Home {},
      #[route("/send")]
      Send {},
      #[route("/profile")]
      Profile {},
      #[route("/trash")]
      Trash {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
