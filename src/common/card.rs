use dioxus::prelude::*;

#[component]
pub fn Card(
  title: Element,
  children: Element,
  #[props(default)] property: Option<Element>,
  #[props(default)] onclick: Option<Callback<MouseEvent>>, 
  #[props(default, into)] class: String, 
  #[props(default, into)] title_class: String, 
  #[props(default, into)] property_class: String, 
) -> Element {
  let title_uv = if property.is_some() { "uv-title-property" } else { "uv-full" };
  rsx!(
    div {
      class: "card-grid {class}",
      onclick: move |event| { if let Some(callback) = onclick { event.stop_propagation(); callback.call(event); } },
      div { class: "{title_uv} {title_class}", {title} }
      if let Some( element ) = property {
        div { class: "uv-property {property_class}", {element} }
      }
      {children}
    }
  )
}
