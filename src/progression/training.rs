use std::ops::{Add, AddAssign};

use dioxus::prelude::*;
use std::fmt;

use crate::operator::{opt_sum, InstanceBonus, StackingBonus};
use crate::progression::track::*;
use crate::rule::components::Modifier;
use crate::character::prelude::*;
// use crate::progression::component::TrainingSignal;

#[derive(Debug, Clone, PartialEq)]
pub enum TrainingClass {
  Expert,
  Adept,
  Endurance,
  Innate,
  Resonance,
  Magic,
}

impl TrainingClass {
  pub fn ordered() -> [TrainingClass; 6] { [
    TrainingClass::Expert, TrainingClass::Adept, TrainingClass::Endurance,
    TrainingClass::Innate, TrainingClass::Resonance, TrainingClass::Magic,
  ] }

  pub fn components( &self ) -> ( Option<RankClass>, Option<Flow> ) {
    return match self {
      TrainingClass::Expert => ( Some( RankClass::Expertise ), None ),
      TrainingClass::Adept => ( Some( RankClass::Capability ), None ),
      TrainingClass::Endurance => ( Some( RankClass::Defense ), None ),
      TrainingClass::Innate => ( None, Some( Flow::Innate ) ),
      TrainingClass::Resonance => ( None, Some( Flow::Resonance ) ),
      TrainingClass::Magic => ( None, Some( Flow::Magic ) ),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingGrowth {
  pub expert: Vec<CharacterBonus>,
  pub adept: Vec<CharacterBonus>,
  pub endurance: Vec<CharacterBonus>,
  pub innate: Vec<CharacterBonus>,
  pub resonance: Vec<CharacterBonus>,
  pub magic: Vec<CharacterBonus>,
}

impl TrainingGrowth {
  fn list( &self, class: &TrainingClass ) -> &Vec<CharacterBonus> {
    return match class {
      TrainingClass::Expert => &self.expert,
      TrainingClass::Adept => &self.adept,
      TrainingClass::Endurance => &self.endurance,
      TrainingClass::Innate => &self.innate,
      TrainingClass::Resonance => &self.resonance,
      TrainingClass::Magic => &self.magic,
    };
  }

  pub fn bonus( &self, class: &TrainingClass, rank: usize ) -> Option<CharacterBonus> {
    let list = self.list( class );
    let index = ( rank - 1 ) % list.len();
    let Some( bonus ) = list.get( index ) else { return None; };
    return Some( bonus.clone() );
  }

  pub fn total_bonus( &self, class: &TrainingClass, rank: usize ) -> CharacterBonus {
    let mut total = CharacterBonus::default();
    for r in 1..=rank {
      let Some(bonus) = self.bonus(class, r) else { continue; };
      total += bonus;
    }
    return total;
  }

  // pub fn modifiers( &self, signals: TrainingSignal ) -> TrainingModifiers {
  //   let mut mods = TrainingModifiers::default();
  //   for class in TrainingClass::ordered() {
  //     let ( rank_class, flow ) = class.components();
  //     let growths = self.list( &class );
  //     let rank = signals.get( &class );
  //     mods += training_modifiers( rank.into(), growths, rank_class, flow );
  //   }
  //   return mods;
  // }
}

pub fn training_modifiers(
  rank: u32,
  growths: &Vec<CharacterBonus>,
  rank_class: Option<RankClass>,
  flow: Option<Flow>
) -> TrainingModifiers {
  let mut mods = TrainingModifiers::default();
  let index: usize = rank.try_into().unwrap_or( growths.len() );
  for i in 0..index {
    let x = &growths[i];
    mods.hp = opt_sum( mods.hp, x.hp.as_opt() );
    mods.con = opt_sum( mods.con, x.con.as_opt() );
    match rank_class {
      Some( RankClass::Capability ) => {
        mods.cap = opt_sum( mods.cap, x.rank.as_opt() );
        mods.cap_max = opt_sum( mods.cap_max, x.max_rank.as_opt() );
      },
      Some( RankClass::Defense ) => {
        mods.def = opt_sum( mods.def, x.rank.as_opt() );
        mods.def_max = opt_sum( mods.def_max, x.max_rank.as_opt() );
      },
      _ => (),
    };
    match flow {
      Some( Flow::Innate ) => {
        mods.innate_flow = opt_sum( mods.innate_flow, x.flow.as_opt() );
        mods.innate_pool = opt_sum( mods.innate_pool, x.pool.as_opt() );
        mods.innate_all = opt_sum( mods.innate_all, x.pool_all.as_opt() );
      },
      _ => (),
    };
  }
  return mods;
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct RankBonus {
  pub rank: StackingBonus<usize>,
  pub max_rank: StackingBonus<usize>,
}

impl RankBonus {
  pub fn description_elements(&self, name: String) -> Vec<String> {
    let mut elements: Vec<String> = Vec::new();
    if let Some( rank ) = self.rank.as_opt() {
      elements.push(format!("+{rank} {name} ranks"));
    }
    if let Some( max_rank ) = self.max_rank.as_opt() {
      elements.push(format!("Increase {max_rank} {name}'s maxmimum and current rank by 1"));
    }
    return elements;
  }
}

impl Add for RankBonus {
  type Output = RankBonus;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      rank: self.rank + rhs.rank,
      max_rank: self.max_rank + rhs.max_rank,
    }
  }
}

impl AddAssign for RankBonus {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct FlowBonus {
  pub flow_base: InstanceBonus<usize>,
  pub flow: StackingBonus<usize>,
  pub pool: StackingBonus<usize>,
  pub pool_all: StackingBonus<usize>,
}

impl FlowBonus {
  pub fn description_elements(&self, name: String) -> Vec<String> {
    let mut elements: Vec<String> = Vec::new();
    if let Some( flow ) = self.flow.as_opt() {
      elements.push(format!("+{flow} to {name} Flow"));
    }
    if let Some( pool_magic ) = self.pool.as_opt() {
      elements.push(format!("+{pool_magic} {name} resource pool size"));
    }
    if let Some( pool_all_magic ) = self.pool_all.as_opt() {
      elements.push(format!("+{pool_all_magic} to all {name} resource pools size"));
    }
    return elements;
  }
}

impl Add for FlowBonus {
  type Output = FlowBonus;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      flow_base: self.flow_base + rhs.flow_base,
      flow:self.flow + rhs.flow,
      pool: self.pool + rhs.pool,
      pool_all: self.pool_all + rhs.pool_all,
    }
  }
}

impl AddAssign for FlowBonus {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CharacterBonus {
  pub hp: StackingBonus<i32>,
  pub con: StackingBonus<i32>,

  pub rank: StackingBonus<usize>,
  pub max_rank: StackingBonus<usize>,

  pub capability: RankBonus,
  pub defense: RankBonus,
  pub expertise: RankBonus,
  pub flow: StackingBonus<i32>,
  pub pool: StackingBonus<i32>,
  pub pool_all: StackingBonus<i32>,

  pub innate: FlowBonus,
  pub resonance: FlowBonus,
  pub magic: FlowBonus,
}

impl Add for CharacterBonus {
  type Output = CharacterBonus;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      hp: self.hp + rhs.hp,
      con:  self.con + rhs.con,
      rank: self.rank + rhs.rank,
      max_rank: self.max_rank + rhs.max_rank,
      capability: self.capability + rhs.capability,
      defense: self.defense + rhs.defense,
      expertise: self.expertise + rhs.expertise,
      flow: self.flow + rhs.flow,
      pool: self.pool + rhs.pool,
      pool_all: self.pool_all + rhs.pool_all,
      innate: self.innate + rhs.innate,
      resonance: self.resonance + rhs.resonance,
      magic: self.magic + rhs.magic,
    }
  }
}

impl AddAssign for CharacterBonus {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

impl fmt::Display for CharacterBonus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut elements: Vec<String> = Vec::new();
    if let Some( hp ) = self.hp.as_opt() {
      elements.push(format!("+{hp} Hit Points"));
    }
    if let Some( con ) = self.con.as_opt() {
      elements.push(format!("+{con} Constitution"));
    }
    elements.append( &mut self.capability.description_elements("Capability".into()));
    elements.append( &mut self.defense.description_elements("Defense".into()));
    elements.append( &mut self.expertise.description_elements("Expertise".into()));
    elements.append( &mut self.innate.description_elements("Innate".into()));
    elements.append( &mut self.resonance.description_elements("Resonance".into()));
    elements.append( &mut self.magic.description_elements("Magic".into()));
    write!(f, "{}.", elements.join( ", "))
  }
}


#[component]
pub fn TrainingPanel( class: TrainingClass, rank: usize ) -> Element {
  let growth = use_context::<TrackContext>().training;
  let Some( bonus ) = growth.bonus( &class, rank ) else { return rsx!(); };
  let ( attribute, flow ) = class.components();
  rsx!(
    if let Some( value ) = bonus.hp.as_opt() {
      div { Modifier { value } span { " HP " } }
    }
    if let Some( value ) = bonus.con.as_opt() {
      div { Modifier { value } span { " Constitution " } }
    }
    if let Some( name ) = attribute {
      if let Some( value ) = bonus.rank.as_opt() {
        div { Modifier { value: value.try_into().unwrap_or( 0 ) } span { " {name} ranks" } }
      }
      if let Some( value ) = bonus.max_rank.as_opt() {
        div { Modifier { value: value.try_into().unwrap_or( 0 ) } span { " to max {name} rank (once / {name} / tier) " } }
      }
    }
    if let Some( name ) = flow {
      if let Some( value ) = bonus.flow.as_opt() {
        div { Modifier { value } span { " {name} Flow (max 8)" } }
      }
      if let Some( value ) = bonus.pool.as_opt() {
        div { Modifier { value } span { " {name} pool " } }
      }
      if let Some( value ) = bonus.pool_all.as_opt() {
        div { Modifier { value } span { " to all {name} pools " } }
      }
    }
  )
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TrainingModifiers {
  pub hp: Option<i32>,
  pub con: Option<i32>,
  pub cap: Option<usize>,
  pub cap_max: Option<usize>,
  pub def: Option<usize>,
  pub def_max: Option<usize>,
  pub expertise: Option<usize>,
  pub expertise_max: Option<usize>,
  pub innate_flow: Option<i32>,
  pub innate_pool: Option<i32>,
  pub innate_all: Option<i32>
}

impl Add for TrainingModifiers {
  type Output = TrainingModifiers;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      hp: opt_sum( self.hp, rhs.hp ),
      con: opt_sum( self.con, rhs.con ),
      cap: opt_sum( self.cap, rhs.cap ),
      cap_max: opt_sum( self.cap_max, rhs.cap_max ),
      def: opt_sum( self.def, rhs.def ),
      def_max: opt_sum( self.def_max, rhs.def_max ),
      expertise: opt_sum( self.expertise, rhs.expertise ),
      expertise_max: opt_sum( self.expertise_max, rhs.expertise_max ),
      innate_flow: opt_sum( self.innate_flow, rhs.innate_flow ),
      innate_pool: opt_sum( self.innate_pool, rhs.innate_pool ),
      innate_all: opt_sum( self.innate_all, rhs.innate_all ),
    }
  }
}

impl AddAssign for TrainingModifiers {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}
