use std::collections::{HashMap, HashSet};

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::modifiers::{ModifierClass, ModifierSet};
use crate::path::prelude::*;
use crate::progression::prelude::*;
use crate::rules::prelude::*;
use crate::server::prelude::*;
use crate::skill::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureCounter {
  pub path_name: String,
  pub skill_filter: SkillFilter,
  pub weight: i32,
}

impl FeatureCounter {
  pub fn from_constraint(constraint: &Constraint, path_name: &String) -> Self {
    return Self {
      path_name: match &constraint.filter.skill_filter {
        SkillFilter::Features | SkillFilter::CoreFeatures => format!("{path_name}"),
        SkillFilter::MinorFeatures | SkillFilter::CoreMinorFeatures => {
          format!("{path_name} - Minors")
        },
        SkillFilter::Cantrips => format!("{path_name} - Cantrips"),
        SkillFilter::Spells => format!("{path_name} - Spells"),
      },
      skill_filter: constraint.filter.skill_filter.clone(),
      weight: constraint.required_weight,
    };
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionValidity {
  Available,
  Minimal,
  Full,
  Invalid,
}

#[derive(Debug, Clone)]
pub enum SelectionStatus {
  Unselected,
  SelectedPreviously,
  SelectedCurrently,
}

#[derive(Debug, Clone, Default)]
pub struct CharacterBuild {
  current_level_index: usize,
  level_selections: Vec<LevelSelections>,
}

impl CharacterBuild {
  fn previous_level_selections(&self) -> impl Iterator<Item = &LevelSelections> {
    let index = self.current_level_index.min(self.level_selections.len());
    return self.level_selections[..index].iter();
  }

  pub fn get_level(&self) -> i32 {
    self.current_level_index as i32 + 1
  }

  pub fn set_level(&mut self, level: i32) {
    self.current_level_index = (level - 1) as usize;
  }

  pub fn current_selection_ref(&self) -> Option<&LevelSelections> {
    return self.level_selections.get(self.current_level_index);
  }

  pub fn path_selection_status(&self, path_id: &ObjectId) -> SelectionStatus {
    if let Some(current_selection) = self.current_selection_ref() {
      match current_selection.paths.get(path_id) {
        Some(_) => {
          return SelectionStatus::SelectedCurrently;
        },
        _ => (),
      }
    };
    return match self.get_previous_paths().get(path_id) {
      Some(_) => SelectionStatus::SelectedPreviously,
      None => SelectionStatus::Unselected,
    };
  }

  pub fn set_skill_ranks(&mut self, id: &ObjectId, ranks: i32) {
    let level_selections = self.get_current_mut();
    level_selections.set_skill_ranks(id, ranks);
  }

  fn get_previous_paths(&self) -> HashSet<ObjectId> {
    let mut paths = HashSet::new();
    for selections in self.previous_level_selections() {
      paths.extend(selections.paths.clone());
    }
    return paths;
  }

  fn get_current_mut(&mut self) -> &mut LevelSelections {
    if self.level_selections.len() < self.current_level_index + 1 {
      for _ in self.level_selections.len()..=self.current_level_index {
        self.level_selections.push(LevelSelections::default());
      }
    }
    return &mut self.level_selections[self.current_level_index];
  }

  pub fn add_path(&mut self, path_id: ObjectId) {
    let current = self.get_current_mut();
    current.add_path(path_id);
  }

  pub fn remove_path(&mut self, path_id: &ObjectId) {
    let current = self.get_current_mut();
    current.remove_path(path_id);
  }

  pub fn get_current_path_ids(&self) -> HashSet<ObjectId> {
    let mut paths = self.get_previous_paths();
    let Some(current_selection) = self.current_selection_ref() else {
      return paths;
    };
    paths.extend(current_selection.paths.clone());
    return paths;
  }

  pub fn get_inheriet_path_ids(&self) -> HashSet<ObjectId> {
    let PathCache(ref path_map_cache) = use_context::<PathCache>();
    return path_map_cache
      .into_vec()
      .iter()
      .filter_map(|path| match path.inherient {
        true => Some(path.id.clone()),
        _ => None,
      })
      .collect();
  }

  pub fn get_all_path_ids(&self) -> HashSet<ObjectId> {
    let mut ids = self.get_current_path_ids();
    ids.extend(self.get_inheriet_path_ids());
    return ids;
  }

  pub fn get_feature_counts(&self) -> (Counter, Counter) {
    let (_, counter_optional) = self.get_path_counts();
    let bonus_features = 0.max(counter_optional.max - counter_optional.current);
    let level_stats = LevelTrack::as_of(self.get_level());
    let features = level_stats.get(&ModifierClass::Feature) + bonus_features;
    let minor_features = level_stats.get(&ModifierClass::MinorFeature);
    return (
      Counter::from_max(features),
      Counter::from_max(minor_features),
    );
  }

  pub fn get_path_counts(&self) -> (Counter, Counter) {
    let level_stats = LevelTrack::as_of(self.get_level());
    let PathCache(path_cache) = use_context();
    let path_ids = self.get_current_path_ids();
    let paths = path_cache.from_object_set(&path_ids);
    let mut initiate_count: i32 = 0;
    for path in paths {
      if path.tier == Tier::Initiate {
        initiate_count += 1;
      }
    }
    let initiate_required = level_stats.get(&ModifierClass::InitiatePathRequired);
    let initiate_optional = level_stats.get(&ModifierClass::InitiatePathOptional);
    let mut counter_required = Counter::from_max(initiate_required);
    let mut counter_optional = Counter::from_max(initiate_optional);
    if initiate_count <= initiate_required {
      counter_required.current += initiate_count;
    }
    if initiate_count > initiate_required {
      counter_required.current += initiate_required;
      counter_optional.current += initiate_count - initiate_required;
    }
    return (counter_required, counter_optional);
  }

  pub fn get_path_validation_status(&self) -> (SelectionValidity, Counter, Counter) {
    let (counter_required, counter_optional) = self.get_path_counts();
    return (
      match (counter_required.valid(), counter_optional.valid()) {
        (SelectionValidity::Available, _) => SelectionValidity::Available,
        (SelectionValidity::Full, SelectionValidity::Available) => SelectionValidity::Minimal,
        (SelectionValidity::Full, SelectionValidity::Full) => SelectionValidity::Full,
        _ => SelectionValidity::Invalid,
      },
      counter_required,
      counter_optional,
    );
  }

  pub fn get_path_constraints(&self) -> Vec<FeatureCounter> {
    let PathCache(ref path_map_cache) = use_context::<PathCache>();
    let path_ids = self.get_all_path_ids();
    let paths = path_map_cache.from_object_set(&path_ids);
    let mut counters: Vec<FeatureCounter> = Vec::new();
    for path in paths {
      let path_name = &path.title;
      let (path_constraints, _weight) = path.selection_constraints();
      for constraint in path_constraints {
        counters.push(FeatureCounter::from_constraint(&constraint, path_name));
      }
    }
    return counters;
  }

  pub fn get_previous_trainings(&self) -> Training {
    let mut previous_trainings = Training::default();
    for selections in self.previous_level_selections() {
      previous_trainings.extend(&selections.trainings);
    }
    return previous_trainings;
  }

  pub fn get_current_trainings(&self) -> Training {
    let mut previous_training = self.get_previous_trainings();
    let Some(level_selection) = self.current_selection_ref() else {
      return previous_training;
    };
    previous_training.extend(&level_selection.trainings);
    return previous_training;
  }

  pub fn get_training_ranks(&self) -> i32 {
    let level_stats = LevelTrack::as_of(self.get_level());
    return level_stats.get(&ModifierClass::GrowthRanks);
  }

  pub fn set_training(&mut self, class: &TrainingClass, value: i32) {
    let current = self.get_current_mut();
    current.trainings.set(class, value)
  }

  pub fn get_training_modifiers(&self) -> ModifierSet {
    let trainings = self.get_current_trainings();
    let mut modifiers = ModifierSet::default();
    for class in TrainingClass::ordered() {
      modifiers.append(&GrowthTrack::class_at(&class, trainings.get(&class)));
    }
    return modifiers;
  }

  pub fn get_inheriet_skill_ids(&self) -> HashSet<ObjectId> {
    let PathCache(ref path_map_cache) = use_context::<PathCache>();
    return path_map_cache
      .into_vec()
      .iter()
      .filter_map(|path| match path.inherient {
        true => Some(path.skill_ids.clone()),
        _ => None,
      })
      .flatten()
      .collect();
  }

  pub fn get_baseline_skills(&self) -> SkillRanks {
    let PathCache(ref path_map_cache) = use_context::<PathCache>();
    let mut path_skill_ids = vec![self.get_inheriet_skill_ids()];
    for path in path_map_cache.from_object_set(&self.get_current_path_ids()) {
      path_skill_ids.push(path.skill_ids);
    }
    return SkillRanks::populate_from_path_skill_ids(path_skill_ids);
  }

  pub fn get_current_skills(&self) -> SkillRanks {
    let Some(level_selection) = self.current_selection_ref() else {
      return SkillRanks::default();
    };
    return level_selection.skill_ranks.clone();
  }

  pub fn get_skill_selection_range_base(&self) -> SkillRanges {
    let mut skill_ranges = SkillRanges::default();
    for selections in self.previous_level_selections() {
      skill_ranges.at_least_minimums(&selections.skill_ranks);
    }
    skill_ranges.at_least(self.get_current_skills());
    skill_ranges.append(self.get_baseline_skills());
    return skill_ranges;
  }

  pub fn get_weights_constraints(&self) -> (Vec<Constraint>, HashMap<u64, ConstraintSet>, i32) {
    let PathCache(ref path_map_cache) = use_context();
    let mut constraints: Vec<Constraint> = Vec::new();
    let mut weight_budget = 0;
    let path_ids = self.get_all_path_ids();
    let (features, minor_features) = self.get_feature_counts();
    if features.max > 0 {
      let feature_weight = 2 * features.max;
      constraints.push(Constraint::feature(feature_weight));
      weight_budget += feature_weight;
    }
    
    if minor_features.max > 0 {
      constraints.push(Constraint::minor_feature(minor_features.max));
      weight_budget += minor_features.max;
    }
    
    for path in path_map_cache.from_object_set(&path_ids) {
      let (mut path_constraints, additional_budget) = path.selection_constraints();
      constraints.append(&mut path_constraints);
      weight_budget += additional_budget;
    }
    
    let mut constraint_sets: HashMap<u64, ConstraintSet> = HashMap::new();
    for (index, constraint) in constraints.iter().enumerate() {
      let mask = 1 << index;
      constraint_sets.insert(
        mask,
        ConstraintSet {
          required_weight: constraint.required_weight,
          filters: vec![constraint.filter.clone()],
          ..Default::default()
        },
      );
    }
    
    return (constraints, constraint_sets, weight_budget);
  }

  pub fn get_skill_ranges(&self) -> (SkillRanges, Vec<Counter>) {
    let PathCache(ref path_map_cache) = use_context();
    let SkillCache(ref skill_map_cache) = use_context();
    let mut selected_weights = 0;

    let mut skill_ranges: SkillRanges = self.get_skill_selection_range_base();
    let (mut constraints, mut constraint_sets, weight_budget) = self.get_weights_constraints();

    for (skill_id, range) in skill_ranges.ranges.iter_mut() {
      let Some(skill) = skill_map_cache.from_object_id(&skill_id) else {
        continue;
      };
      let mut mask: u64 = 0;
      let mut required_weight = 0;
      let mut filters = Vec::<SelectionFilter>::new();
      for (index, constraint) in constraints.iter().enumerate() {
        if !skill.is_match(&constraint.filter) {
          continue;
        }
        mask += 1 << index;
        required_weight += constraint.required_weight;
        filters.push(constraint.filter.clone());
      }
      range.update(mask, &skill);
      selected_weights += range.current_weight();
      let constraint_set = constraint_sets.entry(mask).or_insert(
        ConstraintSet { required_weight, filters, ..Default::default() }
      );
      constraint_set.selected_weight += range.current_weight();
    }

    for (index, constraint) in constraints.iter_mut().enumerate() {
      let constraint_mask: u64 = 1 << index;
      let mut constrained_weight: i32 = 0;
      for (set_mask, constraint_set) in constraint_sets.iter_mut() {
        if constraint_mask & set_mask == 0 {
          continue;
        }
        let constraint_net = (constraint_set.selected_weight - constraint_set.required_weight
          + constraint.required_weight)
          .max(0);
        constrained_weight += constraint_net;
        constraint_set.overage_total += constraint_net;
      }
      constraint.overages = constrained_weight;
    }

    let remaining_weight = weight_budget - selected_weights;

    for (set_mask, constraint_set) in constraint_sets.iter_mut() {
      let mut total_constraint_overages = 0;
      for (index, constraint) in constraints.iter().enumerate() {
        let constraint_mask: u64 = 1 << index;
        if constraint_mask & set_mask == 0 {
          continue;
        }
        total_constraint_overages += constraint.overages;
      }
      constraint_set.leeway = (constraint_set.required_weight
        - constraint_set.selected_weight
        - total_constraint_overages
        + constraint_set.overage_total)
        .min(remaining_weight);
    }

    for (_skill_id, range) in skill_ranges.ranges.iter_mut() {
      let Some(constraint_set) = constraint_sets.get(&range.mask) else {
        continue;
      };
      range.adjust_max(constraint_set.leeway);
    }

    let mut ordered_constraint_counters: Vec<(SelectionFilter,Counter)> = Vec::new();

    for (index, constraint) in constraints.iter().enumerate() {
      let mask = 1 << index;
      let Some(constraint_set) = constraint_sets.get(&mask) else {
        continue;
      };
      let feature_type = constraint.filter.skill_filter.to_string();
      let title = match &constraint.filter.path_filter {
        PathFilter::All => Some(feature_type),
        PathFilter::Single(path_id) => Some(match path_map_cache.from_id(&path_id) {
          Some(path) => format!("{} {feature_type}", path.title),
          None => "undefined".into(),
        }),
      };
      let increment = constraint.filter.skill_filter.weight();
      let max = constraint.required_weight;
      let min = max - constraint_set.leeway;
      ordered_constraint_counters.push((constraint.filter.clone(), Counter {
        title,
        increment,
        current: min,
        max,
        ..Default::default()
      }));
    }

    ordered_constraint_counters.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

    let constraint_counters = ordered_constraint_counters.into_iter().map(|(_, counter)| counter).collect();

    tracing::info!("{constraints:#?}");
    tracing::info!("{constraint_sets:#?}");
    tracing::info!("{skill_ranges:#?}");

    return (skill_ranges, constraint_counters);
  }
}

#[derive(Debug, Clone, Default)]
pub struct LevelSelections {
  paths: HashSet<ObjectId>,
  trainings: Training,
  skill_ranks: SkillRanks,
}

impl LevelSelections {
  pub fn add_path(&mut self, path_id: ObjectId) {
    self.paths.insert(path_id);
  }

  pub fn remove_path(&mut self, path_id: &ObjectId) {
    self.paths.remove(path_id);
  }

  pub fn set_skill_ranks(&mut self, id: &ObjectId, ranks: i32) {
    self.skill_ranks.set_skill_ranks(id, ranks);
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
  pub title: Option<String>,
  pub increment: i32,
  pub current: i32,
  pub max: i32,
}

impl Default for Counter {
  fn default() -> Self {
    Self {
      title: None,
      current: 0,
      max: 0,
      increment: 1,
    }
  }
}

impl Counter {
  pub fn from_max(max: i32) -> Self {
    Self {
      max,
      ..Default::default()
    }
  }

  pub fn effective(&self) -> (i32, i32, i32) {
    if self.increment <=1 {
      return (self.current, 0, self.max);
    }
    return (self.current / self.increment, self.current % self.increment, self.max / self.increment);
  }

  pub fn valid(&self) -> SelectionValidity {
    if self.current < self.max {
      return SelectionValidity::Available;
    }
    if self.current == self.max {
      return SelectionValidity::Full;
    }
    return SelectionValidity::Invalid;
  }
}

#[derive(Debug, Clone, Default)]
pub struct Training {
  pub adept: Option<i32>,
  pub endurance: Option<i32>,
  pub expert: Option<i32>,
  pub innate: Option<i32>,
  pub resonant: Option<i32>,
  pub magic: Option<i32>,
}

impl Training {
  pub fn extend(&mut self, other: &Self) {
    self.adept = optional_max(&self.adept, &other.adept);
    self.endurance = optional_max(&self.endurance, &other.endurance);
    self.expert = optional_max(&self.expert, &other.expert);
    self.innate = optional_max(&self.innate, &other.innate);
    self.resonant = optional_max(&self.resonant, &other.resonant);
    self.magic = optional_max(&self.magic, &other.magic);
  }

  pub fn get(&self, class: &TrainingClass) -> i32 {
    return match class {
      TrainingClass::Adept => self.adept.unwrap_or(0),
      TrainingClass::Endurance => self.endurance.unwrap_or(0),
      TrainingClass::Expert => self.expert.unwrap_or(0),
      TrainingClass::Innate => self.innate.unwrap_or(0),
      TrainingClass::Resonance => self.resonant.unwrap_or(0),
      TrainingClass::Magic => self.magic.unwrap_or(0),
    };
  }

  pub fn set(&mut self, class: &TrainingClass, value: i32) {
    match class {
      TrainingClass::Expert => self.expert = Some(value),
      TrainingClass::Adept => self.adept = Some(value),
      TrainingClass::Endurance => self.endurance = Some(value),
      TrainingClass::Innate => self.innate = Some(value),
      TrainingClass::Resonance => self.resonant = Some(value),
      TrainingClass::Magic => self.magic = Some(value),
    }
  }

  pub fn sum(&self) -> i32 {
    return self.adept.unwrap_or(0)
      + self.endurance.unwrap_or(0)
      + self.expert.unwrap_or(0)
      + self.innate.unwrap_or(0)
      + self.resonant.unwrap_or(0)
      + self.magic.unwrap_or(0);
  }

  pub fn summary(&self) -> String {
    let items: Vec<String> = vec![
      option_formater("Adept".into(), &self.adept),
      option_formater("Endurance".into(), &self.endurance),
      option_formater("Expert".into(), &self.expert),
      option_formater("Innate".into(), &self.innate),
      option_formater("Resonnance".into(), &self.resonant),
      option_formater("Magic".into(), &self.magic),
    ]
    .into_iter()
    .flatten()
    .collect();
    return items.join(", ");
  }
}

fn optional_max(lhs: &Option<i32>, rhs: &Option<i32>) -> Option<i32> {
  return match (lhs, rhs) {
    (None, None) => None,
    (Some(value), None) => Some(*value),
    (None, Some(value)) => Some(*value),
    (Some(left_value), Some(right_value)) => Some(*left_value.max(right_value)),
  };
}

fn option_formater(title: String, value: &Option<i32>) -> Option<String> {
  return match value {
    Some(value) => {
      if value.eq(&0) {
        None
      } else {
        Some(format!("{title} {value}"))
      }
    },
    None => None,
  };
}

#[derive(Debug, Clone, Default)]
pub struct ConstrainedRange {
  pub mask: u64,
  pub min: i32,
  pub current: i32,
  pub max: i32,
  pub weight: i32,
  pub ranked: bool,
  pub fixed: bool,
}

impl ConstrainedRange {
  pub fn current_weight(&self) -> i32 {
    return self.current * self.weight;
  }

  pub fn update(&mut self, mask: u64, skill: &Skill) {
    self.mask = mask;
    self.weight = skill.weight();
    self.ranked = skill.is_ranked();
    self.fixed = match &skill.training_cost {
      TrainingCost::Inherient | TrainingCost::Keystone => {
        self.min = self.current;
        self.max = self.current;
        true
      },
      _ => false,
    }
  }

  pub fn adjust_max(&mut self, leeway: i32) {
    if self.fixed {
      return;
    }
    let mut net_ranks = self.current + if self.weight == 0 { 0 } else { leeway / self.weight };
    if !self.ranked && net_ranks > 1 {
      net_ranks = 1;
    }
    self.max = net_ranks;
  }
}

#[derive(Debug, Clone, Default)]
pub struct SkillRanges {
  pub ranges: HashMap<ObjectId, ConstrainedRange>,
}

impl SkillRanges {
  pub fn at_least_minimums(&mut self, skill_ranks: &SkillRanks) {
    for (skill_id, ranks) in &skill_ranks.ranks {
      let range = self.ranges.entry(skill_id.clone()).or_default();
      range.min = range.min.max(*ranks);
      range.current = range.current.max(*ranks);
    }
  }

  pub fn at_least(&mut self, skill_ranks: SkillRanks) {
    for (skill_id, ranks) in skill_ranks.ranks {
      let range = self.ranges.entry(skill_id).or_default();
      range.current = range.current.max(ranks);
    }
  }

  pub fn append(&mut self, skill_ranks: SkillRanks) {
    for (skill_id, ranks) in skill_ranks.ranks {
      let range = self.ranges.entry(skill_id).or_default();
      range.current += ranks;
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct SkillRanks {
  pub ranks: HashMap<ObjectId, i32>,
}

impl SkillRanks {
  pub fn populate_from_path_skill_ids(path_skill_ids: Vec<HashSet<ObjectId>>) -> Self {
    let mut skill_ranks = SkillRanks::default();
    let SkillCache(ref skill_map) = use_context();
    for skill_ids in path_skill_ids {
      for skill in skill_map.from_object_set(&skill_ids) {
        match skill.training_cost {
          TrainingCost::Inherient => (),
          TrainingCost::Keystone => {
            skill_ranks.insert(skill.id.clone(), 1);
          },
          _ => {
            skill_ranks.insert(skill.id.clone(), 0);
          },
        }
      }
    }
    return skill_ranks;
  }

  pub fn insert(&mut self, skill_id: ObjectId, value: i32) {
    let entry = self.ranks.entry(skill_id).or_default();
    *entry += value;
  }

  pub fn set_skill_ranks(&mut self, id: &ObjectId, ranks: i32) {
    if ranks <= 0 {
      self.ranks.remove(id);
      return;
    }
    let skill_rank = self.ranks.entry(*id).or_default();
    *skill_rank = ranks;
  }
}
