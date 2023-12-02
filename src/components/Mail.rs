use dioxus::prelude::*;

pub fn Mail(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        class: "grid grid-cols-4 my-4 px-4 py-2 rounded-lg h-12 content-center items-center justify-center text-white border-2 border-[#00000030] shadow-lg",
        img {
          alt:"",
          class:"h-full rounded-full"
        }
        p {
          "Alerta de seguridad"
        }
        p {
          "Algo paso bien creizi omaiga"
        }
        p {
          chrono::Local::now().format("%Y-%m-%d").to_string()
        }
      }
    })
}
