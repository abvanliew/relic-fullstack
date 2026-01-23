use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    div {
      class: "row gap-xlarge underhang no-print",
      Link { to: Route::MainRules {}, "Home" }
      Link { to: Route::PathsPage {}, "Paths" }
      Link { to: Route::SkillsPage {}, "Skills" }
      Link { to: Route::CharacterBuilder {}, "Builder" }
      // Link { to: Route::CharacterSheetPage {}, "Sheets" }
    }
    Outlet::<Route> {}
  }
}
