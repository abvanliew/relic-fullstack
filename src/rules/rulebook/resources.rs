use dioxus::prelude::*;

#[component]
pub fn ResourcesThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Resources"
    }
    ResourcesTable {}
    div {
      "Paths often provide characters access to unique resources like Mana, Ki or Rage. When a character starts a path it will provide them a pool of that resource as well as a corresponding Flow value. The Magic flow constrains Mana which fuels spell casting. Resonance Flow is for channeling mystic energies. Whereas Innate limits powers that rely on the inner strength of the character. Each turn you only have access to an amount of a given resource equal to the smaller of the current pool or corresponding flow value. In addition you cannot spend more total resources than their corresponding Flow value per round."
    }
    div {
      class: "underline",
      "Resource Pools"
    }
    div {
      "Spells and Skills can have a resource cost that need to be paid to use that use. When you spend resources you must roll a drain die for each resource spent. On the roll of a 1 or 2 that pool suffers a point of drain. Drain reduces the size of a resource pool, pulling from the reserves of a pool first. Reserves are the portion of a resource pool in excess of its flow value. Drain must fully deplete reserves before it starts affecting the actual amount of resources you can use per round. When you complete a night’s rest drain is fully removed from all resource pools restoring them to their maximum value."
    }
    div {
      class: "underline",
      "Upkeep"
    }
    div {
      "At the end of the duration of the skill with an Upkeep property you can pay the skill’s initial cost (or a specified Upkeep cost if listed) to extend the effect by its base duration. You do not need to make new attack rolls or spend action points for the skill, just pay the upkeep cost. Any stacks or other expendable part of the effect are not refreshed, only the duration. Discounts or reductions to the initial cost do not affect the upkeep cost, they are determined independently of initial cost reductions."
    }
    div {
      class: "underline",
      "Reserves"
    }
    div {
      "Some effects will have a duration that indicates it lasts for as long as you Reserve an amount of a resource. Reserving a resource reduces both that resource pool and its corresponding Flow by the amount listed and does not cause any drain. You can only Reserve if you currently have that much remaining Flow or Resource available to use. If for any reason you have more of a resource reserved that the current pool or flow would allow you must end reservation effects so that you can have at most equal to your current limit. Characters can also choose to end the effect at any point. If the effect ends during the refresh phase your flow and resources are available immediately."
    }
    div {
      "To gain advantage on spells or skills with resource costs you must spend the corresponding resource instead of spending action points. Cantrips are considered Spells even if they do not have a Mana cost listed and you must spend Mana to gain advantage or boost their damage rolls."
    }
    div {
      class: "underline",
      "Mana and Spellcasting"
    }
    div {
      "Mana is a unique resource as it has three different pools, one for each tier of magic: Minor for Initiate, Moderate for Journeyman and Major for Master. A spell can be cast at a higher tier if you can spend Mana of that tier or higher. To gain advantage or boost damage you must also spend mana from its tier or higher. Unlike other resources the drain dice does not change for different tiers of mana, but the value that it drains on changes. Moderate Mana drains on 1, 2 or 3 and Major Mana drains on 1, 2, 3 or 4. Unless otherwise noted spells are considered to have the Verbal, Somatic and Foci keywords. A spell’s Persistence is equal to the caster’s Resolve."
    }
    div {
      class: "underline",
      "Spell Preperation"
    }
    div {
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
      div { class: "uv-animism-area", "Animism" }
      div { class: "uv-animism-drain-area", "d8" }
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
