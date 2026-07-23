use dioxus::prelude::*;

#[component]
pub fn DiceRollsThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Dice Rolls"
    }
    div {
      "Attacks and Checks are the two most basic rolls the game calls for, these are made with 3d6 plus a relevant modifier. Attacks rolls add a listed Capability and the total must equal or exceed the target's Defense to hit. Checks are made with a listed Capability or Expertise and must the total must equal or exceed a set difficulty value or a target's Defense to succeed. Each die of a roll can only be re-rolled once, regardless of the number of re-rolls that might apply. A Luck check is a special check using only 3d6 against a difficulty. Luck checks only apply modifiers, re-rolls, advantage or disadvantage specifically called out as applying to a Luck check."
    }
    div {
      class: "underline",
      "Criticals"
    }
    div {
      "Anytime an Attack or Check rolls triples it is a threatened Critical. If the total rolled would have hit or succeeded then the roll is a critical success. If the total rolled would not have missed or failed it is considered to be a hit or success instead. With the exception of a roll of triple 1s, which is a Botch. Botches do not affect the target in any way and do not apply Miss affects. Critical attacks deal an extra dice of damage which can stack with boosted damage rolls."
    }
    div {
      class: "underline",
      "Advantage/Disadvantage"
    }
    div {
      "When you roll with advantage roll 5d6 and pick three dice to use as your result. When you roll with disadvantage, roll 5d6 using the lowest three dice for the result. Multiple sources of advantage or disadvantage never stack. If both advantage and disadvantage would apply to a roll they instead cancel out, regardless of how many sources of advantage or disadvantage would apply. For most rolls you can grant yourself advantage by spending an action point. When a skill calls for you to make a roll against multiple targets, you gain advantage by spending an action point for each target individually. Each action point can only ever improve a single roll against a single target."
    }
    div {
      class: "underline",
      "Damage Rolls"
    }
    div {
      "Damage rolls can be composed of several different dice specified on the Weapon or Spell you are using. Damage rolls do not succeed or fail like Attacks or Checks. Damage is determined by adding the dice result plus any modifiers. Damage is then reduced by the target’s resistance to the type of the damage to a minimum of zero. Damage rolls also do not gain Advantage or Disadvantage, instead you can choose to spend an action point to Boost the damage, granting an extra damage dice. When boosting Area damage affects you must spend an action point for each target you wish to boost the damage towards. Each damage roll can only benefit from one Boost effect, but extra damage dice from other sources stack. If the roll is composed of different dice then you always grain extra dice of the largest dice. If an effect would ever reduce the number of damage die it always removes the largest die first."
    }
    div {
      class: "underline",
      "Fast Rolling"
    }
    div {
      "When a skill has mutliple targets for a single attack or check you can use fast rolling to speed up the resolution. You can make a single Attack or Check roll against groups of targets that are part of a mob or have the same defensive profile. If you have advantage against individual targets roll those in isolation. Area damage rolls should be made once for the skill. If extra damage dice would be applied to select targets from a boost or critical, you can either use the base damage roll and just roll the extra damage die for the individuals or you can choose to roll the full set of damage against that target. Clarify your preference before resolving Area attacks. The goal of this is to speed up gameplay when the average outcome would suffice. If the rolls are pivotal to the outcome of the scene, then make the most impactful rolls individually."
    }
  }
}