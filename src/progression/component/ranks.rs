use dioxus::prelude::*;

use crate::character::prelude::*;
use crate::rules::components::Modifier;


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
}

impl Default for RankSelections {
  fn default() -> Self {
    let expertise = use_signal( || Vec::new() );
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

#[component]
pub fn CharacterRanks(
  rank_selections: RankSelections,
  min_rank: i32,
  max_rank: i32,
  attribute_ranks: i32,
  capability_max_ranks: i32,
  defense_max_ranks: i32,
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
  return rsx! {
    div { "Attribute ranks remaining: {remaining_ranks} / {attribute_ranks}" }
    if capability_max_ranks > 0 {
      div { "Capacities to Increase Maximum: {remaining_capacity_max} / {capability_max_ranks}" }
    }
    if defense_max_ranks > 0 {
      div { "Defenses to Increase Maximum: {remaining_defense_max} / {defense_max_ranks}" }
    }
    div {
      class: "grid dim-quad",
      div { class: "uv-second", "Ranks" }
      div { "Max" }
      div { "Value" }
      for attribute in attributes {
        RankSelector {
          attribute: attribute.clone(),
          rank_selections: rank_selections.clone(),
          min_rank, max_rank,
          remaining_ranks,
          remaining_max: if attribute.is_capacity() { remaining_capacity_max } else { remaining_defense_max },
        }
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
    match &display {
      RankDisplay::Bonus => rsx! { div { Modifier { value } } },
      RankDisplay::Defense => rsx! { div { "{value + BASE_DEFENSE}" } },
    }
  }
}
