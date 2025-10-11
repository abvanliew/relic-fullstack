use dioxus::prelude::*;

use crate::rule::prelude::MainRulesThread;

#[component]
pub fn MainRules() -> Element {
  rsx! {
    MainRulesThread {}
  }
}
