use dioxus::prelude::*;

use crate::builder::level_selections::CharacterBuild;
use crate::progression::prelude::LevelTrack;
use crate::modifiers::ModifierClass;

#[component]
pub fn CharacterBuildPanel() -> Element {
  let build_signal = use_signal(|| CharacterBuild::default());
  let track = LevelTrack::compile_level_modifiers(12);
  let mut level = 1;
  let mut selection_rows: Vec<Element> = Vec::new();
  for (by_level, running_sum) in track {
    let mut index: usize = 0;
    let init_path_required = by_level.get(&ModifierClass::InitiatePathRequired);
    for _ in 0..init_path_required {
      selection_rows.push(rsx! {
        PathFeatureSelectionSet { level: level, title: "Initiate Path", index, state: SelectionState::Path }
      });
      index += 1;
    }
    let optional_paths = by_level.get(&ModifierClass::InitiatePathOptional);
    for _ in 0..optional_paths {
      selection_rows.push(rsx! {
        PathFeatureSelectionSet { level: level, title: "Join a new Path or Gain extra features", index, state: SelectionState::PathOrFeatures }
      });
      index += 1;
    }
    let full_features = by_level.get(&ModifierClass::Feature);
    let minor_features = by_level.get(&ModifierClass::MinorFeature);
    let half_feature_points = 2 * full_features + minor_features;
    if half_feature_points > 0 {
      selection_rows.push(rsx! {
        PathFeatureSelectionSet { level: level, title: format!( "Gain {full_features} Features and {minor_features} Minor Features" ), index, state: SelectionState::PathOrFeatures }
      });
      index += 1;
    }
    level += 1;
  }
  rsx! {
    div {
      style: "display: grid; grid-template-columns: [title] min-content [title-end details] max-content [property] 1fr [end];",
      div { "Level" }
      div { "Choice" }
      div {}
      for row in selection_rows {
        {row}
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionState {
  Path,
  PathOrFeatures,
  Features,
}

#[component]
pub fn PathFeatureSelectionSet( level: usize, title: String, index: usize, state: SelectionState, value: Option<i32> ) -> Element {
  rsx! {
    div { "{level}" }
    div { "{title}" }
    div { "..." }
  }
}
