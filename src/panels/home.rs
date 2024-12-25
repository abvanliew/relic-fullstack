use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
  rsx! {
    div {
      "Relic TTRPG"
    }
    Link { to: Route::SingleSkill { id: "66f45f6b7e098103fc4220a1".into() }, "Aid (Skill)" }
  }
}