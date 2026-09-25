use std::collections::HashMap;

use dioxus::prelude::*;

use super::common::SectionBar;
use super::CharacterBuild;

use crate::builder::common::{Counter, CounterBadge};
use crate::builder::LevelSelections;
use crate::character::prelude::{CharacterAttribute, Flow, RankDisplay, StandardExpertise};
use crate::common::{NumericInput, NumericRange};
use crate::modifiers::{ModifierClass, ModifierSet};
use crate::progression::fixed::BASE_DEFENSE;
use crate::rules::components::Modifier;
use crate::skill::prelude::ResourcePool;

#[derive(Debug, Clone, Default)]
pub struct FreeformAttribute {
  pub name: String,
  pub attribute: AttributeRank,
}

impl FreeformAttribute {
  pub fn empty() -> Self {
    Self {
      name: "".into(),
      attribute: AttributeRank::default(),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct AttributeRank {
  pub ranks: Option<i32>,
  pub specialization: Option<i32>,
}

impl AttributeRank {
  pub fn extend(&mut self, other: &Self) {
    self.ranks = self.ranks.max(other.ranks);
    self.specialization = self.specialization.max(other.specialization);
  }
}

fn unpack_rank(result: Option<&AttributeRank>) -> i32 {
  result.map_or(0, |attribute| attribute.ranks.unwrap_or(0))
}

fn unpack_spec(result: Option<&AttributeRank>) -> i32 {
  result.map_or(0, |attribute| attribute.specialization.unwrap_or(0))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeClass {
  Ability( CharacterAttribute ),
  Expertise( StandardExpertise ),
  Freeform( usize ),
  Pool( ResourcePool )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankClass {
  Base,
  Specialization,
}

#[derive(Debug, Clone)]
pub struct AttributeRanks {
  pub rank_map: HashMap<(AttributeClass, RankClass), i32>,
  pub freeform: Vec<String>,
  
  pub ranks: HashMap<CharacterAttribute, AttributeRank>,
  pub expertise_standard: HashMap<StandardExpertise, AttributeRank>,
  pub expertise_freeform: Vec<FreeformAttribute>,
  pub resources: HashMap<ResourcePool, i32>,
}

impl Default for AttributeRanks {
  fn default() -> Self {
    Self {
      rank_map: Default::default(),
      freeform: vec!["".into()],

      ranks: Default::default(),
      expertise_standard: Default::default(),
      expertise_freeform: vec![FreeformAttribute::empty()],
      resources: Default::default(),
    }
  }
}

impl AttributeRanks {
  pub fn extend(&mut self, other: &Self) {
    for (key, value) in other.ranks.clone().into_iter() {
      self
        .ranks
        .entry(key)
        .and_modify(|attribute| attribute.extend(&value))
        .or_insert(value);
    }
    for (key, mut value) in other.resources.clone().into_iter() {
      self
        .resources
        .entry(key)
        .and_modify(|pool| *pool = *pool.max(&mut value))
        .or_insert(value);
    }
    self.extend_expertise_standard(&other.expertise_standard);
    self.extend_expertise_freeform(&other.expertise_freeform);
  }

  fn extend_expertise_standard(&mut self, other: &HashMap<StandardExpertise, AttributeRank>) {
    for (key, value) in other.clone().into_iter() {
      self
        .expertise_standard
        .entry(key)
        .and_modify(|ranks| ranks.extend(&value))
        .or_insert(value);
    }
  }

  fn extend_expertise_freeform(&mut self, other: &Vec<FreeformAttribute>) {
    let mut other_iter = other.clone().into_iter();
    let mut current_result = other_iter.next();
    for freeform in self.expertise_freeform.iter_mut() {
      let Some(ref result) = current_result else {
        continue;
      };
      if !freeform.name.eq(&result.name) {
        continue;
      }
      freeform.attribute.extend(&result.attribute);
      current_result = other_iter.next();
    }
    self.expertise_freeform.extend(other_iter);
  }

  pub fn set(&mut self, attribute_class: AttributeClass, rank_class: RankClass, value: i32) {
    self.rank_map.insert((attribute_class, rank_class), value);
  }

  pub fn unset(&mut self, attribute_class: AttributeClass, rank_class: RankClass) {
    self.rank_map.remove(&(attribute_class, rank_class));
  }

  pub fn get(&self, attribute_class: AttributeClass, rank_class: RankClass) -> i32 {
    self.rank_map.get(&(attribute_class, rank_class)).cloned().unwrap_or(0)
  }

  pub fn get_effective(&self, attribute: &CharacterAttribute) -> i32 {
    return self.get_ranks(attribute) + self.get_specialization(attribute);
  }

  pub fn get_ranks(&self, attribute: &CharacterAttribute) -> i32 {
    return unpack_rank(self.ranks.get(attribute))
  }

  pub fn get_specialization(&self, attribute: &CharacterAttribute) -> i32 {
    return unpack_spec(self.ranks.get(attribute))
  }

  pub fn set_rank(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let entry = self.ranks.entry(attribute).or_default();
    entry.ranks = Some(ranks);
  }

  pub fn unset_rank(&mut self, attribute: &CharacterAttribute) {
    self.ranks.remove(attribute);
  }

  pub fn set_specialization(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let entry = self.ranks.entry(attribute).or_default();
    entry.specialization = Some(ranks);
  }

  fn get_attribute_leeways(&self, modifiers: &ModifierSet) -> (i32, i32, Vec<Counter>) {
    let capability_min = modifiers.get(&ModifierClass::CapabilityRank);
    let defense_min = modifiers.get(&ModifierClass::DefenseRank);
    let attribute_cap = capability_min + defense_min + modifiers.get(&ModifierClass::AttributeRank);
    let (cap_count, def_count) = self.get_attribute_counts();
    let cap_leeway = attribute_cap - (def_count.max(defense_min) + cap_count);
    let def_leeway = attribute_cap - (cap_count.max(capability_min) + def_count);
    let counters = vec![
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

  fn get_specialization_leeway(&self, modifiers: &ModifierSet) -> (i32, i32, Vec<Counter>) {
    let cap_spec_pool = modifiers.get(&ModifierClass::CapabilitySpecialization);
    let def_spec_pool = modifiers.get(&ModifierClass::DefenseSpecialization);
    let (cap_spec_count, def_spec_count) = self.get_specialization_counts();
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
    let ranks_standard = self
      .expertise_standard
      .iter()
      .map(|(_, expertise)| expertise.ranks.unwrap_or(0))
      .sum::<i32>();
    let ranks_freeform = self
      .expertise_freeform
      .iter()
      .map(|expertise| expertise.attribute.ranks.unwrap_or(0))
      .sum::<i32>();
    let spec_standard = self
      .expertise_standard
      .iter()
      .map(|(_, expertise)| expertise.specialization.unwrap_or(0))
      .sum::<i32>();
    let spec_freeform = self
      .expertise_freeform
      .iter()
      .map(|expertise| expertise.attribute.specialization.unwrap_or(0))
      .sum::<i32>();
    (
      ranks_standard + ranks_freeform,
      spec_standard + spec_freeform,
    )
  }

  pub fn set_expertise_name(&mut self, index: usize, name: String) {
    if index > self.expertise_freeform.len() {
      return;
    }
    self.expertise_freeform[index].name = name.clone();
    let trimmed = name.trim();
    match (
      index + 1 == self.expertise_freeform.len(),
      trimmed.is_empty(),
    ) {
      (true, false) => {
        self.expertise_freeform.push(FreeformAttribute::empty());
      },
      (false, true) => {
        self.expertise_freeform.remove(index);
      },
      _ => (),
    }
  }

  pub fn set_expertise_starndard_ranks(&mut self, expertise: StandardExpertise, value: i32) {
    let attribute = self.expertise_standard.entry(expertise).or_default();
    attribute.ranks = Some(value);
  }

  pub fn set_expertise_standard_specialization(
    &mut self, expertise: StandardExpertise, value: i32,
  ) {
    let attribute = self.expertise_standard.entry(expertise).or_default();
    attribute.specialization = Some(value);
  }

  pub fn set_expertise_freeform_ranks(&mut self, index: usize, value: i32) {
    if index > self.expertise_freeform.len() {
      return;
    }
    self.expertise_freeform[index].attribute.ranks = Some(value);
  }

  pub fn set_expertise_freeform_specialization(&mut self, index: usize, value: i32) {
    if index > self.expertise_freeform.len() {
      return;
    }
    self.expertise_freeform[index].attribute.specialization = Some(value);
  }

  pub fn get_resource_ranks(&self, resource: &ResourcePool) -> i32 {
    return self.resources.get(resource).copied().unwrap_or(0);
  }
}

impl LevelSelections {
  pub fn get_effective_physique_fortitude(&self) -> (i32, i32) {
    let physique = self.attributes.get_effective(&CharacterAttribute::Physique);
    let fortitude = self
      .attributes
      .get_effective(&CharacterAttribute::Fortitude);
    return (physique, fortitude);
  }
}

impl CharacterBuild {
  pub fn set_attribute(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.set_rank(attribute, ranks);
  }

  pub fn unset_attribute(&mut self, attribute: CharacterAttribute) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.unset_rank(&attribute);
  }

  pub fn set_specialization(&mut self, attribute: CharacterAttribute, ranks: i32) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_specialization(attribute, ranks);
  }

  // pub fn unset_specialization(&mut self, attribute: CharacterAttribute) {
  //   let level_selection = self.get_current_mut();
  //   level_selection
  //     .attributes
  //     .set_specialization(attribute, ranks);
  // }

  pub fn set_expertise_name(&mut self, index: usize, name: String) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.set_expertise_name(index, name);
  }

  pub fn set_expertise_standard_ranks(&mut self, expertise: StandardExpertise, value: i32) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_expertise_starndard_ranks(expertise, value);
  }

  pub fn set_expertise_standard_specialization(
    &mut self, expertise: StandardExpertise, value: i32,
  ) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_expertise_standard_specialization(expertise, value);
  }

  pub fn set_expertise_freeform_ranks(&mut self, index: usize, value: i32) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_expertise_freeform_ranks(index, value);
  }

  pub fn set_expertise_freeform_specialization(&mut self, index: usize, value: i32) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_expertise_freeform_specialization(index, value);
  }

  pub fn set_resource_points(&mut self, resource: ResourcePool, value: i32) {
    let level_selection = self.get_current_mut();
    let entry = level_selection
      .attributes
      .resources
      .entry(resource)
      .or_default();
    *entry = value;
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
    let (previous_selections, current_selections) = self.get_selection_pair();
    let (cap_leeway, def_leeway, mut counters) = current_selections
      .attributes
      .get_attribute_leeways(&modifiers);
    let (cap_spec_leeway, def_spec_leeway, counters_spec) = current_selections
      .attributes
      .get_specialization_leeway(&modifiers);
    counters.extend(counters_spec);
    let attribute_ranges = CharacterAttribute::iter()
      .map(|attribute| {
        let ranks = current_selections.attributes.get_ranks(attribute);
        let previous_ranks = previous_selections.attributes.get_ranks(attribute);
        let spec = current_selections.attributes.get_specialization(attribute);
        let previous_spec = previous_selections.attributes.get_specialization(attribute);
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
          NumericRange::new(previous_ranks, ranks, attribute_max),
          NumericRange::new(previous_spec, spec, attribute_spec_max),
        )
      })
      .collect();
    return (attribute_ranges, counters);
  }

  pub fn get_expertise_constraints(
    &self,
  ) -> (
    Vec<(StandardExpertise, NumericRange, NumericRange)>,
    Vec<(usize, String, NumericRange, NumericRange)>,
    Vec<Counter>,
  ) {
    let modifiers = self.get_current_modifiers();
    let rank_max = modifiers.get(&ModifierClass::RankMax);
    let spec_max = modifiers.get(&ModifierClass::SpecializationMax);
    let (rank_leeway, spec_leeway, counters) = self.get_expertise_leeway(&modifiers);
    let (_previous_selections, current_selections) = self.get_selection_pair();
    let current_standard_expertise = current_selections.attributes.expertise_standard;
    let previous_standard_expertise = _previous_selections.attributes.expertise_standard;
    let expertise_standard_ranges = StandardExpertise::iter()
      .map(|expertise| {
        let current_attribute = current_standard_expertise
          .get(expertise)
          .cloned()
          .unwrap_or_default();
        let previous_attribute = previous_standard_expertise
          .get(expertise)
          .cloned()
          .unwrap_or_default();
        let ranks = current_attribute.ranks.unwrap_or(0);
        let spec = current_attribute.specialization.unwrap_or(0);
        (
          expertise.clone(),
          NumericRange::new(previous_attribute.ranks.unwrap_or(0), ranks, rank_max.min(ranks + rank_leeway)),
          NumericRange::new(previous_attribute.specialization.unwrap_or(0), spec, spec_max.min(spec + spec_leeway)),
        )
      })
      .collect();
    let expertise_freeform_ranges = current_selections
      .attributes
      .expertise_freeform
      .iter()
      .enumerate()
      .map(|(index, expertise)| {
        let ranks = expertise.attribute.ranks.unwrap_or(0);
        let spec = expertise.attribute.specialization.unwrap_or(0);
        (
          index,
          expertise.name.clone(),
          NumericRange::new(0, ranks, rank_max.min(ranks + rank_leeway)),
          NumericRange::new(0, spec, spec_max.min(spec + spec_leeway)),
        )
      })
      .collect();
    return (
      expertise_standard_ranges,
      expertise_freeform_ranges,
      counters,
    );
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

  pub fn get_innate_pool_ranges(
    &self, modifiers: &ModifierSet, level_selection: &LevelSelections,
  ) -> (
    Vec<(Flow, i32, Vec<(ResourcePool, i32, Option<NumericRange>)>)>,
    Vec<Counter>,
  ) {
    return calculate_pool_ranges(
      modifiers,
      level_selection,
      Flow::Innate,
      &ModifierClass::InnateFlow,
      &ModifierClass::InnatePool,
      &ModifierClass::InnatePoolAll,
      ModifierClass::innate_pool_iter(),
    );
  }

  pub fn get_resonance_pool_ranges(
    &self, modifiers: &ModifierSet, level_selection: &LevelSelections,
  ) -> (
    Vec<(Flow, i32, Vec<(ResourcePool, i32, Option<NumericRange>)>)>,
    Vec<Counter>,
  ) {
    return calculate_pool_ranges(
      modifiers,
      level_selection,
      Flow::Resonance,
      &ModifierClass::ResonanceFlow,
      &ModifierClass::ResonancePool,
      &ModifierClass::ResonancePoolAll,
      ModifierClass::resonance_pool_iter(),
    );
  }

  pub fn get_magic_pool_ranges(
    &self, modifiers: &ModifierSet, level_selection: &LevelSelections,
  ) -> (
    Vec<(Flow, i32, Vec<(ResourcePool, i32, Option<NumericRange>)>)>,
    Vec<Counter>,
  ) {
    return calculate_pool_ranges(
      modifiers,
      level_selection,
      Flow::Magic,
      &ModifierClass::MagicFlow,
      &ModifierClass::ManaPool,
      &ModifierClass::ManaPoolAll,
      ModifierClass::magic_pool_iter(),
    );
  }

  pub fn get_flow_resource_selectors(
    &self,
  ) -> (
    Vec<(Flow, i32, Vec<(ResourcePool, i32, Option<NumericRange>)>)>,
    Vec<Counter>,
  ) {
    let modifiers = self.get_current_modifiers();
    let level_selection = self.current_selection();
    let mut flows = Vec::new();
    let mut counters = Vec::new();
    let ranges_by_flow = [
      self.get_innate_pool_ranges(&modifiers, &level_selection),
      self.get_resonance_pool_ranges(&modifiers, &level_selection),
      self.get_magic_pool_ranges(&modifiers, &level_selection),
    ];
    for (flow, counter) in ranges_by_flow {
      flows.extend(flow);
      counters.extend(counter);
    }
    return (flows, counters);
  }
}

fn calculate_pool_ranges<'a, T: Iterator<Item = &'a (ModifierClass, ResourcePool)>>(
  modifiers: &ModifierSet, level_selection: &LevelSelections, flow: Flow,
  flow_modifier: &ModifierClass, rank_modifier: &ModifierClass, rank_all_modifier: &ModifierClass,
  pool_iter: T,
) -> (
  Vec<(Flow, i32, Vec<(ResourcePool, i32, Option<NumericRange>)>)>,
  Vec<Counter>,
) {
  let flow_size = modifiers.get(flow_modifier);
  if flow_size == 0 {
    return (Vec::new(), Vec::new());
  }
  let pool_ranks = modifiers.get(rank_modifier);
  let pool_ranks_all = modifiers.get(rank_all_modifier);
  let mut current_ranks = 0;
  let mut pools: Vec<_> = pool_iter
    .filter_map(|(resource_modifier, resource)| {
      let starting_pool = modifiers.get(resource_modifier);
      match starting_pool > 0 {
        true => {
          let assigned_ranks = level_selection.attributes.get_resource_ranks(resource);
          current_ranks += assigned_ranks;
          Some((
            resource.to_owned(),
            starting_pool + pool_ranks_all,
            assigned_ranks,
          ))
        },
        false => None,
      }
    })
    .collect();
  if pools.len() == 1 {
    pools[0].1 += pool_ranks;
    pools[0].2 = 0;
  }
  let counters = if pools.len() <= 1 || pool_ranks == 0 {
    Vec::new()
  } else {
    vec![Counter {
      title: flow.to_string(),
      current: current_ranks,
      max: pool_ranks,
      ..Default::default()
    }]
  };
  let leeway = pool_ranks - current_ranks;
  let pool_selection_ranges: Vec<_> = pools
    .iter()
    .map(|(resource, base_size, ranks)| {
      (
        resource.clone(),
        *base_size,
        match counters.len() > 0 {
          true => Some(NumericRange::new(0, *ranks, ranks + leeway)),
          false => None,
        },
      )
    })
    .collect();
  return (vec![(flow, flow_size, pool_selection_ranges)], counters);
}

#[component]
pub fn AttributeSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let (attribute_constraints, counters) = build_signal().get_attribute_constraints();
  let (cap_split, def_split) = attribute_constraints.split_at(4);
  let capabilities_constraints = cap_split.to_owned();
  let defense_constraints = def_split.to_owned();
  let (expertise_standard_constraints, expertise_freeform_constraints, expertise_counters) =
    build_signal().get_expertise_constraints();
  let (flows, flow_counters) = build_signal().get_flow_resource_selectors();
  return rsx! {
    SectionBar {
      title: "Attributes",
      bar: rsx! {
        for counter in counters { CounterBadge{ counter } }
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
          for (attribute, rank_range, spec_range) in capabilities_constraints {
            AttributeRow{build_signal, attribute, rank_range, spec_range}
          }
          div { class: "subtitle left spacer", "Defenses" }
          div { class: "uv-third sink", "Ranks" }
          div { class: "sink", "Spec" }
          for (attribute, rank_range, spec_range) in defense_constraints {
            AttributeRow{build_signal, attribute, rank_range, spec_range}
          }
        }
        div {
          class: "grid dim-resource-chart padded-grid",
          div { class: "subtitle left", "Expertise" }
          div { class: "uv-third sink", "Ranks" }
          div { class: "sink", "Spec" }
          for (expertise, rank_range, spec_range) in expertise_standard_constraints {
            ExpertiseStandardRow {build_signal, expertise, rank_range, spec_range}
          }
          for (index, title, rank_range, spec_range) in expertise_freeform_constraints {
            ExpertiseFreeformRow {build_signal, index, title, rank_range, spec_range}
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
  title: Element, total: Element, rank_range: NumericRange, rank_handler: Callback<i32>,
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
  let unset_rank = rank_range.min;
  let unset_spec = spec_range.min;
  return rsx! {
    RankSpecializtionRow {
      title: rsx! { div { class: "left", "{attribute}" } },
      total: match &display {
        RankDisplay::Bonus => rsx! { div { Modifier { value } } },
        RankDisplay::Defense => rsx! { div { "{value}" } },
      },
      rank_range, 
      rank_handler: move |value: i32| {
        let mut new_build = build_signal();
        if value == unset_rank {
          new_build.unset_attribute(attribute.clone());
        } else {
          new_build.set_attribute(attribute.clone(), value);
        }
        build_signal.set(new_build);
      },
      spec_range,
      spec_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_specialization(attribute_spec.clone(), value);
        build_signal.set(new_build);
      }
    }
  };
}

#[component]
pub fn ExpertiseStandardRow(
  mut build_signal: Signal<CharacterBuild>, expertise: StandardExpertise, rank_range: NumericRange,
  spec_range: NumericRange,
) -> Element {
  let value = rank_range.value + spec_range.value;
  return rsx! {
    RankSpecializtionRow {
      title: rsx! { div { "{expertise}" } },
      total: rsx! { div { Modifier { value } } },
      rank_range, spec_range,
      rank_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_expertise_standard_ranks(expertise, value);
        build_signal.set(new_build);
      },
      spec_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_expertise_standard_specialization(expertise, value);
        build_signal.set(new_build);
      }
    }
  };
}

#[component]
pub fn ExpertiseFreeformRow(
  mut build_signal: Signal<CharacterBuild>, index: usize, title: String, rank_range: NumericRange,
  spec_range: NumericRange,
) -> Element {
  let value = rank_range.value + spec_range.value;
  return rsx! {
    RankSpecializtionRow {
      title: rsx! {
        input {
          class: "input full left",
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
        new_build.set_expertise_freeform_ranks(index, value);
        build_signal.set(new_build);
      },
      spec_handler: move |value: i32| {
        let mut new_build = build_signal();
        new_build.set_expertise_freeform_specialization(index, value);
        build_signal.set(new_build);
      }
    }
  };
}

#[component]
pub fn FlowBlock(
  mut build_signal: Signal<CharacterBuild>, flow: Flow, flow_size: i32,
  resource_pools: Vec<(ResourcePool, i32, Option<NumericRange>)>,
) -> Element {
  return rsx! {
    div {
      class: "grid dim-resource-chart padded-grid",
      div { class: "highlight left", "{flow}" }
      div { class: "highlight", "{flow_size}" }
      for (resource, base_size, range) in resource_pools {
        ResourceRow { build_signal, resource, base_size, range }
      }
    }
  };
}

#[component]
pub fn ResourceRow(
  mut build_signal: Signal<CharacterBuild>, resource: ResourcePool, base_size: i32,
  range: Option<NumericRange>,
) -> Element {
  let total_pool = base_size + range.clone().map(|range| range.value).unwrap_or(0);
  return rsx! {
    div { class: "uv-first left", "{resource}" }
    div { "{total_pool}" }
    if let Some(range) = range {
      NumericInput {
        class: "input big-text bumper",
        range: range,
        value_handler: move |value: i32| {
          let mut new_build = build_signal();
          new_build.set_resource_points(resource.clone(), value);
          build_signal.set(new_build);
        }
      }
    }
  };
}
