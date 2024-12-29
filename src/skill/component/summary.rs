use dioxus::prelude::*;
use crate::Route;
use crate::skill::prelude::*;

#[component]
pub fn SkillTable( skills: Vec<Skill> ) -> Element {
  rsx! {
    for skill in skills {
      SkillSummary { skill: skill.to_owned() }
    }
  }
}

#[component]
pub fn SkillSummary( skill: ReadOnlySignal<Skill> ) -> Element {
  let title = skill.read().title.clone();
  let id = skill.read().id.to_string();
  let training_cost = skill.read().training_cost.to_string();
  let activation = skill.read().action.activation();
  let summary = skill.read().summary.clone().unwrap_or( "".into() );
  rsx!(
    div {
      class: "uv-title nowrap",
      Link {
        to: Route::SingleSkill { id }, "{title}"
      }
    }
    div {
      class: "uv-cost nowrap",
      "{training_cost}"
    }
    div {
      class: "uv-activation nowrap",
      "{activation}"
    }
    div {
      class: "uv-details",
      "{summary}"
    }
  )
}