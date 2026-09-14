use dioxus::prelude::*;

pub fn to_modifier(value: i32) -> String {
  match value >= 0 {
    true => format!( "+{value}" ),
    false => format!( "{value}" ),
  }
}

#[component]
pub fn Modifier(
  value: i32, #[props(default)] parenthesis: bool,
  #[props(into)]
  #[props(default)]
  class: String,
) -> Element {
  let modifer = to_modifier(value);
  rsx! { span { class, if parenthesis { "({modifer})" } else { "{modifer}" } } }
}
