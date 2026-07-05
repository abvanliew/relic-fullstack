use dioxus::prelude::*;

use crate::modifiers::{ModifierClass, ModifierSet};
use crate::progression::track::{GrowthTrack, LevelTrack};
use crate::progression::training::TrainingClass;
use crate::rules::components::Modifier;

#[component]
pub fn ProgressChart() -> Element {
  rsx! {
    LevelChartExplainer {}
    TrainingExplainer {}
    LevelTable {}
    TrainingTables {}
  }
}

#[component]
pub fn LevelTable() -> Element {
  let levels = LevelTrack::compile_level_modifiers(6);
  rsx! {
    div {
      class: "table-grid",
      LevelHeader {}
      for (index, (individual_level, running_total)) in levels.into_iter().enumerate() {
        LevelRow { index, individual_level, running_total }
      }
    }
  }
}

#[component]
pub fn LevelChartExplainer() -> Element {
  rsx! {
    div { "HP: The amount of health your character has." }
    div { "Rank Maximum: The maximum ranks you can allocate to a given attribute or expertise." }
    div { "Attribute Ranks: The number of ranks you can spend between your capabilities and defenses. At level 1 you must spend at least 8 ranks in capabilities and 8 ranks in defenses." }
    div { "Expertise Ranks: The number of ranks you have to spend on expertise." }
    div { "Training Ranks: The number of ranks you can select towards training. You cannot spend more ranks on training than your level." }
    div { "Paths and Features: Each character must select at least one path to learn. Optionally they can choose to forgo a Feature to learn a new path. Characters are limited to a maximum number of paths they cannot learn more than what is listed." }
  }
}

#[component]
pub fn LevelHeader() -> Element {
  rsx! {
    div { class: "uv-first", "Level" }
    div { "HP" }
    div { "Rank Maximum" }
    div { "Attribute Ranks" }
    div { "Expretise Ranks" }
    div { "Training Ranks" }
    div { "Maximum Paths" }
    div { class: "left", "Paths and Features" }
  }
}

#[component]
pub fn LevelRow(
  index: usize, individual_level: ModifierSet, running_total: ModifierSet,
) -> Element {
  let level = index + 1;
  let hp = individual_level.get(&ModifierClass::HP);
  let rank_max = running_total.get(&ModifierClass::RankMax);
  let attributes = individual_level.get(&ModifierClass::AttributeRank)
    + individual_level.get(&ModifierClass::CapabilityRank)
    + individual_level.get(&ModifierClass::DefenseRank);
  let expertises = individual_level.get(&ModifierClass::ExpertiseRank);
  let growth = individual_level.get(&ModifierClass::GrowthRanks);
  let path_min = individual_level.get(&ModifierClass::InitiatePathMin);
  let path_max = individual_level.get(&ModifierClass::InitiatePathMax);
  let path_max_running = running_total.get(&ModifierClass::InitiatePathMax);
  let feature = individual_level.get(&ModifierClass::Feature) + path_max - path_min;
  let minor_feature = individual_level.get(&ModifierClass::MinorFeature);
  let mut features_choices: Vec<String> = Vec::new();
  if path_min > 0 {
    features_choices.push(format!("{path_min} Path"));
  }
  if feature > 0 {
    features_choices.push(format!("{feature} Feature(s)"));
  }
  if minor_feature > 0 {
    features_choices.push(format!("{minor_feature} Minor Feature"));
  }
  let feature_text = features_choices.join(", ");
  rsx! {
    div { class: "uv-first", "{level}" }
    div { if level == 1 { "{hp}" } else { Modifier {value: hp} } }
    div { "{rank_max}" }
    div { if level == 1 { "{attributes}" } else { Modifier {value: attributes} } }
    div { if level == 1 { "{expertises}" } else { Modifier {value: expertises} } }
    div { if level == 1 { "{growth}" } else { Modifier {value: growth} } }
    div { "{path_max_running}" }
    div { class: "left", "{feature_text}" }
  }
}

#[component]
pub fn TrainingExplainer() -> Element {
  rsx! {
    div { "Specializations: Specialization ranks do not count against the normal rank limit for attributes and expertise. But each attribute or exptertise can have a maximum of 1 specialization point per tier of the character." }
  }
}

#[component]
pub fn TrainingTables() -> Element {
  rsx! {
    div {
      class: "row-wrap",
      TrainingTable { class: TrainingClass::Adept }
      TrainingTable { class: TrainingClass::Endurance }
      TrainingTable { class: TrainingClass::Expert }
      TrainingTable { class: TrainingClass::Innate }
      TrainingTable { class: TrainingClass::Resonance }
      TrainingTable { class: TrainingClass::Magic }
    }
  }
}

#[component]
pub fn TrainingTable(class: TrainingClass) -> Element {
  let modifier_keys = match &class {
    TrainingClass::Adept => vec![
      ModifierClass::HP,
      ModifierClass::CapabilityRank,
      ModifierClass::CapabilitySpecialization,
    ],
    TrainingClass::Endurance => vec![
      ModifierClass::HP,
      ModifierClass::DefenseRank,
      ModifierClass::DefenseSpecialization,
    ],
    TrainingClass::Expert => vec![
      ModifierClass::HP,
      ModifierClass::ExpertiseRank,
      ModifierClass::ExpertiseSpecialization,
    ],
    TrainingClass::Innate => vec![
      ModifierClass::HP,
      ModifierClass::InnateFlow,
      ModifierClass::InnatePool,
      ModifierClass::InnatePoolAll,
    ],
    TrainingClass::Resonance => vec![
      ModifierClass::HP,
      ModifierClass::ResonanceFlow,
      ModifierClass::ResonancePool,
      ModifierClass::ResonancePoolAll,
    ],
    TrainingClass::Magic => vec![
      ModifierClass::MagicFlow,
      ModifierClass::ManaPoolMinor,
      ModifierClass::ManaPoolModerate,
      ModifierClass::ManaPoolMajor,
    ],
  };
  rsx! {
    div {
      class: "table-grid",
      TrainingHeader { class }
      for rank in 1..=12 {
        TrainingRow { rank, class, modifier_keys: modifier_keys.clone() }
      }
    }
  }
}

#[component]
pub fn TrainingHeader(class: TrainingClass) -> Element {
  return match &class {
    TrainingClass::Adept => rsx! {
      div { class: "uv-first", "Adept Rank" }
      div { "HP" }
      div { "Capability Ranks" }
      div { "Capability Specialization" }
    },
    TrainingClass::Endurance => rsx! {
      div { class: "uv-first", "Endurance Rank" }
      div { "HP" }
      div { "Defense Ranks" }
      div { "Defense Specialization" }
    },
    TrainingClass::Expert => rsx! {
      div { class: "uv-first", "Expert Rank" }
      div { "HP" }
      div { "Expertise Ranks" }
      div { "Expertise Specialization" }
    },
    TrainingClass::Innate => rsx! {
      div { class: "uv-first", "Innate Rank" }
      div { "HP" }
      div { "Innate Flow" }
      div { "Innate Pool" }
      div { "All Innate Pools" }
    },
    TrainingClass::Resonance => rsx! {
      div { class: "uv-first", "Resonance Rank" }
      div { "HP" }
      div { "Resonance Flow" }
      div { "Resonance Pool" }
      div { "All Resonance Pools" }
    },
    TrainingClass::Magic => rsx! {
      div { class: "uv-first", "Magic Rank" }
      div { "Magic Flow" }
      div { "Minor Mana Pool" }
      div { "Moderate Mana Pool" }
      div { "Major Mana Pool" }
    },
  };
}

#[component]
pub fn TrainingRow(rank: i32, class: TrainingClass, modifier_keys: Vec<ModifierClass>) -> Element {
  let modifiers = GrowthTrack::class_at(&class, rank);
  let modifier_values = modifier_keys
    .iter()
    .map(|class| modifiers.get(class))
    .collect::<Vec<i32>>();
  rsx! {
    div { class: "uv-first", "{rank}" }
    for value in modifier_values {
      div { Modifier { value } }
    }
  }
}
