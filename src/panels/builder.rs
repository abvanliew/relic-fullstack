use dioxus::prelude::*;

use crate::progression::component::*;
use crate::server::prelude::*;

#[component]
pub fn CharacterBuilder() -> Element {
  if let Some( elements ) = status_element_paths_skills_keywords() {
    return elements
  }
  return rsx! { CharacterProgression {} }
}
