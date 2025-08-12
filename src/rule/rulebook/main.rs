use dioxus::prelude::*;
// use crate::rule::snippet::{Snippet, SnippetSetDetails};

#[component]
pub fn MainRulesThread() -> Element {
  rsx! {
    div {
      class: "title",
      "Relic"
    }
    div {
      class: "underhang",
      "Relic is a tabletop role playing game with a focus on tactically oriented combat and intricate character builds."
    }
    DiceRollsThread {}
    ActionEconomyThread {}
    CombatRoundThread {}
    ResourcesThread {}
    NonCombatThread {}
  }
}

#[component]
pub fn DiceRollsThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Dice Rolls"
    }
    div {
      class: "underhang",
      "Attacks and Checks are the two most basic rolls the game calls for, these are made with 3d6 plus a relevant modifier. Attacks rolls add a Capability and are rolled against the target's Defense. Checks are made with a Capability or an Expertise against a set difficulty or Defense. Each die of a roll can only be re-rolled once, regardless of the number of re-rolls that might apply. A Luck check is a special check using only 3d6 against a difficulty. Luck checks only apply modifiers, re-rolls, advantage or disadvantage specifically called out as applying to a Luck check."
    }
    div {
      class: "underline",
      "Criticals"
    }
    div {
      class: "underhang",
      "Anytime an Attack or Check rolls triples it is potentially a Critical roll. A roll of triples is considered a success even if the total would not succeed, but if the total would have succeeded then it is a Critical success. Critical attacks deal an extra dice of damage which stacks with boosted damage rolls. A roll of triple 1s is considered to be a Critical Failure and does not affect the target in any way, it is considered Negated and does not apply Miss affects."
    }
    div {
      class: "underline",
      "Advantage/Disadvantage"
    }
    div {
      class: "underhang",
      "When you roll with advantage roll 5d6 and pick three dice to use as your result. When you roll with disadvantage, roll 5d6 using the lowest three dice for the result. Multiple sources of advantage or disadvantage never stack. If both advantage and disadvantage would apply to a roll they instead cancel out, regardless of how many sources of advantage or disadvantage would apply. For most rolls you can grant yourself advantage by spending an action point."
    }
    div {
      class: "underline",
      "Damage Rolls"
    }
    div {
      class: "underhang",
      "Damage rolls can be composed of several different dice specified on the Weapon or Spell you are using. Damage rolls do not succeed or fail like Attacks or Checks. Damage is determined by adding the dice result plus any modifiers. Damage is then reduced by the target’s resistance to the type of the damage to a minimum of zero. Damage rolls also do not gain Advantage or Disadvantage, instead you can choose to spend an action point to Boost the damage, granting an extra damage dice. Each damage roll can only benefit from one Boost effect, but extra damage dice from other sources stack. If the roll is composed of different dice then you always grain extra dice of the largest dice. If an effect would ever reduce the number of damage die it always removes the largest die first."
    }
  }
}

#[component]
pub fn ActionEconomyThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Action Economy"
    }
    ActionEconomyTable {}
    div {
      class: "underhang",
      "The core unit of action economy within the game is the action point. Player characters will have three actions to use in a round. Each skill has a type which indicates when it can be used and how many action points it costs to use. The most basic type is an Action which costs a single action point. If the type is listed as Complex then it costs two action points instead. Actions can only be used during your own turn where Reactions can be used throughout the round. Some skills will have Initial listed next to their action cost, these Actions can only be used once in a given round."
    }
    div {
      class: "underline",
      "Reactions"
    }
    div {
      class: "underhang",
      "Reactions can be used at any point in a round and they cost a single action point. If a Reaction has a Condition listed on the skill then it can only be used when that condition is met. Characters get a single free Reaction each round of combat. Reflexes and Triggers are skill types that function as Reactions, but do not cost an action point. The difference being a Reflex requires a character to choose to use it, whereas a Trigger always happens if the condition is met."
    }
    div {
      class: "underline",
      "Advantage and Boosts"
    }
    div {
      class: "underhang",
      "You can also spend action points to improve rolls. Before making an Attack or Check you can spend an action point to gain advantage on that roll. Once an Attack hits you can choose to spend an action point to boost the damage, gaining an extra dice. If you are using a Spell or Skill that has a resource cost you must spend the appropriate resource, in place of an action point, to gain advantage or to boost the roll."
    }
    div {
      class: "underline",
      "Interactions"
    }
    div {
      class: "underhang",
      "Interaction is a type of Action which is commonly used when characters use items or manipulate simple objects in the environment. Interactions function as normal Actions costing a single action point, but each character has one free Interaction to use each round."
    }
    div {
      class: "underline",
      "Movement"
    }
    div {
      class: "underhang",
      "Each turn a character can move up to their speed in spaces without spending action points. Characters can use the Dash Reaction to gain 3 extra spaces of movement at any point during the round. You can move between attacks or checks, but each attack or check must be resolved completely before moving again. You can spend 3 movement to stand up from prone."
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
    }
  }
}

#[component]
pub fn CombatRoundThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Combat Rounds"
    }
    div {
      class: "underline",
      "Initiative"
    }
    div {
      class: "underhang",
      "Each character that is participating in combat and not surprised at the start of combat has a single initiative card that is shuffled together. At the start of each round a single card is revealed, that character or creature acts first in that round of combat. End of a round if a creature joined the combat or lost the surprised condition they are shuffled into the initiative deck. Most creatures have a single initiative card, but units or hordes of similar creatures will often act at the same time sharing a single initiative card. Some powerful or elite creatures might have multiple initiative cards for an increased chance of acting first."
    }
    div {
      class: "underhang",
      "Whichever creature is revealed must act first in a round. Play then continues where the DM and Players alternate activating creatures and or monsters. Surprised creatures cannot act until all non-surprised creatures have finished their turns. Some powerful elite and legendary creatures can get multiple actions in a given round."
    }
    div {
      class: "underline",
      "Refresh"
    }
    div {
      class: "underhang",
      "At the start of each round all characters have their action points and resources are restored. First resolve any start of round effect that can impact a character’s action economy or resource pools. All readied actions from the previous round are lost and each character’s Actions Points are set to 3 or their current Constitution value whichever is lower. They get a Reaction, an Interaction and Movement equal to their speed. If a creature is surprised, then they start the round with no action points, reactions or movement. When they first act in the turn they gain the actions points, reactions and movement they would have gotten during a refresh. Everything else is refreshed as normal for surprised characters."
    }
    div {
      class: "underhang",
      "Each creature’s resource pools are restored to their current maximum values. A creature can choose to end any effect that suspends resources to restore them to their pool. They can also choose to pay any upkeep costs to maintain effects that would expire. Any other Refreshing stacks are restored to the full value, unused stacks from the last round are lost. Resolve any other start of round effects such as ongoing damage."
    }
    div {
      class: "underline break-before",
      "Distances"
    }
    div {
      class: "underhang",
      "The basic unit of distance measurement is the space. Each space represents 5ft/1 m of distance in the game world or a single square/hex on a combat grid. When using a square grid the first diagonal square measured is considered a distance of one. The second diagonal is considered a distance of two. Further diagonals alternate between distances of one and two. Movement on a square grid follows this same rule with the first Diagonal moved after a refresh being one space and then all movement for that round continuing to alternate."
    }
    div {
      class: "underline",
      "Movement"
    }
    div {
      class: "underhang",
      "When a character’s movement is impeded or slowed by an effect they need to spend an extra movement to move each space. If a character wishes to leave a space where they are in melee threat range, they will need to spend an extra movement. If a character has more than one effect that increases the cost to move then they will need to spend a maximum of 3 movement per space moved. A character that is not immobilized and has some movement to spend can always move at least 1 space in a round regardless of hindrances."
    }
    div {
      class: "underhang",
      "Characters can choose to make a Steady movement that uses an extra point of movement per space moved. Moving at a Steady pace can be called for when trying to lift heavy objects, it can affect how stealth rolls are made, or might allow you to more safely move over narrow or slippery terrain."
    }
    div {
      class: "underline",
      "Shift"
    }
    div {
      class: "underhang",
      "Certain skills will allow a character to Shift a number of spaces. When Shifting you can move ignoring the melee threat movement penalty and you also ignore any movement based reactions. If a character is forced to move they also ignore the melee threat penalty and will not trigger movement based reactions."
    }
  }
}

#[component]
pub fn ResourcesThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Resources"
    }
    ResourcesTable {}
    div {
      class: "underhang",
      "Paths often provide characters access to unique resources like Mana, Ki or Rage. When a character starts a path it will provide them a pool of that resource as well as a corresponding Flow value. The Magic flow constrains Mana which fuels spell casting. Resonance Flow is for channeling mystic energies. Whereas Innate limits powers that rely on the inner strength of the character. Each turn you only have access to an amount of a given resource equal to the smaller of the current pool or corresponding flow value. In addition you cannot spend more total resources than their corresponding Flow value per round."
    }
    div {
      class: "underline",
      "Resource Pools"
    }
    div {
      class: "underhang",
      "Spells and Skills can have a resource cost that need to be paid to use that use. When you spend resources you must roll a drain die for each resource spent. On the roll of a 1 or 2 that pool suffers a point of drain. Drain reduces the size of a resource pool, pulling from the reserves of a pool first. Reserves are the portion of a resource pool in excess of its flow value. Drain must fully deplete reserves before it starts affecting the actual amount of resources you can use per round. When you complete a night’s rest drain is fully removed from all resource pools restoring them to their maximum value."
    }
    div {
      class: "underline",
      "Upkeep"
    }
    div {
      class: "underhang",
      "At the end of the duration of the skill with an Upkeep keyword you can pay the skill’s initial cost (or a specified Upkeep cost if listed) to extend the effect by its listed duration. You do not need to make new attack rolls or spend action points for the skill, just pay the upkeep cost. Any stacks or other expendable part of the effect are not refreshed, only the duration. Discounts or reductions to the initial cost do not affect the upkeep cost, they are determined independently of initial cost reductions. You can only maintain a number of Upkeep effects whose total upkeep cost does not exceed your current limit for that resource."
    }
    div {
      class: "underline",
      "Reserves"
    }
    div {
      class: "underhang",
      "Some effects will have a duration that indicates it lasts for as long as you Reserve an amount of a resource. Reserving a resource reduces both that resource pool and its corresponding Flow by the amount listed and does not cause any drain. You can only Reserve if you currently have that much remaining Flow or Resource available to use. If for any reason you have more of a resource reserved that the current pool or flow would allow you must end reservation effects so that you can have at most equal to your current limit. Characters can also choose to end the effect at any point. If the effect ends during the refresh phase your flow and resources are available immediately."
    }
    div {
      class: "underhang",
      "To gain advantage on spells or skills with resource costs you must spend the corresponding resource instead of spending action points. Cantrips are considered Spells even if they do not have a Mana cost listed and you must spend Mana to gain advantage or boost their damage rolls."
    }
    div {
      class: "underline",
      "Mana and Spellcasting"
    }
    div {
      class: "underhang",
      "Mana is a unique resource as it has three different pools, one for each tier of magic: Minor for Initiate, Moderate for Journeyman and Major for Master. A spell can be cast at a higher tier if you can spend Mana of that tier or higher. To gain advantage or boost damage you must also spend mana from its tier or higher. Unlike other resources the drain dice does not change for different tiers of mana, but the value that it drains on changes. Moderate Mana drains on 1, 2 or 3 and Major Mana drains on 1, 2, 3 or 4. Unless otherwise noted spells are considered to have the Verbal, Somatic and Foci keywords. A spell’s Persistence is equal to the caster’s Resolve."
    }
    div {
      class: "underline",
      "Spell Preperation"
    }
    div {
      class: "underhang",
      "Spell casters have a list of spells known and a number of spell slots. To cast a known spell it must be prepared into a spell slot. Some spells are learned as spontaneous spells, these are always considered to be prepared and do not require a spell slot. Once per day a caster can take an hour in quiet contemplation to reselect what spells are prepared in their spell slots. A creature that has spell slots can learn new spells during downtime from other casters or written works. Spell casters each have a unique perspective on where their magic comes from and how magic works known as their paradigm. Spell casters can only learn spells from fellow casters or texts written from their paradigm."
    }
  }
}

#[component]
pub fn ResourcesTable() -> Element {
  rsx! {
    div {
      class: "grid dim-resource-chart table indent float-right",
      div { class: "uv-flow-area center underline", "Flow" }
      div { class: "uv-resource-area underline", "Resource" }
      div { class: "uv-drain-area underline", "Drain" }

      div { class: "uv-innate-area middle highlight", "Innate" }
      div { class: "uv-anointment-area", "Anointment" }
      div { class: "uv-anointment-drain-area", "d6" }
      div { class: "uv-animalism-area", "Animalism" }
      div { class: "uv-animalism-drain-area", "d8" }
      div { class: "uv-sanguine-area", "Sanguine" }
      div { class: "uv-sanguine-drain-area", "d10" }
      div { class: "uv-rage-area", "Rage" }
      div { class: "uv-rage-drain-area", "d12" }

      div { class: "uv-resonance-area middle highlight", "Resonance" }
      div { class: "uv-mastery-area", "Mastery" }
      div { class: "uv-mastery-drain-area", "d6" }
      div { class: "uv-channel-area", "Channel" }
      div { class: "uv-channel-drain-area", "d8" }
      div { class: "uv-ki-area", "Ki" }
      div { class: "uv-ki-drain-area", "d8" }
      div { class: "uv-virtuoso-area", "Virtuoso" }
      div { class: "uv-virtuoso-drain-area", "d10" }

      div { class: "uv-magic-area middle highlight", "Magic" }
      div { class: "uv-minor-mana-area", "Minor Mana" }
      div { class: "uv-minor-mana-drain-area", "d10" }
      div { class: "uv-moderate-mana-area", "Moderate Mana" }
      div { class: "uv-moderate-mana-drain-area",
        span { "d10" }
        span { class: "italics", " on 1-3" }
      }
      div { class: "uv-major-mana-area", "Major Mana" }
      div { class: "uv-major-mana-drain-area",
        span { "d10" }
        span { class: "italics", " on 1-4" }
      }

    }
  }
}

#[component]
pub fn NonCombatThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Out of Combat Actions"
    }
    div {
      class: "underhang",
      "Spending action points to represent what your character is doing second by second in combat is compelling and tactical, but many important scenes are played out over longer periods of time. When outside of combat instead of spending action points players assign action points. They can be assigned to either accomplish a task or to watch for a specific event. Characters can move at a relaxed pace and hold a simple conversation without needing to assign action points."
    }
    div {
      class: "underline",
      "Tasks"
    }
    div {
      class: "underhang",
      "Tasks cover the majority of actions that a player could take from trying to convince a guard of their innocence or to making a sandwich. If the task is particularly complex a DM can require two action points be assigned, like when building a clockwork device or baking a souffle. Characters can also assign an extra action point to a task to gain advantage on any rolls related to the task. Some tasks might require a character to move at a steady pace, like carrying a fragile piece of art or looking for traps in the dungeon. In these cases you must move at half speed to perform the action (or move at full speed with a penalty set by the DM)."
    }
    div {
      class: "underline",
      "Watching"
    }
    div {
      class: "underhang",
      "A character can also keep an eye out for a specific event, object or situation. They might try to determine if an npc is lying or if someone is following them. While watching for that event the character receives a +4 Defense bonus against any check made to hide from or deceive them or gains advantage on expertise checks made to understand details related to the situation. In some cases the DM can indicate that watching for a specific event would be complex, requiring two dice to be assigned. Such as searching for a trap or listening for a flaw in a legal defense."
    }
    div {
      class: "underline",
      "Context Switching"
    }
    div {
      class: "underhang",
      "If the player wishes to, they can reassign how their action points are distributed. A player can reassign an action point in the middle of doing an action, but cannot then reassign it again for at least another minute. Trying to constantly switch between multiple tasks is difficult and the DM can rule that longer than a minute might be needed if a player is trying to switch between substantially different tasks."
    }
    div {
      class: "underline",
      "Combat Start"
    }
    div {
      class: "underhang",
      "If combat begins while a character has all of their action points assigned they will automatically be surprised. Unless that character was watching for and successfully noticed the event that triggered combat."
    }
  }
}
