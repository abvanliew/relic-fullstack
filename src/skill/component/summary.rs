use crate::skill::prelude::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SkillTable(skills: Vec<Skill>, training: bool) -> Element {
  rsx! {
    for skill in skills {
      SkillSummary { skill: skill.to_owned(), training }
    }
  }
}

#[component]
pub fn SkillSummary(skill: ReadOnlySignal<Skill>, training: bool) -> Element {
  let title = skill.read().title.clone();
  let id = skill.read().id.to_string();
  let training_cost = skill.read().training_cost.to_string();
  let activation = skill.read().action.activation();
  let summary = skill.read().summary.clone().unwrap_or("".into());
  rsx!(
    div {
      class: "uv-title",
      Link { to: Route::SingleSkillPage { id }, "{title}" }
    }
    if training {
      div {
        class: "uv-cost centered",
        "{training_cost}"
      }
    }
    div {
      class: "uv-activation centered",
      "{activation}"
    }
    div {
      class: "uv-details",
      "{summary}"
    }
  )
}

#[component]
pub fn SkillTile(skill: ReadOnlySignal<Skill>) -> Element {
  let title = skill.read().title.clone();
  let activation = skill.read().action.activation();
  rsx!(
    div { "{title}" }
    div { "{activation}" }
    if let Some( summary ) = &skill.read().summary {
      div { "{summary}" }
    }
  )
}
