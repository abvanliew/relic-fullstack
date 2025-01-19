use dioxus::prelude::*;

use crate::progression::prelude::*;
use crate::progression::component::*;

#[component]
pub fn TrainingSelector( class: TrainingClass, rank: usize ) -> Element {
  let level = use_context::<LevelContext>().level;
  // let mut training = use_context::<TrainingSignal>();
  // let selected_rank = training.get( &class );
  // let min: i32 = 0;
  // let max: i32 = level.try_into().unwrap_or( 0 ) - training.sum() + selected_rank;
  // let top = selected_rank == rank.try_into().unwrap_or( 0 );
  // let uv = match class {
  //   TrainingClass::Expert => "uv-expert",
  //   TrainingClass::Adept => "uv-adept",
  //   TrainingClass::Endurance => "uv-endurance",
  //   TrainingClass::Resonance => "uv-resonance",
  //   TrainingClass::Innate => "uv-innate",
  //   TrainingClass::Magic => "uv-magic",
  // };
  // let selected = rank <= selected_rank.try_into().unwrap_or( 0 );
  // let enabled = available && rank.try_into().unwrap_or( 0 ) <= max;
  rsx!(
    div {
      // class: match ( selected, enabled ) {
      //   ( true, true ) => format!( "{uv} small-text selected padded" ),
      //   ( true, false ) => format!( "{uv} small-text selected disabled padded" ),
      //   ( false, true ) => format!( "{uv} small-text unselected padded" ),
      //   ( false, false ) => format!( "{uv} small-text unselected disabled padded" ),
      // },
      // onclick: move |_| {
      //   let mut new_value = match ( selected, enabled, top ) {
      //     ( true, true, true ) => rank - 1,
      //     ( _, true, _ ) => rank,
      //     ( true, false, _ ) => max.try_into().unwrap_or( 0 ),
      //     _ => selected_rank.try_into().unwrap_or( 0 ),
      //   };
      //   if new_value < min.try_into().unwrap_or( 0 ) { new_value = min.try_into().unwrap_or( 0 ); }
      //   training.set( &class, new_value.try_into().unwrap_or( 0 ) )
      // },
      TrainingPanel { class: class.clone(), rank }
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
