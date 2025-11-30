use dioxus::prelude::*;

use crate::rules::prelude::MainRulesThread;

#[component]
pub fn MainRules() -> Element {
  rsx! {
    MainRulesThread {}
  }
}
