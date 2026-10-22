use dioxus::prelude::*;

use crate::modifiers::{ModifierClass, ModifierSet};
use crate::progression::track::{GrowthTrack, LevelTrack};
use crate::progression::training::DevelopmentClass;
use crate::rules::components::Modifier;

#[component]
pub fn LevelTable(#[props(default)] highlight_level: Option<i32>) -> Element {
  let levels = LevelTrack::compile_level_modifiers(18);
  rsx! {
    div {
      class: "table-grid padded-grid alt-background-8",
      LevelHeader {}
      for (index, (net, total)) in levels.into_iter().enumerate() {
        LevelRow { index, net, total, highlight_level }
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
    div { class: "underline", "Development Points" }
    div { class: "underline", "Maximum Paths" }
    div { class: "left underline", "Paths and Features" }
  }
}

#[component]
pub fn LevelRow(
  index: usize, net: ModifierSet, total: ModifierSet,
  #[props(default)] highlight_level: Option<i32>,
) -> Element {
  let level = index + 1;
  let hp = total.get(&ModifierClass::HP);
  let hp_net = net.get(&ModifierClass::HP);

  let rank_max = total.get(&ModifierClass::RankMax);
  let attributes = total.get(&ModifierClass::AttributeRank)
    + total.get(&ModifierClass::CapabilityRank)
    + total.get(&ModifierClass::DefenseRank);
  let attributes_net = net.get(&ModifierClass::AttributeRank)
    + net.get(&ModifierClass::CapabilityRank)
    + net.get(&ModifierClass::DefenseRank);

  let expertises = total.get(&ModifierClass::ExpertiseRank);
  let expertises_net = net.get(&ModifierClass::ExpertiseRank);
  let growth = total.get(&ModifierClass::DevelopmentPoints);
  let growth_net = net.get(&ModifierClass::DevelopmentPoints);
  let path_initiate_required = net.get(&ModifierClass::InitiatePathRequired);
  let path_initiate_optional = net.get(&ModifierClass::InitiatePathOptional);
  let path_initiate_max = path_initiate_required + path_initiate_optional;
  let path_initiate_running = total.get(&ModifierClass::InitiatePathRequired)
    + total.get(&ModifierClass::InitiatePathOptional);
  let path_journeyman_required = net.get(&ModifierClass::JourneymanPathRequired);
  let path_journeyman_running = total.get(&ModifierClass::JourneymanPathOptional);
  let path_master_running = total.get(&ModifierClass::MasterPathOptional);
  let feature = net.get(&ModifierClass::Feature) + path_initiate_max - path_initiate_required;
  let minor_feature = net.get(&ModifierClass::MinorFeature);
  let mut features_choices: Vec<String> = Vec::new();
  if path_initiate_required > 0 {
    features_choices.push(format!("{path_initiate_required} Initiate Path"));
  }
  if path_journeyman_required > 0 {
    features_choices.push(format!("{path_journeyman_required} Journeyman Path"));
  }
  if path_initiate_optional > 0 && feature > 0 {
    let path_feature_overlap = path_initiate_optional.min(feature);
    features_choices.push(format!("{path_feature_overlap} Initiate Path or Feature"));
    if feature > path_initiate_optional {
      let net_feature = feature - path_initiate_optional;
      features_choices.push(if net_feature == 1 {
        "1 Feature".into()
      } else {
        format!("{net_feature} Features")
      });
    }
  } else if feature > 0 {
    features_choices.push(if feature == 1 {
      "1 Feature".into()
    } else {
      format!("{feature} Features")
    });
  }
  if minor_feature > 0 {
    features_choices.push(format!("{minor_feature} Minor Feature"));
  }
  let feature_text = features_choices.join(", ");
  let highlight = match highlight_level {
    Some(target_level) => target_level == level as i32,
    None => false,
  };
  let mid_class = if highlight {
    "fill-height thin-border mid-cap selected row gap-xsmall middle"
  } else {
    "fill-height thin-padding row gap-xsmall middle"
  };
  rsx! {
    div { class: if highlight { "uv-first fill-height thin-border left-cap selected" } else {"uv-first fill-height thin-padding"}, "{level}" }
    div { class: mid_class, span { "{hp}" } if level > 1 && hp_net > 0 { Modifier { class: "small-text", value: hp_net, parenthesis: true } } }
    div { class: mid_class, "{rank_max}" }
    div { class: mid_class, span { "{attributes}" } if level > 1 && attributes_net > 0 { Modifier { class: "small-text", value: attributes_net, parenthesis: true} } }
    div { class: mid_class, span { "{expertises}" } if level > 1 && expertises_net > 0 { Modifier { class: "small-text", value: expertises_net, parenthesis: true} } }
    div { class: mid_class, span { "{growth}" } if level > 1 && growth_net > 0 { Modifier { class: "small-text", value: growth_net, parenthesis: true} } }
    div { class: mid_class, "{path_initiate_running} / {path_journeyman_running} / {path_master_running}" }
    div { class: if highlight {"fill-height left thin-border right-cap selected"} else {"fill-height left thin-padding"}, "{feature_text}" }
  }
}

#[component]
pub fn DevelopmentTables() -> Element {
  rsx! {
    div {
      class: "row-wrap",
      DevelopmentTable { development_class: DevelopmentClass::Adept }
      DevelopmentTable { development_class: DevelopmentClass::Endurance }
      DevelopmentTable { development_class: DevelopmentClass::Expert }
    }
    div {
      class: "row-wrap",
      DevelopmentTable { development_class: DevelopmentClass::Innate }
      DevelopmentTable { development_class: DevelopmentClass::Resonance }
      DevelopmentTable { development_class: DevelopmentClass::Magic }
    }
  }
}

#[component]
pub fn DevelopmentTable(
  development_class: DevelopmentClass, #[props(default)] highlight_rank: Option<i32>,
) -> Element {
  let modifier_keys = match &development_class {
    DevelopmentClass::Adept => vec![
      ModifierClass::HP,
      ModifierClass::CapabilityRank,
      ModifierClass::CapabilitySpecialization,
    ],
    DevelopmentClass::Endurance => vec![
      ModifierClass::HP,
      ModifierClass::DefenseRank,
      ModifierClass::DefenseSpecialization,
    ],
    DevelopmentClass::Expert => vec![
      ModifierClass::HP,
      ModifierClass::ExpertiseRank,
      ModifierClass::ExpertiseSpecialization,
    ],
    DevelopmentClass::Innate => vec![
      ModifierClass::HP,
      ModifierClass::InnateFlow,
      ModifierClass::InnatePool,
      ModifierClass::InnatePoolAll,
    ],
    DevelopmentClass::Resonance => vec![
      ModifierClass::HP,
      ModifierClass::ResonanceFlow,
      ModifierClass::ResonancePool,
      ModifierClass::ResonancePoolAll,
    ],
    DevelopmentClass::Magic => vec![
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
      DevelopmentHeader { development_class }
      for rank in 1..=12 {
        DevelopmentRow { rank, development_class, modifier_keys: modifier_keys.clone(), highlight_rank: highlight_rank.clone() }
      }
    }
  }
}

#[component]
pub fn DevelopmentHeader(development_class: DevelopmentClass) -> Element {
  return match &development_class {
    DevelopmentClass::Adept => rsx! {
      div { class: "uv-first underline", "Adept" }
      div { class: "underline", "HP" }
      div { class: "underline", "Capability Ranks" }
      div { class: "underline", "Capability Specialization" }
    },
    DevelopmentClass::Endurance => rsx! {
      div { class: "uv-first underline", "Endurance" }
      div { class: "underline", "HP" }
      div { class: "underline", "Defense Ranks" }
      div { class: "underline", "Defense Specialization" }
    },
    DevelopmentClass::Expert => rsx! {
      div { class: "uv-first underline", "Expert" }
      div { class: "underline", "HP" }
      div { class: "underline", "Expertise Ranks" }
      div { class: "underline", "Expertise Specialization" }
    },
    DevelopmentClass::Innate => rsx! {
      div { class: "uv-first underline", "Innate" }
      div { class: "underline", "HP" }
      div { class: "underline", "Innate Flow" }
      div { class: "underline", "Innate Pool" }
      div { class: "underline", "All Innate Pools" }
    },
    DevelopmentClass::Resonance => rsx! {
      div { class: "uv-first underline", "Resonance" }
      div { class: "underline", "HP" }
      div { class: "underline", "Resonance Flow" }
      div { class: "underline", "Resonance Pool" }
      div { class: "underline", "All Resonance Pools" }
    },
    DevelopmentClass::Magic => rsx! {
      div { class: "uv-first underline", "Magic" }
      div { class: "underline", "Magic Flow" }
      div { class: "underline", "Minor Mana Pool" }
      div { class: "underline", "Moderate Mana Pool" }
      div { class: "underline", "Major Mana Pool" }
    },
  };
}

#[component]
pub fn DevelopmentRow(
  rank: i32, development_class: DevelopmentClass, modifier_keys: Vec<ModifierClass>,
  #[props(default)] highlight_rank: Option<i32>,
) -> Element {
  let modifiers = GrowthTrack::class_at(&development_class, rank);
  let modifier_values = modifier_keys
    .iter()
    .map(|class| modifiers.get(class))
    .collect::<Vec<i32>>();
  let highlight = match highlight_rank {
    Some(target_rank) => target_rank == rank,
    None => false,
  };
  rsx! {
    div { class: if highlight { "uv-first thin-border left-cap selected" } else {"uv-first thin-padding"}, "{rank}" }
    for i in 0..modifier_values.len() {
      div { class: if highlight && modifier_values.len() - 1 == i { "thin-border right-cap selected" } else if highlight { "thin-border mid-cap selected" } else {"thin-padding"}, Modifier { value: modifier_values[i] } }
    }
  }
}
