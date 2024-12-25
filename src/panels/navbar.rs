use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    div {
      id: "navbar",
      Link { to: Route::Home {}, "Home" }
      Link { to: Route::Skills {}, "Skills" }
    }
    Outlet::<Route> {}
  }
}