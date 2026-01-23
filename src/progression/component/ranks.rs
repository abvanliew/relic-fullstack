use dioxus::prelude::*;

use crate::character::prelude::*;
use crate::modifiers::ModifierClass;
use crate::rules::components::Modifier;
use crate::progression::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RankSignal {
  pub rank: Signal<i32>,
  pub max: Signal<bool>,
}

impl Default for RankSignal {
  fn default() -> Self {
    let rank = use_signal( || 0 );
    let max = use_signal( || false );
    Self { rank, max }
  }
}

impl RankSignal {
  pub fn max_value( &self ) -> i32 { if (self.max)() { 1 } else { 0 } }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RankSelections {
  pub physique: RankSignal,
  pub warfare: RankSignal,
  pub spirit: RankSignal,
  pub manipulation: RankSignal,
  pub tenacity: RankSignal,
  pub fortitude: RankSignal,
  pub resolve: RankSignal,
  pub insight: RankSignal,
  pub expertise: Signal<Vec<(String,RankSignal)>>,
  pub anointment_pool: Signal<i32>,
  pub animalism_pool: Signal<i32>,
  pub sanguine_pool: Signal<i32>,
  pub rage_pool: Signal<i32>,
}

impl Default for RankSelections {
  fn default() -> Self {
    let expertise = use_signal( || Vec::new() );
    let anointment_pool = use_signal( || 0 );
    let animalism_pool = use_signal( || 0 );
    let sanguine_pool = use_signal( || 0 );
    let rage_pool = use_signal( || 0 );
    Self {
      physique: Default::default(),
      warfare: Default::default(),
      spirit: Default::default(),
      manipulation: Default::default(),
      tenacity: Default::default(),
      fortitude: Default::default(),
      resolve: Default::default(),
      insight: Default::default(),
      expertise,
      anointment_pool,
      animalism_pool,
      sanguine_pool,
      rage_pool,
    }
  }
}

impl RankSelections {
  pub fn get_signals( 
    &self,
    attribute: &CharacterAttribute,
  ) -> ( Signal<i32>, Signal<bool> ) {
    let rank_signal = match attribute {
      CharacterAttribute::Physique => self.physique.clone(),
      CharacterAttribute::Warfare => self.warfare.clone(),
      CharacterAttribute::Spirit => self.spirit.clone(),
      CharacterAttribute::Manipulation => self.manipulation.clone(),
      CharacterAttribute::Tenacity => self.tenacity.clone(),
      CharacterAttribute::Fortitude => self.fortitude.clone(),
      CharacterAttribute::Resolve => self.resolve.clone(),
      CharacterAttribute::Insight => self.insight.clone(),
    };
    return ( rank_signal.rank, rank_signal.max );
  }

  pub fn rank_count( &self ) -> i32 {
    return (self.physique.rank)() + (self.warfare.rank)()
    + (self.spirit.rank)() + (self.manipulation.rank)()
    + (self.tenacity.rank)() + (self.fortitude.rank)()
    + (self.resolve.rank)() + (self.insight.rank)()
  }

  pub fn capacity_max_count( &self ) -> i32 {
    return self.physique.max_value() + self.warfare.max_value()
    + self.spirit.max_value() + self.manipulation.max_value();
  }

  pub fn defense_max_count( &self ) -> i32 {
    return self.tenacity.max_value() + self.fortitude.max_value()
    + self.resolve.max_value() + self.insight.max_value();
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StaticRanks {
  pub hp: i32,
  pub constituion: i32,
  pub speed: i32,
  pub dash: i32,
}

#[component]
pub fn CharacterRanks(
  rank_selections: RankSelections,
  min_rank: i32,
  max_rank: i32,
  attribute_ranks: i32,
  capability_max_ranks: i32,
  defense_max_ranks: i32,
  innate_flow: i32,
  innate_ranks: i32,
  innate_all_ranks: i32,
  innate_pools: Vec<(ModifierClass,i32)>,
  static_ranks: StaticRanks,
) -> Element {
  let attributes = CharacterAttribute::ordered();
  let rank_count = use_memo(
    use_reactive( &rank_selections, |rank_selections|
      rank_selections.rank_count()
    )
  );
  let remaining_ranks = attribute_ranks - rank_count();
  let capacity_max_count = use_memo(
    use_reactive( &rank_selections, |rank_selections|
      rank_selections.capacity_max_count()
    )
  );
  let remaining_capacity_max = capability_max_ranks - capacity_max_count();
  let defense_max_count = use_memo(
    use_reactive( &rank_selections, |rank_selections|
      rank_selections.defense_max_count()
    )
  );
  let remaining_defense_max = defense_max_ranks - defense_max_count();
  
  let mut resource_selectors: Vec<Element> = Vec::new();
  let innate_current_ranks = (rank_selections.animalism_pool)() + (rank_selections.anointment_pool)()
  + (rank_selections.sanguine_pool)() + (rank_selections.rage_pool)();
  let innate_remaining_ranks = innate_ranks - innate_current_ranks;
  let innate_selector = innate_pools.len() > 1 && innate_ranks > 0;
  if innate_flow > 0 {
    resource_selectors.push( rsx! {
      div { class: "uv-full highlight spacer", "Resources" }
    } );
  }
  if innate_flow > 0 {
    resource_selectors.push( rsx! {
      div { class: "uv-first underline", "Innate Flow" }
      div { "{innate_flow}" }
    } );
  }
  for ( class, pool ) in innate_pools {
    let ( title, ranks_base ) = match &class {
      ModifierClass::AnointmentPool => ( "Anointment", Some( rank_selections.anointment_pool ) ),
      ModifierClass::AnimalismPool => ( "Animalism", Some( rank_selections.animalism_pool ) ),
      ModifierClass::SanguinePool => ( "Sanguine", Some( rank_selections.sanguine_pool ) ),
      ModifierClass::RagePool => ( "Rage", Some( rank_selections.rage_pool ) ),
      _ => ( "other", None ),
    };
    let ranks_optional = if innate_selector { ranks_base } else { None };
    let flat_bonus = pool + innate_all_ranks + if innate_selector { 0 } else { innate_ranks };
    resource_selectors.push( rsx! {
      ResourceSelector { title, ranks_optional, min_rank: 0, remaining_ranks: innate_remaining_ranks, flat_bonus }
    } );
  }
  
  let hp = static_ranks.hp;
  let constitution = static_ranks.constituion;
  let speed = static_ranks.speed;
  let dash = static_ranks.dash;
  return rsx! {
    div {
      class: "column",
      div { "Attribute ranks remaining: {remaining_ranks} / {attribute_ranks}" }
      if capability_max_ranks > 0 {
        div { "Capacities to Increase Maximum: {remaining_capacity_max} / {capability_max_ranks}" }
      }
      if defense_max_ranks > 0 {
        div { "Defenses to Increase Maximum: {remaining_defense_max} / {defense_max_ranks}" }
      }
      if innate_selector {
        div { "Innate ranks to distribute: {innate_current_ranks} / {innate_ranks}" }
      }
    }
    div {
      class: "row",
      div {
        class: "column",
        div {
          class: "grid dim-quad",
          div { class: "uv-second", "Value" }
          div { "Ranks" }
          div { "Max" }
          for attribute in attributes {
            RankSelector {
              attribute: attribute.clone(),
              rank_selections: rank_selections.clone(),
              min_rank, max_rank,
              remaining_ranks,
              remaining_max: if attribute.is_capacity() { remaining_capacity_max } else { remaining_defense_max },
            }
          }
          for resource in resource_selectors {
            {resource}
          }
        }
      }
      div {
        class: "column",
        div { "HP {hp}" }
        div { "Constitution {constitution}" }
        div { "Speed {speed}" }
        div { "Dash {dash}" }
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankDisplay {
  Bonus,
  Defense,
}

#[component]
pub fn RankSelector(
  attribute: CharacterAttribute,
  rank_selections: RankSelections,
  min_rank: i32,
  max_rank: i32,
  remaining_ranks: i32,
  remaining_max: i32,
) -> Element {
  let title = attribute.to_string();
  let display = attribute.display_as();
  let (
    mut rank,
    mut max_increase,
  ) = rank_selections.get_signals( &attribute );
  let max_value = if max_increase() { 1 } else { 0 };
  let value = rank() + max_value;
  let min_rank = min_rank + max_value;
  let max_rank = ( max_rank + max_value ).min( remaining_ranks + rank() );
  let disabled = remaining_max <= 0 && !max_increase();
  return rsx! {
    div { "{title}" }
    match &display {
      RankDisplay::Bonus => rsx! { div { Modifier { value } } },
      RankDisplay::Defense => rsx! { div { "{value + BASE_DEFENSE}" } },
    }
    input {
      class: "input", type: "number",
      value: rank() + max_value, min: min_rank, max: max_rank,
      oninput: move |event| {
        let value = event.value().parse::<i32>()
        .unwrap_or_default()
        .min(max_rank).max(min_rank);
        rank.set(value - max_value);
      },
      onclick: move |event| {
        event.stop_propagation();
      }
    }
    input {
      r#type: "checkbox",
      checked: max_increase(),
      disabled,
      oninput: move |_| {
        if !disabled {
          max_increase.set( !max_increase() );
        }
      }
    }
  }
}

#[component]
pub fn ResourceSelector(
  title: String,
  ranks_optional: Option<Signal<i32>>,
  min_rank: i32,
  remaining_ranks: i32,
  flat_bonus: i32,
) -> Element {
  let value = flat_bonus + match ranks_optional {
    Some( ranks ) => ranks(),
    _ => 0,
  };
  let max_rank = remaining_ranks + match ranks_optional {
    Some( ranks ) => ranks(),
    _ => 0
  };
  return rsx! {
    div { class: "uv-first", "{title}" }
    div { "{value}" }
    if let Some( mut ranks ) = ranks_optional {
      input {
        class: "input", type: "number",
        value: ranks(), min: min_rank, max: max_rank,
        oninput: move |event| {
          let value = event.value().parse::<i32>()
          .unwrap_or_default()
          .min(max_rank).max(min_rank);
          ranks.set(value);
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
    }
  }
}
