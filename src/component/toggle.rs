use dioxus::prelude::*;

#[component]
pub fn ToggleDisplay( on: Element, off: Element, element: Element ) -> Element {
  let mut display = use_signal(|| false);
  rsx!(
    div {
      onclick: move |_| { display.with_mut(|x| *x = !*x); },
      if *display.read() {
        {on}
      } else {
        {off}
      }
    }
    div {
      class: if *display.read() { "" } else { "hidden" },
      {element}
    }
  )
}