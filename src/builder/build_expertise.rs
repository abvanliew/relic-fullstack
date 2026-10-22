use bson::oid::ObjectId;
use dioxus::prelude::*;

use super::build_common::SectionBar;
use super::CharacterBuild;

use crate::builder::build_attributes::{AttributeClass, AttributeRanks, AttributeRow, RankClass, update_attribute};
use crate::builder::build_common::{Counter, CounterBadge};
use crate::builder::LevelSelections;
use crate::character::prelude::{
  Flow, StandardExpertise,
};
use crate::common::{NumericInput, NumericRange};
use crate::modifiers::{ModifierClass, ModifierSet};
use crate::rules::components::Modifier;
use crate::skill::prelude::ResourcePool;

impl AttributeRanks {
  pub fn new_freeform_object(&mut self) {
    self
      .freeform_objects
      .push((ObjectId::new(), Default::default()));
  }

  pub fn find_freeform_index(&self, object_id: &ObjectId) -> Option<usize> {
    let mut matched_index = None;
    for (index, (id, _)) in self.freeform_objects.iter().enumerate() {
      if id.eq(object_id) {
        matched_index = Some(index);
        break;
      }
    }
    return matched_index;
  }

  pub fn remove_freeform_object(&mut self, object_id: &ObjectId) {
    self.ranks.remove(&(
      AttributeClass::FreeformObject(object_id.clone()),
      RankClass::Base,
    ));
    self.ranks.remove(&(
      AttributeClass::FreeformObject(object_id.clone()),
      RankClass::Specialization,
    ));
    let Some(remove_index) = self.find_freeform_index(object_id) else {
      return;
    };
    self.freeform_objects.remove(remove_index);
  }

  pub fn get_pool(&self, resource: &ResourcePool) -> i32 {
    return self.get(&AttributeClass::Pool(*resource), &RankClass::Base);
  }

  pub fn set_freeform_name(&mut self, object_id: ObjectId, name: String) {
    let Some(index) = self.find_freeform_index(&object_id) else {
      return;
    };
    self.freeform_objects[index].1 = name;
  }

  fn get_expertise_leeway(&self, modifiers: &ModifierSet) -> (i32, i32, Vec<Counter>) {
    let rank_pool = modifiers.get(&ModifierClass::ExpertiseRank);
    let spec_pool = modifiers.get(&ModifierClass::ExpertiseSpecialization);
    let (rank_count, spec_count) = self.get_expertise_counts();
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

  pub fn get_expertise_counts(&self) -> (i32, i32) {
    let ranks_standard = StandardExpertise::iter()
      .map(|expertise| self.get(&AttributeClass::Expertise(*expertise), &RankClass::Base))
      .sum::<i32>();
    let spec_standard = StandardExpertise::iter()
      .map(|expertise| {
        self.get(
          &AttributeClass::Expertise(*expertise),
          &RankClass::Specialization,
        )
      })
      .sum::<i32>();
    let ranks_freeform = self
      .ranks
      .iter()
      .filter(
        |((attribute_class, rank_class), _)| match (attribute_class, rank_class) {
          (AttributeClass::FreeformObject(_), RankClass::Base) => true,
          _ => false,
        },
      )
      .map(|(_, value)| *value)
      .sum::<i32>();
    let spec_freeform = self
      .ranks
      .iter()
      .filter(
        |((attribute_class, rank_class), _)| match (attribute_class, rank_class) {
          (AttributeClass::FreeformObject(_), RankClass::Specialization) => true,
          _ => false,
        },
      )
      .map(|(_, value)| *value)
      .sum::<i32>();
    (
      ranks_standard + ranks_freeform,
      spec_standard + spec_freeform,
    )
  }
}

impl CharacterBuild {
  pub fn set_freeform_name(&mut self, object_id: ObjectId, name: String) {
    let level_selection = self.get_current_mut();
    level_selection
      .attributes
      .set_freeform_name(object_id, name);
  }

  pub fn new_freeform_object(&mut self) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.new_freeform_object();
  }

  pub fn remove_freeform_object(&mut self, object_id: &ObjectId) {
    let level_selection = self.get_current_mut();
    level_selection.attributes.remove_freeform_object(object_id);
  }

  fn get_current_expertise_counts(&self) -> (i32, i32) {
    let level_selection = self.current_selection();
    return level_selection.attributes.get_expertise_counts();
  }

  pub fn get_expertise_constraints(
    &self,
  ) -> (
    Vec<(AttributeClass, String, NumericRange, NumericRange)>,
    Vec<(AttributeClass, bool, String, NumericRange, NumericRange)>,
    Vec<Counter>,
  ) {
    let modifiers = self.get_current_modifiers();
    let rank_max = modifiers.get(&ModifierClass::RankMax);
    let spec_max = modifiers.get(&ModifierClass::SpecializationMax);
    let (previous_selections, current_selections) = self.get_selection_pair();
    let (rank_leeway, spec_leeway, counters) = current_selections
      .attributes
      .get_expertise_leeway(&modifiers);
    let expertise_standard = StandardExpertise::iter()
      .map(|expertise| {
        let attribute_class = AttributeClass::Expertise(*expertise);
        let ranks = current_selections
          .attributes
          .get(&attribute_class, &RankClass::Base);
        let spec = current_selections
          .attributes
          .get(&attribute_class, &RankClass::Specialization);
        let previous_ranks = previous_selections
          .attributes
          .get(&attribute_class, &RankClass::Base);
        let previous_spec = previous_selections
          .attributes
          .get(&attribute_class, &RankClass::Specialization);
        (
          attribute_class,
          expertise.to_string(),
          NumericRange::new(previous_ranks, ranks, rank_max.min(ranks + rank_leeway)),
          NumericRange::new(previous_spec, spec, spec_max.min(spec + spec_leeway)),
        )
      })
      .collect();
    let expertise_freeform = current_selections
      .attributes
      .freeform_objects
      .iter()
      .map(|(object_id, name)| {
        let renamable = match previous_selections
          .attributes
          .find_freeform_index(object_id)
        {
          None => true,
          _ => false,
        };
        let attribute_class = AttributeClass::FreeformObject(*object_id);
        let ranks = current_selections
          .attributes
          .get(&attribute_class, &RankClass::Base);
        let spec = current_selections
          .attributes
          .get(&attribute_class, &RankClass::Specialization);
        let previous_ranks = previous_selections
          .attributes
          .get(&attribute_class, &RankClass::Base);
        let previous_spec = previous_selections
          .attributes
          .get(&attribute_class, &RankClass::Specialization);
        (
          attribute_class,
          renamable,
          name.clone(),
          NumericRange::new(previous_ranks, ranks, rank_max.min(ranks + rank_leeway)),
          NumericRange::new(previous_spec, spec, spec_max.min(spec + spec_leeway)),
        )
      })
      .collect();
    return (expertise_standard, expertise_freeform, counters);
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
          let assigned_ranks = level_selection.attributes.get_pool(resource);
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

#[component]
pub fn ExpertiseFreeformRow(
  mut build_signal: Signal<CharacterBuild>, object_id: ObjectId, title: String,
  rank_range: NumericRange, spec_range: NumericRange,
) -> Element {
  let value = rank_range.value + spec_range.value;
  let unset_rank = rank_range.min;
  let unset_spec = spec_range.min;
  return rsx! {
    RankSpecializtionRow {
      title: rsx! {
        input {
          class: "input full left",
          value: "{title}",
          oninput: move |event| {
            event.stop_propagation();
            let mut new_build = build_signal();
            new_build.set_freeform_name(object_id, event.value());
            build_signal.set(new_build);
          }
        }
      },
      total: rsx! { div { Modifier { value } } },
      rank_range,
      rank_handler: move |value: i32| {
        update_attribute(value, build_signal, unset_rank, AttributeClass::FreeformObject(object_id), RankClass::Base)
      },
      spec_range,
      spec_handler: move |value: i32| {
        update_attribute(value, build_signal, unset_spec, AttributeClass::FreeformObject(object_id), RankClass::Specialization)
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
  let total_pool = base_size + range.as_ref().map_or(0, |range| range.value);
  let unset_rank = range.as_ref().map_or(0, |range| range.min);
  return rsx! {
    div { class: "uv-first left", "{resource}" }
    div { "{total_pool}" }
    if let Some(range) = range {
      NumericInput {
        class: "input big-text bumper",
        range: range,
        value_handler: move |value: i32| {
        update_attribute(value, build_signal, unset_rank, AttributeClass::Pool(resource), RankClass::Base)
        }
      }
    }
  };
}
