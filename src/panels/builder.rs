use dioxus::prelude::*;

use crate::path::Path;
use crate::progression::component::*;
use crate::server::prelude::list_path_skills;

#[component]
pub fn CharacterBuilder() -> Element {
  return rsx! { CharacterProgression {} }
}
