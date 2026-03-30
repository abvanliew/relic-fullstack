use crate::common::StaggeredCell;
use crate::keyword::prelude::*;
use crate::server::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn KeywordsPage() -> Element {
  let KeywordCache( ref keyword_cache) = use_context();
  let keywords_all = keyword_cache.into_result_vec().unwrap_or_default();
  let keywords = terms_and_conditions(keywords_all);
  return rsx! {
    div {
      class: "staggered-grid",
      for keyword in keywords {
        StaggeredCell {
          KeywordCard { keyword }
        }
      }
    }
  }
}
