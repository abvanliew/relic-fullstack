use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    div {
      class: "row underhang no-print",
      Link { to: Route::Home {}, "Home" }
      Link { to: Route::MainRules {}, "Rules" }
      Link { to: Route::CharacterBuilder {}, "Builder" }
      Link { to: Route::PathsPage {}, "Paths" }
      Link { to: Route::SkillsPage {}, "Skills" }
      Link { to: Route::CharacterSheetPage {}, "Sheets" }
    }
    Outlet::<Route> {}
  }
}
