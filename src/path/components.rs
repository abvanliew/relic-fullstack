use dioxus::prelude::*;

use crate::path::keystone::KeystoneDescription;
use crate::path::Path;
use crate::skill::prelude::*;

#[component]
pub fn PathDescription(path: ReadOnlySignal<Path>) -> Element {
  let title = path().title;
  let skills_keystone: Vec<Skill> = path().skills_keystone();
  let skills_full: Vec<Skill> = path().skills_full();
  let skills_half: Vec<Skill> = path().skills_half();
  let spells: Vec<Skill> = path().spells();
  let cantrips: Vec<Skill> = path().cantrips();

  rsx!(
    div { class: "title spacer-medium uv-full", "{title}" }
    if let Some( summary ) = path().summary {
      div { class: "uv-full", "{summary}" }
    }
    SkillTable { skills: skills_keystone.to_owned(), training: false }
    SkillTable { skills: skills_full.to_owned(), training: false }
    if spells.len() > 0 {
      div { class: "uv-full highlight underline spacer-medium", "Spells" }
      SkillTable { skills: spells.to_owned(), training: false, }
    }
    SkillTable { skills: skills_half.to_owned(), training: false }
    if cantrips.len() > 0 {
      div { class: "uv-full highlight underline spacer-medium", "Cantrips" }
      SkillTable { skills: cantrips.to_owned(), training: false }
    }
  )
}

#[component]
pub fn PathTile(path: ReadOnlySignal<Path>) -> Element {
  let title = path().title;
  rsx!(
    div { "{title}" }
    if let Some( summary ) = path().summary {
      div { class: "small-text", "{summary}" }
    }
  )
}
