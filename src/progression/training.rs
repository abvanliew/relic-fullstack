use std::ops::{Add, AddAssign};

use dioxus::prelude::*;

use crate::operator::opt_sum;
use crate::progression::track::*;
use crate::rule::components::Modifier;
use crate::character::prelude::*;
use crate::progression::component::TrainingSignal;

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
  pub fn bonus( &self, class: TrainingClass, rank: usize ) -> Option<CharacterBonus> {
    let list = self.list( &class );
    let index = ( rank - 1 ) % list.len();
    let Some( bonus ) = list.get( index ) else { return None; };
    return Some( bonus.clone() );
  }

  pub fn modifiers( &self, signals: TrainingSignal ) -> TrainingModifiers {
    let mut mods = TrainingModifiers::default();
    for class in TrainingClass::ordered() {
      let ( rank_class, flow ) = class.components();
      let growths = self.list( &class );
      let rank = signals.get( &class );
      mods += training_modifiers( rank.into(), growths, rank_class, flow );
    }
    return mods;
  }

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
    mods.hp = opt_sum( mods.hp, x.hp );
    mods.con = opt_sum( mods.con, x.con );
    match rank_class {
      Some( RankClass::Capability ) => {
        mods.cap = opt_sum( mods.cap, x.rank );
        mods.cap_max = opt_sum( mods.cap_max, x.max_rank );
      },
      Some( RankClass::Defense ) => {
        mods.def = opt_sum( mods.def, x.rank );
        mods.def_max = opt_sum( mods.def_max, x.max_rank );
      },
      _ => (),
    };
    match flow {
      Some( Flow::Innate ) => {
        mods.innate_flow = opt_sum( mods.innate_flow, x.flow );
        mods.innate_pool = opt_sum( mods.innate_pool, x.pool );
        mods.innate_all = opt_sum( mods.innate_all, x.pool_all );
      },
      _ => (),
    };
  }
  return mods;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CharacterBonus {
  pub hp: Option<i32>,
  pub con: Option<i32>,
  pub rank: Option<usize>,
  pub max_rank: Option<usize>,
  pub flow: Option<i32>,
  pub pool: Option<i32>,
  pub pool_all: Option<i32>,
}

#[component]
pub fn TrainingPanel( class: TrainingClass, rank: usize ) -> Element {
  let growth = use_context::<TrackContext>().training;
  let Some( bonus ) = growth.bonus( class.clone(), rank ) else { return rsx!(); };
  let ( attribute, flow ) = class.components();
  rsx!(
    if let Some( value ) = bonus.hp {
      div { Modifier { value } span { " HP " } }
    }
    if let Some( value ) = bonus.con {
      div { Modifier { value } span { " Constitution " } }
    }
    if let Some( name ) = attribute {
      if let Some( value ) = bonus.rank {
        div { Modifier { value: value.try_into().unwrap_or( 0 ) } span { " {name} ranks" } }
      }
      if let Some( value ) = bonus.max_rank {
        div { Modifier { value: value.try_into().unwrap_or( 0 ) } span { " to max {name} rank (once / {name} / tier) " } }
      }
    }
    if let Some( name ) = flow {
      if let Some( value ) = bonus.flow {
        div { Modifier { value } span { " {name} Flow (max 8)" } }
      }
      if let Some( value ) = bonus.pool {
        div { Modifier { value } span { " {name} pool " } }
      }
      if let Some( value ) = bonus.pool_all {
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
