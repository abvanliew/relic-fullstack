use dioxus::prelude::*;

#[component]
pub fn CollapsibleHeader(
  #[props(default, into)] class: String,
  #[props(default, into)] class_closed: String,
  #[props(default, into)] class_opened: String, 
  header: Element,
  children: Element,
) -> Element {
  let mut display = use_signal(|| true);
  let extra = if display() { class_opened } else { class_closed };
  return rsx! {
    div {
      class: "no-select {class} {extra}",
      onclick: move |event| {
        event.stop_propagation();
        display.set(!display());
      },
      {header}
    }
    if display() {
      {children}
    }
  };
}