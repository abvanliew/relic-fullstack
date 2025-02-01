use builder::BuildContext;
use dioxus::prelude::*;

use crate::progression::prelude::*;
use crate::progression::component::*;

// #[component]
// pub fn TrainingRanks() -> Element {
//   let level = use_context::<LevelContext>().level;
//   let build = use_context::<BuildContext>();
//   let ranks_shown = 6 *( ( level() + 5 ) / 6 );
//   rsx!(
//     div {
//       class: "grid dim-training",
//       div { class: "uv-full dotted-underline small-text spacer", "Training Ranks: {level}" }
//       div { class: "uv-expert", "Expert" }
//       div { class: "uv-adept", "Adept" }
//       div { class: "uv-endurance", "Endurance" }
//       div { class: if build.has_innate() { "uv-innate" } else { "uv-innate disabled" }, "Innate" }
//       div { class: if build.has_resonance() { "uv-resonance" } else { "uv-resonance disabled" }, "Resonance" }
//       div { class: if build.has_magic() { "uv-magic" } else { "uv-magic disabled" }, "Magic" }
//       for rank in 1..=ranks_shown {
//         for class in TrainingClass::ordered() {
//           TrainingSelector { class, rank, }
//         }
//       }
//       if ranks_shown < 18 {
//         div { class: "uv-expert", "..." }
//         div { class: "uv-adept", "..." }
//         div { class: "uv-endurance", "..." }
//         div { class: if build.has_innate() { "uv-innate" } else { "uv-innate disabled" }, "..." }
//         div { class: if build.has_resonance() { "uv-resonance" } else { "uv-resonance disabled" }, "..." }
//         div { class: if build.has_magic() { "uv-magic" } else { "uv-magic disabled" }, "..." }
//       }
//     }
//   )
// }

// #[component]
// pub fn TrainingSelector( class: TrainingClass, rank: usize ) -> Element {
//   let build = use_context::<BuildContext>();
//   let level = use_context::<LevelContext>().level;
//   let sum = build.total_training();
//   let selected_rank = build.get_training_rank( &class );
//   let min: u32 = 0;
//   let max: u32 = level().try_into().unwrap_or( 0 ) - sum + selected_rank;
//   let top = selected_rank == rank.try_into().unwrap_or( 0 );
//   let selected = rank <= selected_rank.try_into().unwrap_or( 0 );
//   let enabled = build.has_training( &class ) && rank.try_into().unwrap_or( 0 ) <= max;
//   let uv = match &class {
//     TrainingClass::Expert => "uv-expert",
//     TrainingClass::Adept => "uv-adept",
//     TrainingClass::Endurance => "uv-endurance",
//     TrainingClass::Resonance => "uv-resonance",
//     TrainingClass::Innate => "uv-innate",
//     TrainingClass::Magic => "uv-magic",
//   };
//   rsx!(
//     div {
//       class: match ( selected, enabled ) {
//         ( true, true ) => format!( "{uv} small-text selected padded" ),
//         ( true, false ) => format!( "{uv} small-text selected disabled padded" ),
//         ( false, true ) => format!( "{uv} small-text unselected padded" ),
//         ( false, false ) => format!( "{uv} small-text unselected disabled padded" ),
//       },
//       onclick: move |_| {
//         let mut new_value = match ( selected, enabled, top ) {
//           ( true, true, true ) => rank - 1,
//           ( _, true, _ ) => rank,
//           ( true, false, _ ) => max.try_into().unwrap_or( 0 ),
//           _ => selected_rank.try_into().unwrap_or( 0 ),
//         };
//         if new_value < min.try_into().unwrap_or( 0 ) { new_value = min.try_into().unwrap_or( 0 ); }
//         build.current_training().set( &class, new_value.try_into().unwrap_or( 0 ) )
//       },
//       TrainingPanel { class: class.clone(), rank }
//     }
//   )
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct TrainingSignal {
//   pub expert: Signal<u32>,
//   pub adept: Signal<u32>,
//   pub endurance: Signal<u32>,
//   pub innate: Signal<u32>,
//   pub resonance: Signal<u32>,
//   pub magic: Signal<u32>,
// }

// impl TrainingSignal {
//   pub fn use_context_provider() -> Self {
//     return use_context_provider( || TrainingSignal::new_signal() );
//   }

//   pub fn new_signal()-> Self {
//     let expert = use_signal( || 0 );
//     let adept = use_signal( || 0 );
//     let endurance = use_signal( || 0 );
//     let resonance = use_signal( || 0 );
//     let innate = use_signal( || 0 );
//     let magic = use_signal( || 0 );
//     return Self{ expert, adept, endurance, resonance, innate, magic }
//   }

//   pub fn get( &self, class: &TrainingClass ) -> u32 {
//     return match class {
//       TrainingClass::Expert => (self.expert)(),
//       TrainingClass::Adept => (self.adept)(),
//       TrainingClass::Endurance => (self.endurance)(),
//       TrainingClass::Innate => (self.innate)(),
//       TrainingClass::Resonance => (self.resonance)(),
//       TrainingClass::Magic => (self.magic)(),
//     };
//   }

//   pub fn set( &mut self, class: &TrainingClass, value: u32 ) {
//     match class {
//       TrainingClass::Expert => self.expert.set( value ),
//       TrainingClass::Adept => self.adept.set( value ),
//       TrainingClass::Endurance => self.endurance.set( value ),
//       TrainingClass::Innate => self.innate.set( value ),
//       TrainingClass::Resonance => self.resonance.set( value ),
//       TrainingClass::Magic => self.magic.set( value ),
//     };
//   }

//   pub fn set_from( &mut self, values: [u32; 6] ) {
//     for i in 0..6 {
//       match i {
//         0 => self.expert.set( values[i] ),
//         1 => self.adept.set( values[i] ),
//         2 => self.endurance.set( values[i] ),
//         3 => self.innate.set( values[i] ),
//         4 => self.resonance.set( values[i] ),
//         5 => self.magic.set( values[i] ),
//         _ => ()
//       }
//     }
//   }

//   pub fn sum( &self ) -> u32 {
//     return (self.expert)() + (self.adept)() + (self.endurance)() + (self.resonance)() + (self.innate)() + (self.magic)();
//   }
// }
