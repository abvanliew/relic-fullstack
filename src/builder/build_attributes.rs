use std::collections::HashMap;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use super::build_common::SectionBar;
use super::CharacterBuild;

use crate::builder::build_common::{Counter, CounterBadge};
use crate::builder::LevelSelections;
use crate::builder::build_expertise::FlowBlock;
use crate::character::prelude::{
  AbilityClass, CharacterAbility, RankDisplay, StandardExpertise,
};
use crate::common::{NumericInput, NumericRange};
use crate::modifiers::{ModifierClass, ModifierSet};
use crate::skill::prelude::ResourcePool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeClass {
  Ability(CharacterAbility),
  Expertise(StandardExpertise),
  FreeformObject(ObjectId),
  Pool(ResourcePool),
}

impl AttributeClass {
  pub fn display_as(&self) -> RankDisplay {
    match self {
      AttributeClass::Ability(character_ability) => character_ability.display_as(),
      AttributeClass::Expertise(_) | AttributeClass::FreeformObject(_) => RankDisplay::Bonus,
      AttributeClass::Pool(_) => RankDisplay::Flat,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RankClass {
  Base,
  Specialization,
}

impl RankClass {
  pub fn iter() -> impl Iterator<Item = &'static Self> {
    return [RankClass::Base, RankClass::Specialization].iter();
  }
}

#[derive(Debug, Clone, Default)]
pub struct AttributeRanks {
  pub ranks: HashMap<(AttributeClass, RankClass), i32>,
  pub freeform_objects: Vec<(ObjectId, String)>,
}

impl AttributeRanks {
  pub fn extend(&mut self, other: &Self) {
    self.extend_ranks(&other.ranks);
    self.freeform_objects.extend(other.freeform_objects.clone());
  }

  fn extend_ranks(&mut self, other: &HashMap<(AttributeClass, RankClass), i32>) {
    for ((attribute_class, rank_class), value) in other {
      let entry = self
        .ranks
        .entry((*attribute_class, *rank_class))
        .or_default();
      *entry = (*entry).max(*value);
    }
  }

  pub fn set(&mut self, attribute_class: AttributeClass, rank_class: RankClass, value: i32) {
    self.ranks.insert((attribute_class, rank_class), value);
  }

  pub fn unset(&mut self, attribute_class: &AttributeClass, rank_class: &RankClass) {
    self.ranks.remove(&(*attribute_class, *rank_class));
  }

  pub fn get(&self, attribute_class: &AttributeClass, rank_class: &RankClass) -> i32 {
    self
      .ranks
      .get(&(*attribute_class, *rank_class))
      .cloned()
      .unwrap_or(0)
  }

  pub fn get_ability(&self, ability: &CharacterAbility) -> i32 {
    self.get(&AttributeClass::Ability(*ability), &RankClass::Base)
  }

  pub fn get_ability_specialization(&self, ability: &CharacterAbility) -> i32 {
    self.get(
      &AttributeClass::Ability(*ability),
      &RankClass::Specialization,
    )
  }

  pub fn get_effective(&self, attribute_class: &AttributeClass) -> i32 {
    return self.get(attribute_class, &RankClass::Base)
      + self.get(attribute_class, &RankClass::Specialization);
  }

  fn get_attribute_leeways(
    &self, modifiers: &ModifierSet, prev_cap: i32, prev_def: i32,
  ) -> (i32, i32, Vec<Counter>) {
    let cap_ranks = modifiers.get(&ModifierClass::CapabilityRank);
    let def_ranks = modifiers.get(&ModifierClass::DefenseRank);
    let attribute_cap = cap_ranks + def_ranks + modifiers.get(&ModifierClass::AttributeRank);
    let capability_min = cap_ranks.max(prev_cap);
    let defense_min = def_ranks.max(prev_def);
    let (cap_count, def_count) = self.get_ability_counts(RankClass::Base);
    let cap_leeway = attribute_cap - (def_count.max(defense_min) + cap_count);
    let def_leeway = attribute_cap - (cap_count.max(capability_min) + def_count);
    let counters = vec![Counter {
      title: "Abilities".into(),
      current: cap_count + def_count,
      max: attribute_cap,
      ..Default::default()
    }];
    return (cap_leeway, def_leeway, counters);
  }

  fn get_specialization_leeway(&self, modifiers: &ModifierSet) -> (i32, i32, Vec<Counter>) {
    let cap_spec_pool = modifiers.get(&ModifierClass::CapabilitySpecialization);
    let def_spec_pool = modifiers.get(&ModifierClass::DefenseSpecialization);
    let (cap_spec_count, def_spec_count) = self.get_ability_counts(RankClass::Specialization);
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

  pub fn get_ability_counts(&self, rank_class: RankClass) -> (i32, i32) {
    let capabilites = CharacterAbility::capability_iter()
      .map(|ability| self.get(&AttributeClass::Ability(*ability), &rank_class))
      .sum::<i32>();
    let defenses = CharacterAbility::defense_iter()
      .map(|ability| self.get(&AttributeClass::Ability(*ability), &rank_class))
      .sum::<i32>();
    (capabilites, defenses)
  }
}

impl LevelSelections {
  pub fn get_effective_physique_fortitude(&self) -> (i32, i32) {
    let physique = self
      .attributes
      .get_effective(&AttributeClass::Ability(CharacterAbility::Physique));
    let fortitude = self
      .attributes
      .get_effective(&AttributeClass::Ability(CharacterAbility::Fortitude));
    return (physique, fortitude);
  }
}

impl CharacterBuild {
  pub fn set_attribute(
    &mut self, attribute_class: AttributeClass, rank_class: RankClass, ranks: i32,
  ) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set(attribute_class, rank_class, ranks);
  }

  pub fn unset_attribute(&mut self, attribute_class: &AttributeClass, rank_class: &RankClass) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .unset(attribute_class, rank_class);
  }

  pub fn get_attribute_constraints(
    &self,
  ) -> (
    Vec<(AttributeClass, String, NumericRange, NumericRange)>,
    Vec<(AttributeClass, String, NumericRange, NumericRange)>,
    Vec<Counter>,
  ) {
    let modifiers = self.get_current_modifiers();
    let rank_max = modifiers.get(&ModifierClass::RankMax);
    let spec_max = modifiers.get(&ModifierClass::SpecializationMax);
    let (previous_selections, current_selections) = self.get_selection_pair();
    let (prev_cap, prev_def) = previous_selections
      .attributes
      .get_ability_counts(RankClass::Base);
    let (cap_leeway, def_leeway, mut counters) = current_selections
      .attributes
      .get_attribute_leeways(&modifiers, prev_cap, prev_def);
    let (cap_spec_leeway, def_spec_leeway, counters_spec) = current_selections
      .attributes
      .get_specialization_leeway(&modifiers);
    counters.extend(counters_spec);
    let cap_ranges = CharacterAbility::capability_iter()
      .map(|ability| {
        let ranks = current_selections.attributes.get_ability(ability);
        let previous_ranks = previous_selections.attributes.get_ability(ability);
        let spec = current_selections
          .attributes
          .get_ability_specialization(ability);
        let previous_spec = previous_selections
          .attributes
          .get_ability_specialization(ability);
        let (attribute_max, attribute_spec_max) = match ability.class() {
          AbilityClass::Capability => (
            rank_max.min(ranks + cap_leeway),
            spec_max.min(spec + cap_spec_leeway),
          ),
          AbilityClass::Defense => (
            rank_max.min(ranks + def_leeway),
            spec_max.min(spec + def_spec_leeway),
          ),
        };
        (
          AttributeClass::Ability(*ability),
          ability.to_string(),
          NumericRange::new(previous_ranks, ranks, attribute_max),
          NumericRange::new(previous_spec, spec, attribute_spec_max),
        )
      })
      .collect();
    let def_ranges = CharacterAbility::defense_iter()
      .map(|ability| {
        let ranks = current_selections.attributes.get_ability(ability);
        let previous_ranks = previous_selections.attributes.get_ability(ability);
        let spec = current_selections
          .attributes
          .get_ability_specialization(ability);
        let previous_spec = previous_selections
          .attributes
          .get_ability_specialization(ability);
        let (attribute_max, attribute_spec_max) = match ability.class() {
          AbilityClass::Capability => (
            rank_max.min(ranks + cap_leeway),
            spec_max.min(spec + cap_spec_leeway),
          ),
          AbilityClass::Defense => (
            rank_max.min(ranks + def_leeway),
            spec_max.min(spec + def_spec_leeway),
          ),
        };
        (
          AttributeClass::Ability(*ability),
          ability.to_string(),
          NumericRange::new(previous_ranks, ranks, attribute_max),
          NumericRange::new(previous_spec, spec, attribute_spec_max),
        )
      })
      .collect();
    return (cap_ranges, def_ranges, counters);
  }
}

#[component]
pub fn AttributeSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let (
    capabilities_constraints, 
    defense_constraints, 
    attribute_counters
  ) = build_signal().get_attribute_constraints();
  let (
    expertise_standard_constraints, 
    _expertise_freeform_constraints, 
    expertise_counters
  ) = build_signal().get_expertise_constraints();
  let (
    flows, 
    flow_counters
  ) = build_signal().get_flow_resource_selectors();
  return rsx! {
    SectionBar {
      title: "Attributes",
      bar: rsx! {
        for counter in attribute_counters { CounterBadge{ counter } }
        for counter in expertise_counters { CounterBadge{ counter } }
        for counter in flow_counters { CounterBadge{ counter } }
      },
      explainer: rsx! {
        div {
          "You have a pool of ranks to distributed between Capabilities and Defenses. Depending on your level the number of ranks you can distribute to your attributes is capped. Depeding on your Development choices you might gain some specialization ranks. Each attribute can have at most 1 specialization rank per tier of the character. Specialization ranks counts towards attribute requirements as well granting the flat bonuses."
        }
      },
      div {
        class: "row-wrap",
        div {
          class: "grid dim-resource-chart padded-grid",
          div { class: "subtitle left", "Capabilities" }
          div { class: "uv-third sink", "Ranks" }
          div { class: "sink", "Spec" }
          for (class, name, rank_range, spec_range) in capabilities_constraints {
            AttributeRow{build_signal, class, name, rank_range, spec_range}
          }
          div { class: "subtitle left spacer", "Defenses" }
          div { class: "uv-third sink", "Ranks" }
          div { class: "sink", "Spec" }
          for (class, name, rank_range, spec_range) in defense_constraints {
            AttributeRow{build_signal, class, name, rank_range, spec_range}
          }
        }
        div {
          class: "grid dim-resource-chart-interactible padded-grid",
          div { class: "subtitle left", "Expertise" }
          div { class: "uv-third sink", "Ranks" }
          div { class: "sink", "Spec" }
          for (class, name, rank_range, spec_range) in expertise_standard_constraints {
            AttributeRow {build_signal, class, name, rank_range, spec_range}
          }
          for (class, renamable, name, rank_range, spec_range) in _expertise_freeform_constraints {
            AttributeRow {build_signal, class, renamable, name, rank_range, spec_range}
          }
          div {
            onclick: move |event| {
              event.stop_propagation();
              let mut new_build = build_signal().clone();
              new_build.new_freeform_object();
              build_signal.set(new_build);
            },
            class: "uv-first circle no-select", "\u{FF0B}"
          }
        }
      }
      if flows.len() > 0 {
        div { class: "subtitle", "Flows & Resources" }
      }
      for (flow, flow_size, resource_pools) in flows {
        FlowBlock {
          build_signal, flow, flow_size, resource_pools
        }
      }
    }
  };
}

#[component]
pub fn RankSpecializtionRow(
  title: Element, total: Element,
  rank_range: NumericRange, rank_handler: Callback<i32>,
  spec_range: NumericRange, spec_handler: Callback<i32>,
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

pub fn update_attribute(
  value: i32, mut build_signal: Signal<CharacterBuild>, unset_rank: i32,
  attribute_class: AttributeClass, rank_class: RankClass,
) {
  let mut new_build = build_signal().clone();
  if value == unset_rank {
    new_build.unset_attribute(&attribute_class, &rank_class);
  } else {
    new_build.set_attribute(attribute_class, rank_class, value);
  }
  build_signal.set(new_build);
}

#[component]
pub fn AttributeRow(
  mut build_signal: Signal<CharacterBuild>, class: AttributeClass, name: String,
  rank_range: NumericRange, spec_range: NumericRange, #[props(default)] renamable: bool,
) -> Element {
  let value = rank_range.value + spec_range.value;
  let value_element = class.display_as().from_value(value);
  let unset_rank = rank_range.min;
  let unset_spec = spec_range.min;
  let object_id = match (class, renamable) {
    (AttributeClass::FreeformObject(object_id), true) => Some(object_id),
    _ => None,
  };
  return rsx! {
    if let Some( object_id ) = object_id.clone() {
      input {
        class: "uv-first input full left",
        value: "{name}",
        oninput: move |event| {
          event.stop_propagation();
          let mut new_build = build_signal();
          new_build.set_freeform_name(object_id, event.value());
          build_signal.set(new_build);
        }
      }
    } else {
      div { class: "uv-first", "{name}" }
    }
    div { {value_element} }
    NumericInput {
      class: "input big-text bumper",
      range: rank_range,
      value_handler: move |value: i32| {
        update_attribute(value, build_signal, unset_rank, class, RankClass::Base)
      },
    }
    NumericInput {
      class: "input big-text bumper",
      range: spec_range,
      value_handler: move |value: i32| {
        update_attribute(value, build_signal, unset_spec, class, RankClass::Specialization)
      }
    }
    if let Some(object_id) = object_id.clone() {
      div {
        onclick: move |event| {
          event.stop_propagation();
          let mut new_build = build_signal().clone();
          new_build.remove_freeform_object(&object_id);
          build_signal.set(new_build);
        },
        class: "circle no-select", "\u{FF0D}"
      }
    }
  };
}
