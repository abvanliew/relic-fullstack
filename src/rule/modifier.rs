use dioxus::prelude::*;
use serde::{ Deserialize, Serialize };

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Bonus {
  pub base: Option<i32>,
  pub bonus: Option<i32>,
}

#[component]
pub fn Modifier( value: i32 ) -> Element {
  rsx! {
    span {
      match value >= 0 {
        true => format!( "+{value}" ),
        false => format!( "{value}" ),
      }
    }
  }
}