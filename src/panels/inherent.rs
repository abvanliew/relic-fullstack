use dioxus::prelude::*;
use crate::server::prelude::{GameLibrarySignal};
use crate::skill::prelude::TrainingCost;
use crate::skill::{prelude::SkillDescription, Skill};

#[component]
pub fn InherentSkills() -> Element {
  let signal = use_context::<GameLibrarySignal>();
  let skills_response = signal.get_skills();
  let results = match skills_response.clone() {
    Some( skills ) => {
      let mut inherent_skills: Vec<Skill> = skills.into_iter()
      .filter_map(
        |( _, skill)|
        if skill.training_cost == TrainingCost::Inherient { Some( skill.clone() ) } else { None }
      )
      .collect();
      inherent_skills.sort_by_key( |skill| skill.title.clone() );
      Some( inherent_skills )
    },
    _ => { None },
  };
  return match results {
    None => {
      rsx! { div { r#"Loading"# } }
    },
    Some( skills ) => {
      rsx! {
        div {
          class: "row-wrap",
          for skill in skills {
            SkillDescription { id: skill.id.to_string(), show_terms: true }
          }
        }
      }
    }
  }
}