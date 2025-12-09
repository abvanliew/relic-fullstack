use std::collections::{HashMap, HashSet};

use bson::oid::ObjectId;
use dioxus::prelude::*;

use super::level::LevelSelector;
use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::path::Path;
use crate::progression::fixed::MIN_LEVEL;
use crate::progression::track::TrackContext;
use crate::progression::training::TrainingClass;
use crate::server::prelude::*;
use crate::skill::prelude::*;

#[derive(Debug, Clone)]
pub enum BuilderTab {
  Paths,
  Growth,
}

pub fn sorted_paths() -> Vec<(String, Path)> {
  let library = use_context::<ServerRequestSignals>();
  let res_paths = library.get_paths();
  let path_map: HashMap<String, Path>;
  match res_paths {
    Some(result) => path_map = result,
    _ => { return Vec::new(); }
  };
  let mut path_sorted: Vec<(String, Path)> = path_map.clone()
    .into_iter()
    .filter(|(_, path)| path.inherient != Some(true))
    .collect();
  path_sorted.sort_by(|(_, lhs_path), (_, rhs_path)| lhs_path.cmp(rhs_path));
  return path_sorted
}

#[derive(Clone)]
struct PathDisplay(Signal<Option<String>>);

#[component]
pub fn CharacterProgression(paths: Vec<Path>) -> Element {
  let mut current_tab: Signal<BuilderTab> = use_signal(|| BuilderTab::Paths);
  let signal_path_display = PathDisplay(use_signal(|| None));
  use_context_provider(|| signal_path_display );
  let path_sorted = sorted_paths();
  let track = TrackContext::use_context_provider();
  let build_context = BuildContext::use_context_provider();
  let paths = build_context.paths;
  let path_selection = paths.selection;
  let path_count = paths.len();
  let skills = build_context.skills;
  let skill_selection = skills.selection;
  let level = build_context.level;
  let stats = track.character.stats(level());
  let qualifiers = BuildContext::get_qualifiers(
    path_selection.into(), 
    skill_selection.into()
  );
  let extra_path_features = 
    if path_count.max(stats.iniatite.path_min) >= stats.iniatite.path_max { 
      0
    } 
    else { 
      stats.iniatite.path_max - path_count.max(stats.iniatite.path_min)
    } 
  ;
  rsx! {
    div { "{stats:?}" }
    div { "{extra_path_features:?}" }
    // div { "{qualifiers:?}" }
    div {
      class: "row",
      LevelSelector {}
      div {
        onclick: move |_| {
          current_tab.set(BuilderTab::Paths);
        },
        "Paths"
      }
      div {
        onclick: move |_| {
          current_tab.set(BuilderTab::Growth);
        },
        "Growth"
      }
    }
    match current_tab() {
      BuilderTab::Paths =>rsx!(
        div { "You must select at least one path, then you can choose up to X additional paths or gain additional features in your existing paths" }
        div { "Path Count: {path_count}" }
        div {
          class: "path-grid",
          RankedSelector { max: extra_path_features }
          for ( id, path ) in path_sorted {
            PathSelector { id, path }
          }
        }
      ),
      BuilderTab::Growth =>rsx!(
        GrowthSelector { training: TrainingClass::Adept, level }
        GrowthSelector { training: TrainingClass::Endurance, level }
        GrowthSelector { training: TrainingClass::Expert, level }
        GrowthSelector { training: TrainingClass::Innate, level }
        GrowthSelector { training: TrainingClass::Resonance, level }
        GrowthSelector { training: TrainingClass::Magic, level }
      ),
    }
  }
}

#[component]
pub fn RankedSelector( max: usize ) -> Element {
  let mut number = use_signal(|| 0);
  let max = max;
  rsx! {
    div {
      class: "path-card row",
      input {
        class: "input",
        type: "number",
        value: number(),
        oninput: move |event| {
          let value = event.value().parse::<usize>().unwrap_or(0).min(max);
          number.set(value);
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
      div { class: "italics", "Extra Features" }
    }
  }
}

#[derive(PartialEq, Props, Clone)]
struct GrowthSelectorProps {
  training: TrainingClass,
  level: ReadOnlySignal<usize>,
}

#[component]
pub fn GrowthSelector(props: GrowthSelectorProps) -> Element {
  let track = use_context::<TrackContext>();
  let training = props.training;
  let mut number = use_signal(|| 0);
  let level = props.level;
  let bonuses = track.training.total_bonus(&training, number());
  rsx! {
    div {
      class: "path-card row",
      input {
        type: "number",
        class: "input",
        value: number(),
        oninput: move |event| {
          let value = event.value().parse::<usize>().unwrap_or(0);
          let constrained = if value > level() { level() } else { value };
          number.set(constrained);
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
      div {"{training:?}"}
      div {"{bonuses}"}
    }
  }
}


pub fn split_sort_skill_ids(
  result_skill_ids: Option<Vec<ObjectId>>
) -> Option<(Vec<String>, Vec<String>, Vec<String>)> {
  let library = use_context::<ServerRequestSignals>();
  let res_skills = library.get_skills();
  let skill_map: HashMap<String, Skill>;
  match res_skills {
    Some(result) => skill_map = result,
    _ => { return None; }
  };
  let skill_ids: Vec<String> = result_skill_ids
    .unwrap_or(Vec::new())
    .iter()
    .map(|x| x.to_string())
    .collect();
  // let mut sorted_skills: Vec<Skill> = skill_ids.iter()
  //   .filter_map(|ids| match skill_map.get(ids) {
  //     Some( skill) => Some( skill.clone() ),
  //     None => None,
  //   } )
  //   .collect();
  let mut keystone_ids: Vec<String> = Vec::new();
  let mut feature_ids: Vec<String> = Vec::new();
  let mut minor_features_ids: Vec<String> = Vec::new();
  for id in skill_ids {
    let Some(skill) = skill_map.get(&id) else {
      continue;
    };
    match &skill.training_cost {
      TrainingCost::Inherient | TrainingCost::Keystone => keystone_ids.push(id.clone()),
      TrainingCost::Full | TrainingCost::Spell => feature_ids.push(id.clone()),
      TrainingCost::Half | TrainingCost::Cantrip => minor_features_ids.push(id.clone()),
    }
  }
  return Some(
    (keystone_ids, feature_ids, minor_features_ids)
  )
}



#[component]
pub fn PathSelector(
  id: String,
  path: ReadOnlySignal<Path>,
) -> Element {
  let mut build = use_context::<BuildContext>();
  let path = path();
  let title = path.title;
  let Some( ( keystone_ids, feature_ids, minor_features_ids ) ) = split_sort_skill_ids(path.skill_ids.clone()) else {
    return rsx ! {
      div { "loading ..." }
    }
  };
  let behavoir = build.path_behavoir(&id);
  let (class, img_src) = match behavoir {
    SelectionState::Deselected => ("", IMG_UNSELECTED),
    SelectionState::Selected => ("", IMG_SELECTED),
    SelectionState::SelectedFixed => ("", IMG_SELECTED),
    SelectionState::Disabled => ("hidden", IMG_UNSELECTED),
    SelectionState::Invalid => ("disabled", IMG_SELECTED),
    SelectionState::Visibile => ("", IMG_UNSELECTED),
  };
  let PathDisplay( mut path_display ) = use_context::<PathDisplay>();
  let result_path_display = path_display();
  let display = match &result_path_display {
    Some( display_id ) => *display_id == id,
    None => false, 
  };
  let current_id = id.clone();
  return rsx! {
    div {
      class: "row path-card {class}",
      onclick: move |_| {
        path_display.set(
          match &result_path_display {
            None => Some( current_id.to_owned() ),
            Some( path_id ) => {
              if current_id.eq( path_id ) {
                None
              } else {
                Some( current_id.to_owned() )
              }
            }
          }
        )
      },
      div {
        class: "path-checkbox-wrapper",
        onclick: move |evt: Event<MouseData>| {
          evt.stop_propagation();
          build.path_toggle(&id);
        },
        img { src: "{img_src}" }
      }
      div { class: "path-title", "{title}" }
    }
    if display {
      div {
        class: "path-skill-panels {class}",
        if keystone_ids.len() > 0 {
          div { class: "small-text dotted-underline spacer-medium", "Keystones" }
          for skill_id in keystone_ids {
            div {
              class: "path-skill-group",
              SkillSelector { id: skill_id, path_id: id.clone() }
            }
          }
        }
        if feature_ids.len() > 0 {
          div { class: "small-text dotted-underline spacer-medium", "Features" }
          for skill_id in feature_ids {
            div {
              class: "path-skill-group",
              SkillSelector { id: skill_id, path_id: id.clone() }
            }
          }
        }
        if minor_features_ids.len() > 0 {
          div { class: "small-text dotted-underline spacer-medium", "Minor Features" }
          for skill_id in minor_features_ids {
            div {
              class: "path-skill-group",
              SkillSelector { id: skill_id, path_id: id.clone() }
            }
          }
        }
      }
    }
  };
}

#[derive(PartialEq, Props, Clone)]
struct SkillSelectorProps {
  id: String,
  path_id: String,
}

#[component]
pub fn SkillSelector(props: SkillSelectorProps) -> Element {
  let mut build = use_context::<BuildContext>();
  let id = props.id;
  let behavoir = build.skill_behavoir(&id, &props.path_id);
  let (class, enabled) = match behavoir {
    SelectionState::Deselected => ("", true),
    SelectionState::Selected => ("card-selected", true),
    SelectionState::SelectedFixed => ("card-selected", false),
    SelectionState::Disabled => ("hidden", false),
    SelectionState::Invalid => ("disabled", true),
    SelectionState::Visibile => ("", false),
  };
  rsx! {
    div {
      class,
      onclick: move |event: Event<MouseData>| {
        if !enabled {return;}
        event.stop_propagation();
        build.skill_toggle(&id);
      },
      SkillCard { id: id.clone(), display: SkillTermDisplay::Embeded }
    }
  }
}

#[derive(PartialEq, Props, Clone)]
struct HashButtonProps {
  name: String,
  id: String,
  class: Option<String>,
}

#[derive(PartialEq)]
pub enum SelectionState {
  Visibile,
  Deselected,
  Selected,
  SelectedFixed,
  Disabled,
  Invalid,
}

#[component]
pub fn HashButton(props: HashButtonProps) -> Element {
  let mut build = use_context::<BuildContext>();
  let behavoir = build.path_behavoir(&props.name);
  let class: String = match behavoir {
    SelectionState::Deselected => "tile unselected",
    SelectionState::Selected => "tile selected",
    SelectionState::SelectedFixed => "tile selected",
    SelectionState::Disabled => "tile unselected hidden",
    SelectionState::Invalid => "tile selected disabled",
    SelectionState::Visibile => "tile unselected",
  }
  .into();
  rsx! {
    div {
      class: class,
      onclick: move |_| {
        build.path_toggle( &props.name );
      },
      "{props.name}"
    }
  }
}

#[derive(Debug, Clone)]
pub struct HashSelectorContext {
  pub selection: Signal<HashSet<String>>,
}

impl HashSelectorContext {
  pub fn use_context_provider() -> Self {
    let selection: Signal<HashSet<String>> = use_signal(|| HashSet::new());
    use_context_provider(|| Self { selection })
  }

  pub fn len(&self) -> usize {
    return self.selection.read().len();
  }

  pub fn contains(&self, id: &String) -> bool {
    let selection = self.selection.read();
    return selection.contains(id);
  }

  pub fn selection_behavoir(&self, id: &String, max_len: usize) -> SelectionState {
    match self.contains(id) {
      true => SelectionState::Selected,
      false => SelectionState::Deselected,
    }
    // match
    // (
    //   self.contains(id),
    //   selection.len() < max_len,
    //   selection.len() > max_len,
    // )
    // {
    //   (true, _, false) => SelectionState::SelectedToggle,
    //   (false, true, _) => SelectionState::Selectable,
    //   (false, false, _) => SelectionState::Disabled,
    //   (true, _, true) => SelectionState::Invalid,
    // }
  }

  pub fn toggle(&mut self, name: &String, max_len: usize) {
    let mut current = self.selection.read().clone();
    match self.selection_behavoir(name, max_len) {
      SelectionState::Selected | SelectionState::Invalid => current.remove(name),
      SelectionState::Deselected => current.insert(name.clone()),
      _ => false,
    };
    self.selection.set(current);
  }
}

#[derive(Debug, Clone, Default)]
pub struct CharacterQualifiers {
  pub path_min: u32,
  pub path_max: u32,
  pub minor_features: u32,
  pub path_qualifiers: HashMap<String, bool>,
}

// #[derive(Debug, Clone, Default)]
// pub struct PathQualifiers {
//   pub min_feature: u32,
//   pub min_minor_feature: u32,
// }

#[derive(Debug, Clone)]
pub struct BuildContext {
  pub level: Signal<usize>,
  pub paths: HashSelectorContext,
  pub feature_path: Signal<usize>,
  pub skills: HashSelectorContext,
  pub qualifiers: Signal<CharacterQualifiers>,
  pub training: Signal<TrainingSet>,
}

impl BuildContext {
  pub fn use_context_provider() -> Self {
    let level = use_signal(|| MIN_LEVEL);
    let paths = HashSelectorContext::use_context_provider();
    let feature_path = use_signal(|| 0);
    let skills = HashSelectorContext::use_context_provider();
    let qualifiers = use_signal(|| CharacterQualifiers::default());
    let training: Signal<TrainingSet> = use_signal(|| TrainingSet::default());
    use_context_provider(|| Self {
      level,
      paths,
      feature_path,
      skills,
      qualifiers,
      training,
    })
  }

  pub fn get_qualifiers(
    selected_paths: ReadOnlySignal<HashSet<String>>,
    selected_skills: ReadOnlySignal<HashSet<String>>,
  ) -> CharacterQualifiers {
    let library = GameLibrarySignal::use_context_provider();
    let paths = library.paths.read();
    let skills = library.skills.read();
    let mut qualifiers = CharacterQualifiers {
      path_min: 1,
      path_max: 2,
      minor_features: 5,
      path_qualifiers: HashMap::new(),
    };
    if let Some(path_data) = &*paths {
      if let Some(skill_data) = &*skills {
        let mut skill_paths: HashMap<String, HashSet<String>> = HashMap::new();
        for path_id in selected_paths().iter() {
          let Some(path) = path_data.get(path_id)
          else { continue; };
          let Some(skill_ids) = &path.skill_ids
          else { continue; };
          let skill_set: HashSet<String> = skill_ids.iter().map(|id| id.to_string()).collect();
          let x = selected_skills();
          let intersection_ids = x.intersection(&skill_set);
          for skill_id in intersection_ids {
            let path_set = skill_paths.entry(skill_id.clone()).or_default();
            path_set.insert(path_id.clone());
          }
        }
        let mut mono_skill_count_by_path: HashMap<String, u32> = HashMap::new();
        for (skill_id, path_set) in skill_paths {
          if path_set.len() != 1 {
            continue;
          }
          let Some(skill) = skill_data.get(&skill_id) 
          else { continue; };
          for path_id in path_set {
            let count = mono_skill_count_by_path.entry(path_id).or_default();
            *count += skill.minor_feature_cost();
          }
        }
        let mut minor_features_remaining: u32 = qualifiers.minor_features;
        for skill_id in selected_skills() {
          let Some(skill) = skill_data.get(&skill_id)
          else { continue; };
          minor_features_remaining -= skill.minor_feature_cost();
        }
        for path_id in selected_paths() {
          let Some(mono_skill_count) = mono_skill_count_by_path.get(&path_id) 
          else { continue; };
          qualifiers.path_qualifiers.insert(
            path_id.clone(),
            minor_features_remaining > *mono_skill_count,
          );
        }
      }
    }
    return qualifiers;
  }

  pub fn path_max(&self) -> usize {
    let track = use_context::<TrackContext>();
    let level = self.level;
    let stats = track.character.stats(level());
    return stats.iniatite.path_max;
  }

  pub fn path_behavoir(&self, name: &String) -> SelectionState {
    return self.paths.selection_behavoir(name, self.path_max());
  }

  pub fn path_toggle(&mut self, name: &String) {
    return self.paths.toggle(name, self.path_max());
  }

  pub fn skill_max(&self) -> usize {
    let track = use_context::<TrackContext>();
    let level = self.level;
    let stats = track.character.stats(level());
    return stats.iniatite.features;
  }

  pub fn skill_behavoir(&self, name: &String, path_id: &String) -> SelectionState {
    if self.paths.selection_behavoir(path_id, self.path_max()) == SelectionState::Selected {
      return self.skills.selection_behavoir(name, self.skill_max());
    }
    return SelectionState::Visibile;
  }

  pub fn skill_toggle(&mut self, name: &String) {
    return self.skills.toggle(name, self.skill_max());
  }
}

#[derive(Debug, Clone, Default)]
pub struct TrainingSet {
  pub adept: i32,
  pub endurance: i32,
  pub expert: i32,
  pub innate: i32,
  pub resonance: i32,
  pub magic: i32,
}
