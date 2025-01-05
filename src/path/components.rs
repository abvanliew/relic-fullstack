use dioxus::prelude::*;

use crate::path::keystone::KeystoneDescription;
use crate::path::Path;
use crate::skill::prelude::*;

#[component]
pub fn PathDescription( path: ReadOnlySignal<Path> ) -> Element {
  let title = path().title;
  let skills_keystone: Vec<Skill> = path().skills_keystone();
  let skills_full: Vec<Skill> = path().skills_full();
  let skills_half: Vec<Skill> = path().skills_half();
  let spells: Vec<Skill> = path().spells();
  let cantrips: Vec<Skill> = path().cantrips();

  rsx!(
    div { class: "title spacer uv-full", "{title}" }
    if let Some( summary ) = path().summary {
      div { class: "uv-full", "{summary}" }
    }
    if path().keystones.is_some() || skills_keystone.len() > 0 {
      div { class: "uv-full highlight underline spacer", "Keystones" }
    }
    if let Some( keystones ) = path().keystones {
      for keystone in keystones {
        KeystoneDescription { keystone }
      }
    }
    SkillTable { skills: skills_keystone.to_owned(), training: false }
    // if path().full_features.is_some() || skills_full.len() > 0 {
    //   div { class: "uv-full highlight underline spacer", "Features (Full)" }
    // }
    SkillTable { skills: skills_full.to_owned(), training: false }
    if spells.len() > 0 {
      div { class: "uv-full highlight underline spacer", "Spells" }
      SkillTable { skills: spells.to_owned(), training: false, }
    }
    // if path().half_features.is_some() || skills_half.len() > 0 {
    //   div { class: "uv-full highlight underline spacer", "Features (Half)" }
    // }
    SkillTable { skills: skills_half.to_owned(), training: false }
    if cantrips.len() > 0 {
      div { class: "uv-full highlight underline spacer", "Cantrips" }
      SkillTable { skills: cantrips.to_owned(), training: false }
    }
  )
}

#[component]
pub fn PathTile( path: ReadOnlySignal<Path> ) -> Element {
  let title = path().title;
  rsx!(
    div { "{title}" }
    if let Some( summary ) = path().summary {
      div { "{summary}" }
    }
  )
}