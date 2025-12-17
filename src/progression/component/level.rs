use dioxus::prelude::*;

use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};

#[component]
pub fn LevelSelector(level_signal: Signal<u32>) -> Element {
  rsx!(
    div {
      class: "grid dim-keywords",
      div { class: "uv-title", "Level" }
      div {
        class: "uv-after-title",
        select {
          onchange: move |event| {
            let mut new_level = event.value().parse::<u32>().ok().unwrap_or(MIN_LEVEL);
            if new_level > MAX_LEVEL { new_level = MAX_LEVEL; }
            if new_level < MIN_LEVEL { new_level = MIN_LEVEL; }
            level_signal.set( new_level );
          },
          for lvl in MIN_LEVEL..=MAX_LEVEL {
            option { value: lvl, label: lvl, selected: level_signal() == lvl, }
          }
        }
      }
    }
  )
}
