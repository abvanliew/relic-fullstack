use std::collections::HashMap;

use dioxus::prelude::*;

use crate::skill::prelude::*;
use crate::skill::component::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillSelections {
  pub rank_signal: Signal<HashMap<String, i32>>,
  pub keystones: HashMap<String, Skill>,
  pub selectable: HashMap<String, Skill>,
  pub leeway: HashMap<String, i32>,
  pub remaining_weight: i32,
}

impl Default for SkillSelections {
  fn default() -> Self {
    Self {
      rank_signal: use_signal( || HashMap::new() ),
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
    match skill.weight() {
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
}

#[component]
pub fn CharacterSkills(
  skill_selection: SkillSelections,
  core_constraints: Vec<String>,
) -> Element {
  let skills = skill_selection.to_vec();
  rsx! {
    for core in core_constraints {
      div { "{core}" }
    }
    div {
      class: "block-columns",
      for skill in skills {
        SkillSelector {
          skill,
          skill_selection: skill_selection.clone(),
        }
      }
    }
  }
}

#[component]
pub fn SkillSelector(
  skill: Skill,
  mut skill_selection: SkillSelections,
) -> Element {
  let id = skill.id.to_string();
  let ranked = skill.is_ranked();

  let rank_map = skill_selection.rank_signal.cloned();
  let rank = *rank_map.get( &id ).unwrap_or( &0 );
  
  let selected = rank > 0;
  let leeway = skill_selection.leeway.get( &id ).unwrap_or( &0 );
  let leeway_rank = leeway / skill.weight();
  let min_rank = 0;
  let mut max_rank = rank + leeway_rank;
  if !ranked {
    max_rank = max_rank.min(1);
  }
  let conditional_class = match (selected, rank >= max_rank) {
    ( true, _ ) => "card-selected",
    ( _, true ) => "disabled",
    _ => "",
  };
  let input_id = id.clone();
  let input_map = rank_map.clone();
  let toggle_id = id.clone();
  let toggle_map = rank_map.clone();

  let ranked_input = match ranked {
    false => None,
    true => Some( rsx! {
      input {
        class: "input", type: "number",
        value: rank, min: min_rank, max: max_rank,
        oninput: move |event| {
          let value = event.value().parse::<i32>()
          .unwrap_or_default()
          .min(max_rank).max(min_rank);
          let new_id = input_id.clone();
          let mut new_map = input_map.clone();
          new_map.insert( new_id, value );
          skill_selection.rank_signal.set( new_map );
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
    } ),
  };
  rsx! {
    div {
      class: "max-content {conditional_class}",
      onclick: move |_| {
        if ranked { return; }
        let new_id = toggle_id.clone();
        let mut new_map = toggle_map.clone();
        match selected {
          true => {
            new_map.insert( new_id, 0 );
          },
          false => {
            if max_rank <= 0 { return; }
            new_map.insert( new_id, 1 );
          },
        };
        skill_selection.rank_signal.set( new_map );
      },
      SkillCard {
        skill,
        display: SkillTermDisplay::Embeded,
        input: ranked_input,
      }
    }
  }
}
