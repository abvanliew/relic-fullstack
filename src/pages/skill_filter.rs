use dioxus::prelude::*;

use crate::filter::*;

#[component]
pub fn SkillFilterPage() -> Element {
  return rsx! {
    SkillSearch {}
  }
}