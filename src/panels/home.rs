use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
    div {
      "Relic TTRPG"
    }
    Link { to: Route::SingleSkillPage { id: "66f45f6b7e098103fc4220a1".into() }, "Aid (Skill)" }
  }
}
