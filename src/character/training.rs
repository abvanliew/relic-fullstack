use dioxus::prelude::*;

use crate::rule::prelude::*;

use super::{prelude::AttributeClass, resource::Flow};

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
  pub fn components( &self ) -> ( Option<AttributeClass>, Option<Flow> ) {
    return match self {
      TrainingClass::Expert => ( Some( AttributeClass::Expertise ), None ),
      TrainingClass::Adept => ( Some( AttributeClass::Capability ), None ),
      TrainingClass::Endurance => ( Some( AttributeClass::Defense ), None ),
      TrainingClass::Resonance => ( None, Some( Flow::Resonance ) ),
      TrainingClass::Innate => ( None, Some( Flow::Innate ) ),
      TrainingClass::Magic => ( None, Some( Flow::Magic ) ),
    }
  }
}

pub fn main_training_growth() -> TrainingGrowth {
  return TrainingGrowth {
    expert: vec![
      TrainingBonus { rank: Some( 1 ), ..Default::default() },
      TrainingBonus { rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), max_rank: Some( 1 ), ..Default::default() },
      TrainingBonus { rank: Some( 1 ), ..Default::default() },
      TrainingBonus { rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), max_rank: Some( 1 ), ..Default::default() },
    ],
    adept: vec![
      TrainingBonus { hp: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      TrainingBonus { max_rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      TrainingBonus { max_rank: Some( 1 ), ..Default::default() },
    ],
    endurance: vec![
      TrainingBonus { hp: Some( 2 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 2 ), max_rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 2 ), rank: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), con: Some( 1 ), max_rank: Some( 1 ), ..Default::default() },
    ],
    resonance: vec![
      TrainingBonus { pool: Some( 1 ), ..Default::default() },
      TrainingBonus { pool: Some( 2 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), flow: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
      TrainingBonus { pool: Some( 1 ), ..Default::default() },
      TrainingBonus { pool: Some( 2 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), flow: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
    ],
    innate: vec![
      TrainingBonus { hp: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), pool: Some( 2 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), pool_all: Some( 1 ), flow: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), pool: Some( 2 ), ..Default::default() },
      TrainingBonus { hp: Some( 1 ), pool_all: Some( 1 ), flow: Some( 1 ), ..Default::default() },
    ],
    magic: vec![
      TrainingBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      TrainingBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      TrainingBonus { flow: Some( 1 ), pool: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
      TrainingBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      TrainingBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      TrainingBonus { flow: Some( 1 ), pool: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
    ],
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingGrowth {
  pub expert: Vec<TrainingBonus>,
  pub adept: Vec<TrainingBonus>,
  pub endurance: Vec<TrainingBonus>,
  pub innate: Vec<TrainingBonus>,
  pub resonance: Vec<TrainingBonus>,
  pub magic: Vec<TrainingBonus>,
}

impl TrainingGrowth {
  pub fn bonus( &self, class: &TrainingClass, rank: i32 ) -> Option<TrainingBonus> {
    let Ok( rank_cast ): Result<usize, _> = rank.try_into() else { return None; };
    let list = match class {
      TrainingClass::Expert => &self.expert,
      TrainingClass::Adept => &self.adept,
      TrainingClass::Endurance => &self.endurance,
      TrainingClass::Resonance => &self.resonance,
      TrainingClass::Innate => &self.innate,
      TrainingClass::Magic => &self.magic,
    };
    let index = rank_cast - 1 % list.len();
    let Some( bonus ) = list.get( index ) else { return None; };
    return Some( bonus.clone() );
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TrainingBonus {
  pub hp: Option<i32>,
  pub con: Option<i32>,
  pub rank: Option<i32>,
  pub max_rank: Option<i32>,
  pub flow: Option<i32>,
  pub pool: Option<i32>,
  pub pool_all: Option<i32>,
}

#[component]
pub fn TrainingPanel( growth: TrainingGrowth, class: TrainingClass, rank: i32 ) -> Element {
  let Some( bonus ) = growth.bonus( &class, rank ) else { return rsx!(); };
  let ( attribute, flow ) = class.components();
  rsx!(
    if let Some( value ) = bonus.hp {
      div { Modifier { value } span { " HP. " } }
    }
    if let Some( value ) = bonus.con {
      div { Modifier { value } span { " constitution. " } }
    }
    if let Some( name ) = attribute {
      if let Some( value ) = bonus.rank {
        div { Modifier { value } span { " {name} ranks." } }
      }
      if let Some( value ) = bonus.max_rank {
        div { Modifier { value } span { " to maximum & current {name} rank (once per {name} per tier). " } }
      }
    }
    if let Some( name ) = flow {
      if let Some( value ) = bonus.flow {
        Modifier { value } span { " {name} Flow (maximum 8)." }
      }
      if let Some( value ) = bonus.pool {
        Modifier { value } span { " {name} Resource pool. " }
      }
      if let Some( value ) = bonus.pool_all {
        Modifier { value } span { " to all {name} Resource pools. " }
      }
    }
  )
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TrainingSignal {
  pub expert: Signal<i32>,
  pub adept: Signal<i32>,
  pub endurance: Signal<i32>,
  pub innate: Signal<i32>,
  pub resonance: Signal<i32>,
  pub magic: Signal<i32>,
}

impl TrainingSignal {
  pub fn use_context_provider()-> Self {
    let expert = use_signal( || 0 );
    let adept = use_signal( || 0 );
    let endurance = use_signal( || 0 );
    let resonance = use_signal( || 0 );
    let innate = use_signal( || 0 );
    let magic = use_signal( || 0 );
    use_context_provider( ||
      Self{ expert, adept, endurance, resonance, innate, magic }
    )
  }

  pub fn get( &self, class: &TrainingClass ) -> i32 {
    return match class {
      TrainingClass::Expert => (self.expert)(),
      TrainingClass::Adept => (self.adept)(),
      TrainingClass::Endurance => (self.endurance)(),
      TrainingClass::Innate => (self.innate)(),
      TrainingClass::Resonance => (self.resonance)(),
      TrainingClass::Magic => (self.magic)(),
    };
  }

  pub fn set( &mut self, class: &TrainingClass, value: i32 ) {
    match class {
      TrainingClass::Expert => self.expert.set( value ),
      TrainingClass::Adept => self.adept.set( value ),
      TrainingClass::Endurance => self.endurance.set( value ),
      TrainingClass::Innate => self.innate.set( value ),
      TrainingClass::Resonance => self.resonance.set( value ),
      TrainingClass::Magic => self.magic.set( value ),
    };
  }

  pub fn sum( &self ) -> i32 {
    return (self.expert)() + (self.adept)() + (self.endurance)() + (self.resonance)() + (self.innate)() + (self.magic)();
  }
}
