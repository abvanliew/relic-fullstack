use std::collections::HashMap;

use dioxus::prelude::*;

use super::build_common::SectionBar;
use super::CharacterBuild;

use crate::builder::build_common::{Counter, CounterBadge};
use crate::character::prelude::CharacterAttribute;
use crate::common::{NumericInput, NumericRange};
use crate::modifiers::{ModifierClass, ModifierSet};
use crate::progression::component::ranks::RankDisplay;
use crate::progression::fixed::BASE_DEFENSE;
use crate::rules::components::Modifier;
use crate::skill::prelude::ResourcePool;
#[derive(Debug, Clone)]
pub struct AttributeRanks {
  pub ranks: HashMap<CharacterAttribute, AttributeRank>,
  pub expertise: Vec<(String, AttributeRank)>,
  pub resources: HashMap<ResourcePool, i32>,
}

#[derive(Debug, Clone, Default)]
pub struct AttributeRank {
  pub ranks: i32,
  pub specialization: i32,
}

impl Default for AttributeRanks {
  fn default() -> Self {
    Self {
      ranks: Default::default(),
      expertise: vec![("".into(), AttributeRank::default())],
      resources: Default::default(),
    }
  }
}

impl AttributeRanks {
  pub fn get_ranks(&self, attribute: &CharacterAttribute) -> i32 {
    return self
      .ranks
      .get(attribute)
      .map_or(0, |attribute| attribute.ranks);
  }

  pub fn get_specialization(&self, attribute: &CharacterAttribute) -> i32 {
    return self
      .ranks
      .get(attribute)
      .map_or(0, |attribute| attribute.specialization);
  }

  pub fn set_rank(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let entry = self.ranks.entry(attribute).or_default();
    entry.ranks = ranks;
  }

  pub fn set_specialization(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let entry = self.ranks.entry(attribute).or_default();
    entry.specialization = ranks;
  }

  pub fn get_attribute_counts(&self) -> (i32, i32) {
    let capabilites = CharacterAttribute::capability_iter()
      .map(|attribute| self.get_ranks(attribute))
      .sum::<i32>();
    let defenses = CharacterAttribute::defense_iter()
      .map(|attribute| self.get_ranks(attribute))
      .sum::<i32>();
    (capabilites, defenses)
  }

  pub fn get_specialization_counts(&self) -> (i32, i32) {
    let capabilites = CharacterAttribute::capability_iter()
      .map(|attribute| self.get_specialization(attribute))
      .sum::<i32>();
    let defenses = CharacterAttribute::defense_iter()
      .map(|attribute| self.get_specialization(attribute))
      .sum::<i32>();
    (capabilites, defenses)
  }

  pub fn get_expertise_counts(&self) -> (i32, i32) {
    let ranks = self
      .expertise
      .iter()
      .map(|expertise| expertise.1.ranks)
      .sum::<i32>();
    let spec = self
      .expertise
      .iter()
      .map(|expertise| expertise.1.specialization)
      .sum::<i32>();
    (ranks, spec)
  }

  pub fn set_expertise_name(&mut self, index: usize, name: String) {
    if index > self.expertise.len() {
      return;
    }
    self.expertise[index].0 = name.clone();
    let trimmed = name.trim();
    match (index + 1 == self.expertise.len(), trimmed.is_empty()) {
      (true, false) => {
        self.expertise.push(("".into(), AttributeRank::default()));
      },
      (false, true) => {
        self.expertise.remove(index);
      },
      _ => (),
    }
  }

  pub fn set_expertise_ranks(&mut self, index: usize, value: i32) {
    if index > self.expertise.len() {
      return;
    }
    self.expertise[index].1.ranks = value;
  }

  pub fn set_expertise_specialization(&mut self, index: usize, value: i32) {
    if index > self.expertise.len() {
      return;
    }
    self.expertise[index].1.specialization = value;
  }
}

impl CharacterBuild {
  pub fn set_attribute(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.set_rank(attribute, ranks);
  }

  pub fn set_specialization(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_specialization(attribute, ranks);
  }

  pub fn set_expertise_name(&mut self, index: usize, name: String) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.set_expertise_name(index, name);
  }

  pub fn set_expertise_ranks(&mut self, index: usize, value: i32) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.set_expertise_ranks(index, value);
  }

  pub fn set_expertise_specialization(&mut self, index: usize, value: i32) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.set_expertise_specialization(index, value);
  }

  fn get_current_attribute_counts(&self) -> (i32, i32) {
    let level_selection = self.current_selection();
    return level_selection.attributes.get_attribute_counts();
  }

  fn get_current_specialization_counts(&self) -> (i32, i32) {
    let level_selection = self.current_selection();
    return level_selection.attributes.get_specialization_counts();
  }

  fn get_current_expertise_counts(&self) -> (i32, i32) {
    let level_selection = self.current_selection();
    return level_selection.attributes.get_expertise_counts();
  }

  pub fn get_attribute_constraints(
    &self,
  ) -> (
    Vec<(CharacterAttribute, NumericRange, NumericRange)>,
    Vec<Counter>,
  ) {
    let modifiers = self.get_current_modifiers();
    let rank_max = modifiers.get(&ModifierClass::RankMax);
    let spec_max = modifiers.get(&ModifierClass::SpecializationMax);
    let (cap_leeway, def_leeway, mut counters) = self.get_attribute_rank_leeway(&modifiers);
    let (cap_spec_leeway, def_spec_leeway, counters_spec) =
      self.get_attribute_specialization_leeway(&modifiers);
    counters.extend(counters_spec);
    let level_selection = self.current_selection();
    let attribute_ranges = CharacterAttribute::iter()
      .map(|attribute| {
        let ranks = level_selection.attributes.get_ranks(attribute);
        let spec = level_selection.attributes.get_specialization(attribute);
        let (attribute_max, attribute_spec_max) = match attribute.display_as() {
          RankDisplay::Bonus => (
            rank_max.min(ranks + cap_leeway),
            spec_max.min(spec + cap_spec_leeway),
          ),
          RankDisplay::Defense => (
            rank_max.min(ranks + def_leeway),
            spec_max.min(spec + def_spec_leeway),
          ),
        };
        (
          attribute.clone(),
          NumericRange::new(0, ranks, attribute_max),
          NumericRange::new(0, spec, attribute_spec_max),
        )
      })
      .collect();
    return (attribute_ranges, counters);
  }

  fn get_attribute_rank_leeway(&self, modifiers: &ModifierSet) -> (i32, i32, Vec<Counter>) {
    let capability_min = modifiers.get(&ModifierClass::CapabilityRank);
    let defense_min = modifiers.get(&ModifierClass::DefenseRank);
    let attribute_cap = capability_min + defense_min + modifiers.get(&ModifierClass::AttributeRank);
    let (cap_count, def_count) = self.get_current_attribute_counts();
    let cap_leeway = attribute_cap - (def_count.max(defense_min) + cap_count);
    let def_leeway = attribute_cap - (cap_count.max(capability_min) + def_count);
    let counters: Vec<Counter> = vec![
      Counter {
        title: "Abilities".into(),
        current: cap_count + def_count,
        max: attribute_cap,
        ..Default::default()
      },
      Counter {
        title: "Capability Min".into(),
        current: cap_count.min(capability_min),
        max: capability_min,
        ..Default::default()
      },
      Counter {
        title: "Defense Min".into(),
        current: def_count.min(defense_min),
        max: defense_min,
        ..Default::default()
      },
    ];
    return (cap_leeway, def_leeway, counters);
  }

  fn get_attribute_specialization_leeway(
    &self, modifiers: &ModifierSet,
  ) -> (i32, i32, Vec<Counter>) {
    let cap_spec_pool = modifiers.get(&ModifierClass::CapabilitySpecialization);
    let def_spec_pool = modifiers.get(&ModifierClass::DefenseSpecialization);
    let (cap_spec_count, def_spec_count) = self.get_current_specialization_counts();
    let cap_spec_leeway = cap_spec_pool - cap_spec_count;
    let def_spec_leeway = def_spec_pool - def_spec_count;
    let mut counters: Vec<Counter> = Vec::new();
    if cap_spec_count > 0 || cap_spec_pool > 0 {
      counters.push(Counter {
        title: "Capability Spec".into(),
        current: cap_spec_count,
        max: cap_spec_pool,
        ..Default::default()
      });
    }
    if def_spec_count > 0 || def_spec_pool > 0 {
      counters.push(Counter {
        title: "Defense Spec".into(),
        current: def_spec_count,
        max: def_spec_pool,
        ..Default::default()
      });
    }
    return (cap_spec_leeway, def_spec_leeway, counters);
  }

  pub fn get_expertise_constraints(
    &self,
  ) -> (
    Vec<(usize, String, NumericRange, NumericRange)>,
    Vec<Counter>,
  ) {
    let modifiers = self.get_current_modifiers();
    let rank_max = modifiers.get(&ModifierClass::RankMax);
    let spec_max = modifiers.get(&ModifierClass::SpecializationMax);
    let (rank_leeway, spec_leeway, counters) = self.get_expertise_leeway(&modifiers);
    let level_selection = self.current_selection();
    let expertise_ranges = level_selection
      .attributes
      .expertise
      .iter()
      .enumerate()
      .map(|(index, (title, expertise))| {
        let ranks = expertise.ranks;
        let spec = expertise.specialization;
        (
          index,
          title.clone(),
          NumericRange::new(0, ranks, rank_max.min(ranks + rank_leeway)),
          NumericRange::new(0, spec, spec_max.min(spec + spec_leeway)),
        )
      })
      .collect();
    return (expertise_ranges, counters);
  }

  fn get_expertise_leeway(&self, modifiers: &ModifierSet) -> (i32, i32, Vec<Counter>) {
    let rank_pool = modifiers.get(&ModifierClass::ExpertiseRank);
    let spec_pool = modifiers.get(&ModifierClass::ExpertiseSpecialization);
    let (rank_count, spec_count) = self.get_current_expertise_counts();
    let rank_leeway = rank_pool - rank_count;
    let spec_leeway = spec_pool - spec_count;
    let mut counters: Vec<Counter> = vec![Counter {
      title: "Expertise".into(),
      current: rank_count,
      max: rank_pool,
      ..Default::default()
    }];
    if spec_count > 0 || spec_pool > 0 {
      counters.push(Counter {
        title: "Expertise Spec".into(),
        current: spec_count,
        max: spec_pool,
        ..Default::default()
      });
    }
    return (rank_leeway, spec_leeway, counters);
  }
}

// ModifierClass::AnointmentPool
// ModifierClass::AnimismPool
// ModifierClass::SanguinePool
// ModifierClass::RagePool
// ModifierClass::InnatePool
// ModifierClass::InnatePoolAll
// ModifierClass::InnateFlow

// ModifierClass::MasteryPool
// ModifierClass::ChannelPool
// ModifierClass::KiPool
// ModifierClass::VirtuosoPool
// ModifierClass::ResonancePool
// ModifierClass::ResonancePoolAll
// ModifierClass::ResonanceFlow

// ModifierClass::ManaPoolMinor
// ModifierClass::ManaPoolModerate
// ModifierClass::ManaPoolMajor
// ModifierClass::MagicFlow

#[component]
pub fn AttributeSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let (attribute_constraints, counters) = build_signal().get_attribute_constraints();
  let (expertise_constraints, expertise_counters) = build_signal().get_expertise_constraints();
  return rsx! {
    SectionBar {
      title: "Attributes",
      bar: rsx! {
        for counter in counters { CounterBadge{ counter } }
        for counter in expertise_counters { CounterBadge{ counter } }
      },
      div {
        class: "grid dim-resource-chart",
        div { class: "uv-second", "Total" }
        div { "Ranks" }
        div { "Spec" }
        for (attribute, rank_range, spec_range) in attribute_constraints {
          AttributeRow{build_signal, attribute, rank_range, spec_range}
        }
        for (index, title, rank_range, spec_range) in expertise_constraints {
          ExpertiseRow {build_signal, index, title, rank_range, spec_range}
        }
      }
    }
  };
}

#[component]
pub fn RankSpecializtionRow(
  title: Element, total: Element, rank_range: NumericRange,
  rank_handler: Callback<i32>, spec_range: NumericRange, spec_handler: Callback<i32>,
) -> Element {
  return rsx! {
    { title }
    { total }
    NumericInput {
      class: "input big-text bumper",
      range: rank_range,
      value_handler: rank_handler,
    }
    NumericInput {
      class: "input big-text bumper",
      range: spec_range,
      value_handler: spec_handler
    }
  };
}

#[component]
pub fn AttributeRow(
  mut build_signal: Signal<CharacterBuild>, attribute: CharacterAttribute,
  rank_range: NumericRange, spec_range: NumericRange,
) -> Element {
  let attribute_spec = attribute.clone();
  let display = attribute.display_as();
  let value = match &display {
    RankDisplay::Bonus => rank_range.value + spec_range.value,
    RankDisplay::Defense => rank_range.value + spec_range.value + BASE_DEFENSE,
  };
  return rsx! {
    RankSpecializtionRow {
      title: rsx! { div { "{attribute}" } },
      total: match &display {
        RankDisplay::Bonus => rsx! { div { Modifier { value } } },
        RankDisplay::Defense => rsx! { div { "{value}" } },
      },
      rank_range, spec_range,
      rank_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_attribute(attribute.clone(), value);
        build_signal.set(new_build);
      },
      spec_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_specialization(attribute_spec.clone(), value);
        build_signal.set(new_build);
      }
    }
  };
}

#[component]
pub fn ExpertiseRow(
  mut build_signal: Signal<CharacterBuild>, index: usize,
  title: String,
  rank_range: NumericRange, spec_range: NumericRange,
) -> Element {
  let value = rank_range.value + spec_range.value;
  return rsx! {
    RankSpecializtionRow {
      title: rsx! {
        input {
          class: "input full",
          value: "{title}",
          oninput: move |event| {
            event.stop_propagation();
            let mut new_build = build_signal();
            new_build.set_expertise_name(index, event.value());
            build_signal.set(new_build);
          }
        }
      },
      total: rsx! { div { Modifier { value } } },
      rank_range, spec_range,
      rank_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_expertise_ranks(index, value);
        build_signal.set(new_build);
      },
      spec_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_expertise_specialization(index, value);
        build_signal.set(new_build);
      }
    }
  };
}
