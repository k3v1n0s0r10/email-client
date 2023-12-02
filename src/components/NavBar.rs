use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_regular_icons::FaTrashCan,
        fa_solid_icons::{FaInbox, FaPaperPlane, FaUser},
    },
    Icon,
};
use dioxus_router::prelude::*;

use crate::routes::Router::Route;

#[inline_props]
pub fn NavBar(cx: Scope) -> Element {
    render! {
      nav {
        class: "bg-[#1C1B1B] py-3 h-full lg:p-4",
        ul {
          class: "flex justify-evenly lg:flex-col lg:space-y-5",
            li {
              class: "px-4 py-2 rounded-lg hover:bg-[#242434] cursor-pointer",
              Link {
                to: Route::Home {},
                Icon {
                  fill: "white",
                  icon: FaInbox,
                }
              }
            }
            li {
              class: "px-4 py-2 rounded-lg hover:bg-[#242434] cursor-pointer",
              Link {
                to: Route::Send {},
                Icon {
                  fill: "white",
                  icon: FaPaperPlane,
                }
              }
            }
            li {
              class: "px-4 py-2 rounded-lg hover:bg-[#242434] cursor-pointer",
              Link {
                to: Route::Trash {},
                Icon {
                  fill: "white",
                  icon: FaTrashCan,
                }
              }
            }
            li {
              class: "px-4 py-2 rounded-lg hover:bg-[#242434] cursor-pointer",
              Link {
                to: Route::Profile {},
                Icon {
                  fill: "white",
                  icon: FaUser,
                }
              }
          }
        }
      }
    }
}
