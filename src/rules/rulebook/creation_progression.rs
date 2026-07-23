use dioxus::prelude::*;
use crate::pages::QuickTerm;
use crate::progression::prelude::{LevelTable, TrainingTables};


#[component]
pub fn CreationExplainer() -> Element {
  rsx! {
    div { class: "subtitle break-before", "Character Creation" }
    QuickStartCreation {}
    LevelChartExplainer {}
    LevelTable {}
    TrainingTables {}
  }
}

#[component]
pub fn QuickStartCreation() -> Element {
  rsx! {
    ol {
      li {
        "At character creation you can either select one or two Paths for your character to start on."
        ul {
          li { "If you select a single path you can pick an extra feature." }
          li { "When a character joins a Path they gain all of Keystone features." }
        }
      }
      li {
        "Pick skills, spell and feature(s)"
        ul {
          li { "Features gained from a keystone are limited to only be selected from the Path granting it." }
          li { "Gain any additional features from your level, which can be choosen from any path you are on or any inherient features." }
          li { "You can always choose to pick two minor features in place of a feature." }
          li { "Spells cost a full feature and Cantrips cost a minor feature." }
        }
      }
      li {
        "Assign initial training rank"
        ul {
          li { "Each training cannot exceed a number of ranks equal to your level." }
          li { "You cannot put points into Innate, Resonance or Magic trainings unless you are on a Path grants the corresponding Flow." }
        }
      }
      li {
        "Assign 20 attribute ranks"
        ul {
          li { "Attributes starts at 0 and can have up to 5 ranks at character creation." }
          li { "You must spend at least 8 ranks on Capabilities and at least 8 ranks on Defenses." }
        }
      }
      li {
        "Select Race and Background, learned defeault languages and assign 20 ranks to expertise max rank of 5."
      }
      li {
        "Records starting HP, Constitution and Resource pools if applicable"
      }
      li {
        "Select weapons and armor your character meets to requirement to use."
      }
    }
  }
}

#[component]
pub fn ProgressChart() -> Element {
  rsx! {
    LevelChartExplainer {}
    TrainingTables {}
  }
}

#[component]
pub fn LevelChartExplainer() -> Element {
  rsx! {
    div {
      QuickTerm { title: "HP", "The amount of health your character has." }
      QuickTerm { title: "Rank Maximum", "The maximum ranks you can allocate to a given attribute or expertise." }
      QuickTerm { title: "Attribute Ranks", "The number of ranks you can spend between your capabilities and defenses. At level 1 you must spend at least 8 ranks in capabilities and 8 ranks in defenses." }
      QuickTerm { title: "Expertise Ranks", "The number of ranks you have to spend on expertise." }
      QuickTerm { title: "Training Ranks", "The number of ranks you can spend on trainings. Each training cannot have more ranks than your current level. All characters can pick from Adept Endurance and Expert trainings. If your character is on a path that provides Innate, Resonance or Magic Flows then you can put ranks into those trainings." }
      QuickTerm { title: "Paths and Features", "Each character must select at least one path to learn. Optionally they can choose to forgo a Feature to learn a new path. Characters are limited to a maximum number of paths they cannot learn more than what is listed." }
      QuickTerm { title: "Specializations", "Specializations increase the effective rank of an attribute or expertise, increasing them above the normal limit of ranks. Each attribute or exptertise can have a maximum of 1 specialization point per tier of the character." }
    }
  }
}