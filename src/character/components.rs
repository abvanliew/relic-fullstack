use dioxus::prelude::*;

#[component]
pub fn AttributeRow(
  name: String, children: Element, #[props(default)] name_class: Option<String>,
) -> Element {
  let name_class = name_class.unwrap_or("highlight".into());
  rsx!(
    div {
      class: "row full",
      div {
        class: name_class,
        "{name}"
      }
      div {
        class: "align-right",
        { children }
      }
    }
  )
}
#[component]
pub fn ConstitutionRow(constitution: i32) -> Element {
  rsx!(
    div { "Constituion {constitution}" }
    BoxRow { count: constitution }
  )
}

#[component]
pub fn BoxRow(count: i32) -> Element {
  rsx!(
    div {
      class: "box-row",
      if count > 0 {
        for _ in 0..count { div { class: "box" } }
      }
    }
  )
}
