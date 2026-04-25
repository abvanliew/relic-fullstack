use dioxus::prelude::*;

use crate::keyword::prelude::*;

#[component]
pub fn KeywordsPage() -> Element {
  return rsx! {
    TermsConditions {}
  }
}
