use dioxus::prelude::*;

use crate::keyword::prelude::*;
use crate::pages::QuickTerm;
use crate::rules::rulebook::action_economy::{ActionEconomyThread, CombatRoundThread};
use crate::rules::rulebook::creation_progression::CreationExplainer;
use crate::rules::rulebook::dice_rolling::DiceRollsThread;
use crate::rules::rulebook::example_skills::{awesome_spell, ranked_boon, secret_handshake};
use crate::rules::rulebook::out_of_combat::OutOfCombatThread;
use crate::rules::rulebook::resources::ResourcesThread;
use crate::skill::component::SkillCard;

#[component]
pub fn MainRulesThread() -> Element {
  rsx! {
    div {
      class: "rulesbook",
      RelicIntroduction {}
      DiceRollsThread {}
      ActionEconomyThread {}
      CombatRoundThread {}
      ResourcesThread {}
      OutOfCombatThread {}
      CreationExplainer {}
      ReadingSkillCards {}
      TermsConditions {}
    }
  }
}

#[component]
pub fn RelicIntroduction() -> Element {
  rsx! {
    div {
      class: "title",
      "Relic"
    }
    div {
      "Relic is a tabletop role playing game with a focus on tactically oriented combat and intricate character builds."
    }
  }
}

#[component]
pub fn ReadingSkillCards() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Reading Skill Cards"
    }
    div {
      "The skill card is the representation of all actions you can do in Relic. Reading and understanding terms on a card is very important to understanding how your spells and abilities will work."
    }
    ExampleSkills {}
    div {
      span { class: "highlight", "Skill Name" }
      " - The underlined bold text on the top left of the card is the skill name."
    }
    QuickTerm { 
      title: "Training Requirements", 
      "In the top right of a card in italics you will see its tier and training cost. The tiers are "
      span { class: "italics", "Initiate, Journeyman and Master" }
      ". Your character must be at least the tier listed to learn the skill. If the training cost is qualified as a Ranked then the skill can be taken multiple times for increased affect, otherwise it can learned once. The training cost of a skill can be "
      span { class: "italics", "Inherient, Keystone, Feature, Minor Feature, Spell or Cantrip" }
      ". Inherient skills are available to all characters. Keystones are granted when you start a path associated with the skill. Otherwise you must spend a feature or minor feature to learn the skill. Spell or Cantrip can be learned in place of a Feature or Minor Feature respectively. In some cases you will be limited to selecting from Spells or Cantrips."
    }
    QuickTerm {
      title: "Usage",
      "Below the Skill's name is it's activation in bold and its associated keywords in italics. If an activation is listed as Initial, then it can only be used once per round. Activations are covered in more depth in the Action Economy rules with the exception of the Boon which is a passive bonus. The keywords help determine how this skill might interact with other skills or game effects."
    }
    QuickTerm {
      title: "Condition",
      "Any requirement or trigger to use a skill go hear. What triggers a skill with often dictate what it targets."
    }
    QuickTerm {
      title: "Cost",
      "Some skills have a resource cost listed to use them. You must spent those resources to use the skill in parentheses the drain die for that resource type is listed."
    }
    QuickTerm {
      title: "Duration",
      "How long a skill affect lasts, either rounds, a length of time, or while resources are reserved. If there is an upkeep listed then this is the cost you can chose pay when the affect would end to refresh its duration. Skills that has limited number of uses will also mention that last until expended and will end when their uses are spent."
    }
    QuickTerm {
      title: "Target",
      "This explains what the skill can affect. Including restrictions on whether the skill affect enenmies or allies or creatures or objects. It will often list the range in which the skill operates. When refering to range of reach that is generally adjacant target but larger creatures can have longer reach."
    }
    QuickTerm {
      title: "Refresh",
      "If a skill lists something as a Refresh then it applies a benefit at the start of each round. If a stack of something is refresh then it restores expended stacks but does not accure more than the listed amount."
    }
    QuickTerm {
      title: "Secondary Actions",
      "In some cases skills learned will give you the ability to do more than one action or carry with it certain triggers. These secondary actions will have their own action name, activation and keywords. Generally these actions are dependant on the primary action and it will describe how and when to use them."
    }
    div { class: "clear-both" }
  }
}


#[component]
pub fn ExampleSkills() -> Element {
  let ranked_boon = ranked_boon();
  let awesome_spell = awesome_spell();
  let secret_handshake = secret_handshake();
  rsx! {
    div {
      class: "column gap-large indent float-right",
      SkillCard { skill: ranked_boon }
      SkillCard { skill: awesome_spell }
      SkillCard { skill: secret_handshake }
    }
  }
}




