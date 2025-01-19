use dioxus::prelude::*;

pub struct RankSignal {
  pub start: Signal<i32>,
  pub current: Signal<i32>,
}
