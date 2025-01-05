use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    div {
      class: "row underhang",
      Link { to: Route::Home {}, "Home" }
      Link { to: Route::SkillList {}, "Skills" }
      Link { to: Route::PathList {}, "Paths" }
      Link { to: Route::CharacterBuilder {}, "Builder" }
      Link { to: Route::CharacterSheet {}, "Sheet" }
    }
    Outlet::<Route> {}
  }
}
