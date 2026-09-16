use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NumericRange {
  pub min: i32, pub value: i32, pub max: i32,
}

impl NumericRange {
  pub fn new(min: i32, value: i32, max: i32) -> Self {
    Self {min, value, max}
  }

  pub fn decomp(&self) -> (i32, i32, i32) {
    return (self.min, self.value, self.max);
  }
}

#[component]
pub fn NumericInput(
  range: NumericRange,
  value_handler: Callback<i32>,
  #[props(default, into)] class: String,
) -> Element {
  let mut dirty = use_signal(|| false); dirty();
  let (min, value, max) = range.decomp();
  rsx! {
    input {
      type: "number",
      class, value, min, max,
      onchange: move |event| { 
        let new_value = event.value().parse::<i32>().unwrap_or_default();
        let clamped_value = new_value.clamp(min, max);
        if value == clamped_value && new_value != clamped_value {
          dirty.set(!dirty());
        }
        value_handler.call(clamped_value); },
      onclick: move |event| { event.stop_propagation(); }
    }
  }
}

