use std::collections::{HashMap};

use dioxus::prelude::*;

// use crate::common::InputSignal;
use crate::skill::prelude::*;
use crate::skill::component::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillSelections {
  pub rank_signal: HashMap<String, Signal<i32>>,
  pub keystones: HashMap<String, Skill>,
  pub selectable: HashMap<String, Skill>,
  pub leeway: HashMap<String, i32>,
  pub remaining_weight: i32,
}

impl Default for SkillSelections {
  fn default() -> Self {
    Self {
      rank_signal: HashMap::new(),
      keystones: HashMap::new(),
      selectable: HashMap::new(),
      leeway: HashMap::new(),
      remaining_weight: 0,
    }
  }
}

impl SkillSelections {
  pub fn add_skill( &mut self, skill: &Skill ) {
    let id = skill.id.to_string();
    match skill.feature_weight() {
        0 => { self.keystones.insert(id, skill.clone() ); },
        1 | 2 =>{ self.selectable.insert(id, skill.clone() ); },
        _ => (),
    };
  }

  pub fn to_vec(&self) -> Vec<Skill> {
    let mut skills: Vec<Skill> = self.selectable.values().cloned().collect();
    skills.sort();
    return skills;
  }

  pub fn to_split_vec(&self) -> (Vec<Skill>, Vec<Skill>) {
    let mut skills_full: Vec<Skill> = Vec::new();
    let mut skills_half: Vec<Skill> = Vec::new();
    for skill in self.selectable.values() {
      match skill.feature_weight() {
        2 => skills_full.push( skill.clone() ),
        1 => skills_half.push( skill.clone() ),
        _ => (),
      }
    }
    skills_full.sort();
    skills_half.sort();
    return ( skills_full, skills_half )
  }
}

#[component]
pub fn CharacterSkills(
  skill_selection: SkillSelections,
) -> Element {
  let ( skills_feature, skills_minor_feature ) = skill_selection.to_split_vec();
  let skill_selectable: Vec<(Skill, Signal<i32>)> = skills_feature.iter()
  .map( |skill| {
    let id = skill.id.to_string();
    let rank_signal = *skill_selection.rank_signal.entry(id).or_insert(use_signal( || 0 ));
    ( skill.clone(), rank_signal )
  } )
  .collect();
  rsx! {
    div {
      "Pick some skills that look cool to you"
    }
    div {
      class: "block-columns",
      for (skill, rank_signal) in skill_selectable {
        SkillSelector {
          skill,
          rank_signal,
          skill_selection: skill_selection.clone(),
        }
      }
    }
    div {
      class: "block-columns",
      for skill in skills_minor_feature {
        SkillCard {
          skill,
          display: SkillTermDisplay::Embeded
        }
      }
    }
  }
}

#[component]
pub fn SkillSelector(
  skill: Skill,
  rank_signal: Signal<i32>,
  mut skill_selection: SkillSelections,
) -> Element {
  let id = skill.id.to_string();
  // let ranked = skill.is_ranked();
  
  let selected = rank_signal() > 0;
  let remaining_weight = skill_selection.remaining_weight;
  let leeway = skill_selection.leeway.get( &id ).unwrap_or( &remaining_weight );
  let leeway_weight = leeway / skill.feature_weight();
  let min_rank = 0;
  let max_rank = rank_signal() + leeway_weight;
  // let input_optional = match ranked {
  //   true => Some( rsx!{
  //     InputSignal {
  //       rank: rank_signal.clone(), max_rank
  //     }
  //   } ),
  //   false => None,
  // };
  let conditional_class = match selected {
    true => "card-selected",
    false => "",
  };
  rsx! {
    div { "{id:?}" }
    div { "{leeway:?} ({leeway_weight:?}) / {remaining_weight:?} : {max_rank:?}" }
    input {
      class: "input", type: "number",
      value: rank_signal(), min: min_rank, max: max_rank,
      oninput: move |event| {
        let value = event.value().parse::<i32>()
        .unwrap_or_default()
        .min(max_rank).max(min_rank);
        rank_signal.set(value);
      },
      onclick: move |event| {
        event.stop_propagation();
      }
    }
    div {
      class: "max-content {conditional_class}",
      // id: "{id}",
      onclick: move |_| {
        // if ranked { return; }
        match selected {
          true => { rank_signal.set(0); },
          false => { rank_signal.set(1); },
        };
      },
      SkillCard {
        skill,
        display: SkillTermDisplay::Embeded,
        // input: input_optional,
      }
    }
  }
}
