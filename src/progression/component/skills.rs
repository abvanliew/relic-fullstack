use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

use crate::common::InputSignal;
use crate::skill::prelude::*;
use crate::skill::component::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillSelections {
  pub skill_toggle_signal: Signal<HashSet<String>>,
  pub skill_rank_signal: HashMap<String, Signal<i32>>,

  pub skills_keystone: HashMap<String, Skill>,
  pub skills_feature: HashMap<String, Skill>,
  pub skills_minor_feature: HashMap<String, Skill>,
}

impl Default for SkillSelections {
  fn default() -> Self {
    Self {
      skill_toggle_signal: use_signal(|| HashSet::new()),
      skill_rank_signal: HashMap::new(),
      skills_keystone: HashMap::new(),
      skills_feature: HashMap::new(),
      skills_minor_feature: HashMap::new(),
    }
  }
}

impl SkillSelections {
  pub fn add_skill( &mut self, skill: &Skill ) {
    let id = skill.id.to_string();
    match skill.training_cost {
        TrainingCost::Inherient | TrainingCost::Keystone => {self.skills_keystone.insert(id, skill.clone() ); },
        TrainingCost::Full | TrainingCost::Spell => {self.skills_feature.insert(id, skill.clone() ); },
        TrainingCost::Half | TrainingCost::Cantrip =>{ self.skills_minor_feature.insert(id, skill.clone() ); },
    };
  }

  pub fn to_vec(&self) -> (Vec<Skill>, Vec<Skill>) {
    let skills_feature: Vec<Skill> = self.skills_feature.values().cloned().collect();
    let skills_minor_feature: Vec<Skill> = self.skills_minor_feature.values().cloned().collect();
    return (skills_feature, skills_minor_feature)
  }
}

#[component]
pub fn CharacterSkills(
  skill_selection: SkillSelections,
) -> Element {
  let ( skills_feature, skills_minor_feature ) = skill_selection.to_vec();
  rsx! {
    div {
      "Pick some skills that look cool to you"
    }
    div {
      class: "block-columns",
      for skill in skills_feature {
        SkillSelector {
          skill,
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
  skill_selection: SkillSelections,
) -> Element {
  let id = skill.id.to_string();
  let ranked = skill.is_ranked();
  let (selected, signal_result) = match ranked {
    true => {
      let signal_result = skill_selection.skill_rank_signal.entry(id.clone()).or_insert(use_signal(|| 0));  
      let element = Some(
        rsx!{
          InputSignal {
            rank: *signal_result, max_rank: 10
          }
        }
      );
      (signal_result() > 0, element)
    },
    false => {(
      (skill_selection.skill_toggle_signal)().contains(&id),
      None
    )}
  };

  let conditional_class = match selected {
    true => "card-selected",
    false => "",
  };

  // let element: Option<Element> = match ranked {
  //   true => ,
  //   false => None,
  // };

  rsx! {
    div {
      class: "max-content {conditional_class}",
      onclick: move |_| {
        if ranked { return; }
        let mut new_selections = (skill_selection.skill_toggle_signal)().clone();
        match selected {
          true => {
            new_selections.remove(&id);
          },
          false => {
            new_selections.insert(id.clone());
          }
        }
        skill_selection.skill_toggle_signal.set(new_selections);
      },
      SkillCard {
        skill,
        display: SkillTermDisplay::Embeded,
        input: signal_result,
      }
    }
  }
}
