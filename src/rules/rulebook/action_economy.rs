use dioxus::prelude::*;

#[component]
pub fn ActionEconomyThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Action Economy"
    }
    ActionEconomyTable {}
    div {
      "The core unit of action economy within the game is the action point. Player characters will have three actions to use in a round. Each skill has a type which indicates when it can be used and how many action points it costs to use. The most basic type is an Action which costs a single action point. If the type is listed as Complex then it costs two action points instead. Actions can only be used during your own turn where Reactions can be used throughout the round. Some skills will have Initial listed next to their action cost, these Actions can only be used once in a given round."
    }
    div {
      class: "underline",
      "Reactions"
    }
    div {
      "Reactions can be used at any point in a round and they cost a single action point. If a Reaction has a Condition listed on the skill then it can only be used when that condition is met. Characters get a single free Reaction each round of combat. Reflexes and Triggers are skill types that function as Reactions, but do not cost an action point. The difference being a Reflex requires a character to choose to use it, whereas a Trigger always happens if the condition is met."
    }
    div {
      class: "underline",
      "Advantage and Boosts"
    }
    div {
      "You can also spend action points to improve rolls. Before making an Attack or Check you can spend an action point to gain advantage on that roll. Once an Attack hits you can choose to spend an action point to boost the damage, gaining an extra dice. If you are using a Spell or Skill that has a resource cost you must spend the appropriate resource, in place of an action point, to gain advantage or to boost the roll."
    }
    div {
      class: "underline",
      "Interactions"
    }
    div {
      "Interaction is a type of Action which is commonly used when characters use items or manipulate simple objects in the environment. Interactions function as normal Actions costing a single action point, but each character has one free Interaction to use each round."
    }
    div {
      class: "underline",
      "Readied Actions"
    }
    div {
      "Many skills will grant a readied actions. A readied action gives you an action point that can be within a given restriction or with skills of a certain keyword. Readied actions allow you to use simple Action as Reactions. Alternatively you can use the readied action to gain advantage or boost the damage of an applicable skill. Readied actions do not count as Action Points for purposes of conditions or action point limits."
    }
    div {
      class: "underline",
      "Movement"
    }
    div {
      "Each turn a character can move up to their speed in spaces without spending action points. Characters can use the Dash skill to gain 3 extra spaces of movement. You can move between attacks or checks, but each attack or check must be resolved completely before moving again. You can spend 3 movement to stand up from prone."
    }
    div {
      "When a character’s movement is impeded or slowed by an effect they need to spend an extra movement to move each space. If a character wishes to leave a space where they are in melee threat range, they will need to spend an extra movement. If a character has more than one effect that increases the cost to move then they will need to spend a maximum of 3 movement per space moved. A character that is not immobilized and has some movement to spend can always move at least 1 space in a round and use of a movmement related skill regardless of hindrances."
    }
    div {
      "Characters can choose to make a Steady movement that uses an extra point of movement per space moved. Moving at a Steady pace can be called for when trying to lift heavy objects, it can affect how stealth rolls are made, or might allow you to more safely move over narrow or slippery terrain."
    }
    div {
      class: "underline",
      "Shift"
    }
    div {
      "Certain skills will allow a character to Shift a number of spaces. When Shifting you can move ignoring the melee threat movement penalty and you also ignore any movement based reactions. If a character is forced to move they also ignore the melee threat penalty and will not trigger movement based reactions."
    }
    div {
      class: "underline",
      "Distances"
    }
    div {
      "The basic unit of distance measurement is the space. Each space represents 5ft, 1 yard, or 1m of distance in the game world or a single square/hex on a combat grid. When measuring distance on a square grid diagonals alternate between counting as 1 space then 2 spaces. Movement on a square grid follows this same rule with the first diagonal moved using one movement then alternating after that."
    }
  }
}

#[component]
pub fn ActionEconomyTable() -> Element {
  rsx! {
    div {
      class: "grid table dim-triple indent float-right",
      div { class: "uv-full", "3 Action Points a Round" }
      div {}
      div { class: "centered", "AP Cost" }
      div { class: "centered", "Free" }
      div { "Complex Action" }
      div { class: "centered", "2" }
      div {}
      div { "Action" }
      div { class: "centered", "1" }
      div {}
      div { "Interaction" }
      div { class: "centered", "1" }
      div { class: "centered", "1" }
      div { "Reaction" }
      div { class: "centered", "1" }
      div { class: "centered", "1" }
      div { "Reflex" }
      div { class: "centered", "0" }
      div {}
      div { "Trigger" }
      div { class: "centered", "0" }
      div {}
      div { class: "uv-full italics fit-width", "Actions with the Intial qualifier can be used once per Round" }
    }
  }
}

#[component]
pub fn CombatRoundThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Combat Round"
    }
    div {
      "Combat is played out in rounds where creatures alternate taking turns."
    }
    div {
      class: "underline",
      "Initiative"
    }
    div {
      "Each character that is participating in combat and not surprised at the start of combat has a single initiative card that is shuffled together. At the start of each round a single card is revealed, that character or creature acts first in that round of combat. End of a round if a creature joined the combat or lost the surprised condition they are shuffled into the initiative deck. Most creatures have a single initiative card, but units or hordes of similar creatures will often act at the same time sharing a single initiative card. Some powerful or elite creatures might have multiple initiative cards for an increased chance of acting first."
    }
    div {
      "Whichever creature is revealed must act first in a round. Play then continues where the DM and Players alternate activating creatures and or monsters. Surprised creatures cannot act until all non-surprised creatures have finished their turns. Some powerful elite and legendary creatures can get multiple actions in a given round."
    }
    div {
      class: "underline",
      "Refresh"
    }
    div {
      "At the start of each round all characters have their action points and resources are restored. First resolve any start of round effect that can impact a character’s action economy or resource pools. All readied actions from the previous round are lost and each character’s Actions Points are set to 3 or their current Constitution value whichever is lower. They get a Reaction, an Interaction and Movement equal to their speed. If a creature is surprised, then they start the round with no action points, reactions or movement. When they first act in the turn they gain the actions points, reactions and movement they would have gotten during a refresh. Everything else is refreshed as normal for surprised characters."
    }
    div {
      "Each creature’s resource pools are restored to their current maximum values. A creature can choose to end any effect that suspends resources to restore them to their pool. They can also choose to pay any upkeep costs to maintain effects that would expire. Any other Refreshing stacks are restored to the full value, unused stacks from the last round are lost. Resolve any other start of round effects such as ongoing damage."
    }
  }
}
