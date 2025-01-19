use dioxus::prelude::*;

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