use dioxus::prelude::*;

// use crate::progression::growth::LevelStats;
// use crate::progression::track::TrackContext;
use crate::progression::{
  component::builder::BuildContext,
  fixed::{MAX_LEVEL, MIN_LEVEL},
};

#[derive(Debug, Clone)]
pub struct LevelContext {
  pub level: Signal<usize>,
}

impl LevelContext {
  pub fn use_context_provider() -> Self {
    let level = use_signal(|| MIN_LEVEL);
    use_context_provider(|| Self { level })
  }
}

#[component]
pub fn LevelSelector() -> Element {
  let mut level = use_context::<BuildContext>().level;
  rsx!(
    div {
      class: "grid dim-keywords",
      div { class: "uv-title", "Level" }
      div {
        class: "uv-after-title",
        select {
          onchange: move |event| {
            let mut new_level = event.value().parse().unwrap_or( MIN_LEVEL );
            if new_level > MAX_LEVEL { new_level = MAX_LEVEL; }
            if new_level < MIN_LEVEL { new_level = MIN_LEVEL; }
            level.set( new_level );
          },
          for lvl in MIN_LEVEL..=MAX_LEVEL {
            option { value: lvl, label: lvl, selected: level() == lvl, }
          }
        }
      }
    }
  )
}
