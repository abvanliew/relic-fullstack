use dioxus::prelude::*;

use std::fmt;
use serde::{Deserialize, Serialize};

use crate::rule::components::Modifier;

// use self::Attribute::{Capability as CapabilityClass,Defense as DefenseClass};
// use self::Capability::{Manipulation,Physique,Spirit,Warfare};
// use self::Defense::{Dodge,Fortitude,Insight,Resolve,Tenacity};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributeMask {
  pub capabilities: Option<Vec<Capability>>,
  pub defenses: Option<Vec<Defense>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AttributeRanks {
  pub physique: i32,
  pub warfare: i32,
  pub spirit: i32,
  pub manipulation: i32,
  pub tenacity: i32,
  pub fortitude: i32,
  pub resolve: i32,
  pub insight: i32,
}

// impl AttributeSet {
//   pub fn get_value( &self, attribute: &AttributeType ) -> i32 {
//     match attribute {
//       CapabilityType(Physique) => self.physique,
//       CapabilityType(Warfare) => self.warfare,
//       CapabilityType(Spirit) => self.spirit,
//       CapabilityType(Manipulation) => self.manipulation,
//       DefenseType(Tenacity) => self.tenacity,
//       DefenseType(Fortitude) => self.fortitude,
//       DefenseType(Resolve) => self.resolve,
//       DefenseType(Insight) => self.insight,
//       DefenseType(Dodge) => self.dodge,
//     }
//   }
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AttributeSignal {
  pub physique: Signal<i32>,
  pub warfare: Signal<i32>,
  pub spirit: Signal<i32>,
  pub manipulation: Signal<i32>,
  pub tenacity: Signal<i32>,
  pub fortitude: Signal<i32>,
  pub resolve: Signal<i32>,
  pub insight: Signal<i32>,
  pub dodge: Signal<i32>,
}

// impl AttributeSignal {
//   pub fn use_context_provider()-> Self {
//     let physique = use_signal( || 0 );
//     let warfare = use_signal( || 0 );
//     let spirit = use_signal( || 0 );
//     let manipulation = use_signal( || 0 );
//     let tenacity = use_signal( || 0 );
//     let fortitude = use_signal( || 0 );
//     let resolve = use_signal( || 0 );
//     let insight = use_signal( || 0 );
//     let dodge = use_signal( || 0 );
//     use_context_provider( || Self{
//       physique, warfare, spirit, manipulation,
//       tenacity, fortitude, resolve, insight, dodge
//     } )
//   }

//   pub fn cap( &mut self, value: i32 ) {
//     if (self.physique)() > value { self.physique.set( value ); }
//     if (self.warfare)() > value { self.warfare.set( value ); }
//     if (self.spirit)() > value { self.spirit.set( value ); }
//     if (self.manipulation)() > value { self.manipulation.set( value ); }
//     if (self.tenacity)() > value { self.tenacity.set( value ); }
//     if (self.fortitude)() > value { self.fortitude.set( value ); }
//     if (self.resolve)() > value { self.resolve.set( value ); }
//     if (self.insight)() > value { self.insight.set( value ); }
//     if (self.dodge)() > value { self.dodge.set( value ); }
//   }

//   pub fn get( &self, attribute: &Attribute ) -> Signal<i32> {
//     match attribute {
//       CapabilityClass(Physique) => self.physique,
//       CapabilityClass(Warfare) => self.warfare,
//       CapabilityClass(Spirit) => self.spirit,
//       CapabilityClass(Manipulation) => self.manipulation,
//       DefenseClass(Tenacity) => self.tenacity,
//       DefenseClass(Fortitude) => self.fortitude,
//       DefenseClass(Resolve) => self.resolve,
//       DefenseClass(Insight) => self.insight,
//       DefenseClass(Dodge) => self.dodge,
//     }
//   }

//   pub fn sum( &self ) -> i32 {
//     (self.physique)() + (self.warfare)() + (self.spirit)() + (self.manipulation)() +
//     (self.tenacity)() + (self.fortitude)() + (self.resolve)() + (self.insight)() + (self.dodge)()
//   }

//   pub fn cap_def( &self ) -> ( i32, i32 ) { (
//     (self.physique)() + (self.warfare)() + (self.spirit)() + (self.manipulation)(),
//     (self.tenacity)() + (self.fortitude)() + (self.resolve)() + (self.insight)() + (self.dodge)(),
//   ) }
// }

#[component]
pub fn AttributeDetails( attributes: AttributeRanks ) -> Element {
  rsx!(
    div { "Capabilites" }
    div {
      class: "row",
      div { "Physique" }
      div { Modifier { value: attributes.physique } }
    }
    div {
      class: "row",
      div { "Warfare" }
      div { Modifier { value: attributes.warfare } }
    }
    div {
      class: "row",
      div { "Spirit" }
      div { Modifier { value: attributes.spirit } }
    }
    div {
      class: "row",
      div { "Manipulation" }
      div { Modifier { value: attributes.manipulation } }
    }
    div { "Defenses" }
    div {
      class: "row",
      div { "Tenacity" }
      div { "{attributes.tenacity + 10}" }
    }
    div {
      class: "row",
      div { "Fortitude" }
      div { "{attributes.fortitude + 10}" }
    }
    div {
      class: "row",
      div { "Resolve" }
      div { "{attributes.resolve + 10}" }
    }
    div {
      class: "row",
      div { "Insight" }
      div { "{attributes.insight + 10}" }
    }
  )
}

#[derive(Debug, Clone, PartialEq)]
pub enum RankClass {
  Capability,
  Defense,
  Expertise,
}

impl fmt::Display for RankClass {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      RankClass::Capability => "Capability",
      RankClass::Defense => "Defense",
      RankClass::Expertise => "Expertise",
    } )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
  Capability( Capability ),
  Defense( Defense ),
}

impl fmt::Display for Attribute {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Attribute::Capability(capability) => format!( "{capability}" ),
      Attribute::Defense(defense) => format!( "{defense}" ),
    } )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Capability {
  Physique,
  Warfare,
  Spirit,
  Manipulation,
}

impl fmt::Display for Capability {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Capability::Physique => "Physique",
      Capability::Warfare => "Warfare",
      Capability::Spirit => "Spirit",
      Capability::Manipulation => "Manipulation",
    } )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Defense {
  Tenacity,
  Fortitude,
  Resolve,
  Insight,
  Dodge,
}

impl fmt::Display for Defense {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Defense::Tenacity => "Tenacity",
      Defense::Fortitude => "Fortitude",
      Defense::Resolve => "Resolve",
      Defense::Insight => "Insight",
      Defense::Dodge => "Dodge",
    } )
  }
}
