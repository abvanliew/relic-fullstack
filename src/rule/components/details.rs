use dioxus::prelude::*;
use crate::rule::snippet::RuleSnippet;

#[component]
pub fn RuleDescription( rule: RuleSnippet ) -> Element {
  match ( rule.text, rule.roll, rule.outcomes, rule.effect ) {
    ( Some( text ), _, _, _, ) => {
      rsx!( span { "{text}" } )
    },
    ( _, Some( roll ), _, _, ) => {
      rsx!( span { "{roll}" } )
    },
    ( _, _, Some( outcomes ), _, ) => {
      rsx!(
        div {
          class: "indent grid dim-keywords",
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
    ( _, _, _, Some( effect ), ) => 
      rsx!(
        div {
          if let Some( title ) = effect.title {
            div { "{title}" }
          }
          if let Some( rules ) = effect.rules {
            ul {
              for rule in rules {
                li { RuleDescription { rule } }
              }
            }
          }
        }
      ),
    _ => rsx!()
  }
}
