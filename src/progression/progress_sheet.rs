use dioxus::prelude::*;

use crate::modifiers::{ModifierClass, ModifierSet};
use crate::progression::track::{GrowthTrack, LevelTrack};
use crate::progression::training::TrainingClass;
use crate::rules::components::Modifier;


#[component]
pub fn LevelTable() -> Element {
  let levels = LevelTrack::compile_level_modifiers(12);
  rsx! {
    div {
      class: "table-grid padded-grid alt-background-8",
      LevelHeader {}
      for (index, (individual_level, running_total)) in levels.into_iter().enumerate() {
        LevelRow { index, individual_level, running_total }
      }
    }
  }
}


#[component]
pub fn LevelHeader() -> Element {
  rsx! {
    div { class: "uv-first underline", "Level" }
    div { class: "underline", "HP" }
    div { class: "underline", "Rank Maximum" }
    div { class: "underline", "Attribute Ranks" }
    div { class: "underline", "Expretise Ranks" }
    div { class: "underline", "Training Ranks" }
    div { class: "underline", "Maximum Paths" }
    div { class: "left underline", "Paths and Features" }
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
  let path_initiate_required = individual_level.get(&ModifierClass::InitiatePathRequired);
  let path_initiate_optional = individual_level.get(&ModifierClass::InitiatePathOptional);
  let path_initiate_max = path_initiate_required + path_initiate_optional;
  let path_initiate_running = running_total.get(&ModifierClass::InitiatePathRequired) + running_total.get(&ModifierClass::InitiatePathOptional);
  let path_journeyman_required = individual_level.get(&ModifierClass::JourneymanPathRequired);
  let path_journeyman_running = running_total.get(&ModifierClass::JourneymanPathOptional);
  let path_master_running = running_total.get(&ModifierClass::MasterPathOptional);
  let feature = individual_level.get(&ModifierClass::Feature) + path_initiate_max - path_initiate_required;
  let minor_feature = individual_level.get(&ModifierClass::MinorFeature);
  let mut features_choices: Vec<String> = Vec::new();
  if path_initiate_required > 0 {
    features_choices.push(format!("{path_initiate_required} Initiate Path"));
  }
  if path_journeyman_required > 0 {
    features_choices.push(format!("{path_journeyman_required} Journeyman Path"));
  }
  if path_initiate_optional > 0 && feature > 0 {
    let path_feature_overlap = path_initiate_optional.min( feature);
    features_choices.push(format!("{path_feature_overlap} Initiate Path or Feature"));
    if feature > path_initiate_optional {
      let net_feature = feature - path_initiate_optional;
      features_choices.push(if net_feature == 1 {"1 Feature".into()} else {format!( "{net_feature} Features" )});
    }
  } else if feature > 0 {
    features_choices.push(if feature == 1 {"1 Feature".into()} else {format!( "{feature} Features" )});
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
    div { "{path_initiate_running} / {path_journeyman_running} / {path_master_running}" }
    div { class: "left", "{feature_text}" }
  }
}

#[component]
pub fn TrainingTables() -> Element {
  rsx! {
    div {
      class: "row-wrap",
      TrainingTable { training_class: TrainingClass::Adept }
      TrainingTable { training_class: TrainingClass::Endurance }
      TrainingTable { training_class: TrainingClass::Expert }
    }
    div {
      class: "row-wrap",
      TrainingTable { training_class: TrainingClass::Innate }
      TrainingTable { training_class: TrainingClass::Resonance }
      TrainingTable { training_class: TrainingClass::Magic }
    }
  }
}

#[component]
pub fn TrainingTable(training_class: TrainingClass) -> Element {
  let modifier_keys = match &training_class {
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
  let table_class = match modifier_keys.len() {
    3 => "alt-background-4",
    4 => "alt-background-5",
    _ => "",
  };
  rsx! {
    div {
      class: "table-grid padded-grid {table_class}",
      TrainingHeader { training_class }
      for rank in 1..=12 {
        TrainingRow { rank, training_class, modifier_keys: modifier_keys.clone() }
      }
    }
  }
}

#[component]
pub fn TrainingHeader(training_class: TrainingClass) -> Element {
  return match &training_class {
    TrainingClass::Adept => rsx! {
      div { class: "uv-first underline", "Adept Rank" }
      div { class: "underline", "HP" }
      div { class: "underline", "Capability Ranks" }
      div { class: "underline", "Capability Specialization" }
    },
    TrainingClass::Endurance => rsx! {
      div { class: "uv-first underline", "Endurance Rank" }
      div { class: "underline", "HP" }
      div { class: "underline", "Defense Ranks" }
      div { class: "underline", "Defense Specialization" }
    },
    TrainingClass::Expert => rsx! {
      div { class: "uv-first underline", "Expert Rank" }
      div { class: "underline", "HP" }
      div { class: "underline", "Expertise Ranks" }
      div { class: "underline", "Expertise Specialization" }
    },
    TrainingClass::Innate => rsx! {
      div { class: "uv-first underline", "Innate Rank" }
      div { class: "underline", "HP" }
      div { class: "underline", "Innate Flow" }
      div { class: "underline", "Innate Pool" }
      div { class: "underline", "All Innate Pools" }
    },
    TrainingClass::Resonance => rsx! {
      div { class: "uv-first underline", "Resonance Rank" }
      div { class: "underline", "HP" }
      div { class: "underline", "Resonance Flow" }
      div { class: "underline", "Resonance Pool" }
      div { class: "underline", "All Resonance Pools" }
    },
    TrainingClass::Magic => rsx! {
      div { class: "uv-first underline", "Magic Rank" }
      div { class: "underline", "Magic Flow" }
      div { class: "underline", "Minor Mana Pool" }
      div { class: "underline", "Moderate Mana Pool" }
      div { class: "underline", "Major Mana Pool" }
    },
  };
}

#[component]
pub fn TrainingRow(rank: i32, training_class: TrainingClass, modifier_keys: Vec<ModifierClass>) -> Element {
  let modifiers = GrowthTrack::class_at(&training_class, rank);
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
