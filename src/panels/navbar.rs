use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    div {
      class: "row underhang no-print",
      Link { to: Route::Home {}, "Home" }
      Link { to: Route::SkillList {}, "Skills" }
      Link { to: Route::FullSkillList {}, "Skills Display" }
      Link { to: Route::PathList {}, "Paths" }
      Link { to: Route::CharacterBuilder {}, "Builder" }
      Link { to: Route::CharacterSheetList {}, "Sheets" }
      Link { to: Route::InherentSkills {}, "Inherent" }
    }
    Outlet::<Route> {}
  }
}
