use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use crate::builder::character_build::{SelectionStatus, SelectionValidity};

pub enum Interactible {
  Selectable,
  Deselectable,
  LockedOut,
  LockedIn,
}

pub fn interaction(state: &SelectionStatus, validity: &SelectionValidity) -> Interactible {
  return match (state, validity) {
    (SelectionStatus::SelectedCurrently, _) => Interactible::Deselectable,
    (SelectionStatus::SelectedPreviously, _) => Interactible::LockedIn,
    (SelectionStatus::Unselected, SelectionValidity::Available | SelectionValidity::Minimal) => {
      Interactible::Selectable
    },
    (SelectionStatus::Unselected, SelectionValidity::Full | SelectionValidity::Invalid) => {
      Interactible::LockedOut
    },
  };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
  pub title: String,
  pub increment: i32,
  pub current: i32,
  pub max: i32,
}

impl Default for Counter {
  fn default() -> Self {
    Self {
      title: Default::default(),
      current: 0,
      max: 0,
      increment: 1,
    }
  }
}

impl Counter {
  pub fn from_max(title: String, max: i32) -> Self {
    Self {
      title,
      max,
      ..Default::default()
    }
  }

  pub fn effective(&self) -> (i32, i32, i32) {
    if self.increment <= 1 {
      return (self.current, 0, self.max);
    }
    return (
      self.current / self.increment,
      self.current % self.increment,
      self.max / self.increment,
    );
  }

  pub fn valid(&self) -> SelectionValidity {
    if self.current < self.max {
      return SelectionValidity::Available;
    }
    if self.current == self.max {
      return SelectionValidity::Full;
    }
    return SelectionValidity::Invalid;
  }
}

#[component]
pub fn SectionBar(
  title: String, bar: Element, children: Element, #[props(default)] explainer: Option<Element>,
  #[props(default)] display_default: bool,
) -> Element {
  let mut display_section = use_signal(|| display_default);
  let mut display_explainer = use_signal(|| false);
  return rsx! {
    div {
      class: "section secondary-background",
      onclick: move |event| {
        event.stop_propagation();
        display_section.set(!display_section());
      },
      div {
        class: "row fixed-title-width",
        div { class: "heavier no-select", "{title}" }
        if explainer.is_some() {
          div {
            class: "circle no-select",
            onclick: move |event| {
              event.stop_propagation();
              if !display_section() {
                display_explainer.set(true);
                display_section.set(true);
              } else {
                display_explainer.set(!display_explainer());
              }
            },
            "i"
          }
        }
      }
      div { class: "section", {bar} }
    }
    if display_section() {
      if display_explainer() { {explainer.unwrap_or(rsx! {})} }
      {children}
    }
  };
}

#[component]
pub fn CollapsibleSection(class: String, section: Element, children: Element) -> Element {
  let mut display = use_signal(|| true);
  return rsx! {
    div {
      class,
      onclick: move |event| {
        event.stop_propagation();
        display.set(!display());
      },
      {section}
    }
    if display() {
      {children}
    }
  };
}

#[component]
pub fn CounterBadge(counter: Counter) -> Element {
  let title = &counter.title;
  let (value, remainder, max) = counter.effective();
  let term = match (value == 0, remainder == 0) {
    (true, true) => format!("0"),
    (true, false) => format!("\u{00BD}"),
    (false, true) => format!("{value}"),
    (false, false) => format!("{value} \u{00BD}"),
  };
  let extra_class = match counter.valid() {
    SelectionValidity::Available | SelectionValidity::Minimal => "bg-warn",
    SelectionValidity::Full => "bg-info",
    SelectionValidity::Invalid => "bg-error",
  };
  return rsx! {
    div {
      class: "compact-badge column align-center small-text {extra_class}",
      div { "{title}" }
      div { "{term} / {max}" }
    }
  };
}

#[component]
pub fn FilterButton<T: 'static>(
  title: String, value: T, mut filter_signal: Signal<HashSet<T>>,
) -> Element
where
  T: Eq,
  T: Clone + PartialEq + Eq + Hash,
{
  let filter_set = filter_signal();
  let selected = filter_set.contains(&value);
  return rsx! {
    div {
      class: "row",
      div {
        class: if selected { "medium-border selected" } else { "thin-border minimal-background" },
        onclick: move |event| {
          event.stop_propagation();
          let mut new_filter = filter_set.clone();
          match selected {
            true => new_filter.remove(&value),
            false => new_filter.insert(value.clone()),
          };
          filter_signal.set(new_filter);
        },
        "{title}"
      }
    }
  };
}
