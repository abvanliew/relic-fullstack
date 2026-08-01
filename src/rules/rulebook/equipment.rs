use crate::pages::QuickTerm;
pub use dioxus::prelude::*;

#[component]
pub fn WeaponExplainer() -> Element {
  rsx! {
    div {
      class: "column gap-2xsmall",
      div { class: "underline highlight", "Armors & Weapons" }
      QuickTerm { title: "Damage", "The amount of damage dealt on hit with the weapon." }
      QuickTerm { title: "Resistance", "The amount of physical resistance granted by the armor." }
      QuickTerm { title: "Attribute Requirements", "Weapons and Armor require the user to have ranks in Physique and Fortitude respectively. If you do not meet this requirement and try to use the item all rolls will be made at disadvantage and you can suffer stacks of Exhaustion." }
      QuickTerm { title: "Melee", "Weapons that allow you to attack and threaten adajacent enemies." }
      QuickTerm { title: "Ranged", "Weapons that allow you to attack at range. These attacks are made at disadvantage if you are threatened in melee. Range is listed as the optional range with a maximum range in parentheses. If you are attacking outside of the optimal range the attack is made at disadvantage." }
      QuickTerm { title: "Thrown", "Weapons that can be used as a melee weapon or thrown to be used as a ranged weapon." }
      QuickTerm { title: "Weight Class", "Different weapons and armors have a weight class (Light, Balanced, Heavy) that interacts with skills. Weapons generally can be used to attack more often the lighter the weight class but don't do as much damage. Heavier armor tends to restrict the use of movement or dodge related skills." }
      QuickTerm { title: "Handed", "The number of hands required to use the weapon effectively" }
      QuickTerm { title: "Reload", "Weapons with the reload property have to be reloaded after each use. This property lists the action required to reload the weapon. This assumes you have ammo readily at hand in a quiever or bandolier. Otherwise you will need to use an Interaction first to draw the ammo." }
      QuickTerm { title: "Bulk", "Armor has a bulk property that is applied as a penalty to your dodge ranks. This penaty can only reduce the dodge benefit from ranks to 0, cannot be negative." }
      QuickTerm { title: "Drag", "This reduces the Speed and Dash values of the wearer. Neither can be reduced below 1." }
      QuickTerm { title: "Elemental Resistance", "Armor with Element Resistance grants resistance to a single elemental damage type in addition to its physical reistance. This element is based on the type of creature the armor is made of." }
    }
  }
}
