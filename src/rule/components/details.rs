use dioxus::prelude::*;
use crate::rule::element::RuleElement;

#[component]
pub fn RuleDescription( rule: RuleElement ) -> Element {
  match ( rule.text, rule.roll, rule.outcomes ) {
    ( Some( text ), _, _ ) => {
      rsx!( span { "{text}" } )
    },
    ( _, Some( roll ), _ ) => {
      rsx!( span { "{roll}" } )
    },
    ( _, _, Some( outcomes ) ) => {
      rsx!(
        div {
          class: "indent grid dim-keyword-table",
          for outcome in outcomes {
            div {
              class: "uv-title highlight",
              "{outcome.result}"
            }
            div {
              class: "uv-details",
              for rule in outcome.rules {
                RuleDescription { rule }
              }
            }
          }
        }
      )
    },
    _ => rsx!()
  }
}
