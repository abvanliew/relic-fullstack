use std::cmp;

use dioxus::prelude::*;

use crate::{character::{attribute::{AttributeSignal, AttributeType::{self, Capability, Defense}}, build::{Selection, SelectionClass}}, path::components::PathTile};

use crate::path::Path;
use crate::character::progression::CharacterGrowth;
use crate::character::prelude::{Capability::{Manipulation,Physique,Spirit,Warfare},Defense::{Dodge,Fortitude,Insight,Resolve,Tenacity}};

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  // let mut path_id = use_signal( || "".to_string() );
  let mut level = use_signal( || 1 );
  let mut attrib_signal = AttributeSignal::use_context_provider();
  let stats = CharacterGrowth::default().stats( level() );
  use_effect( move || attrib_signal.cap( stats.max_ranks ) );
  let remaining = stats.attributes - attrib_signal.sum();
  rsx! {
    div {
      class: "grid dim-keywords",
      div { class: "uv-title", "Level" }
      div {
        class: "uv-after-title",
        select {
          onchange: move |event| {
            level.set( event.value().parse().unwrap_or( 1 ) );
          },
          for lvl in 1..=12 {
            option { value: lvl, label: lvl, selected: level() == lvl, }
          }
        }
      }
      div { class: "uv-title", "Remaining" }
      div { class: "uv-after-title", "{remaining}" }
      div { class: "uv-title highlight spacer", "Capabilities" }
      AttributeSelector {
        attribute: Capability(Physique), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Capability(Warfare), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Capability(Spirit), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Capability(Manipulation), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      div { class: "uv-title highlight spacer", "Defenses" }
      AttributeSelector {
        attribute: Defense(Tenacity), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Defense(Fortitude), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Defense(Resolve), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Defense(Insight), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      AttributeSelector {
        attribute: Defense(Dodge), min: 0, max: stats.max_ranks, total: stats.attributes,
      }
      // div { class: "uv-full highlight spacer", }
      // div { class: "uv-title highlight", "Path" }
      // div {
      //   class: "uv-after-title",
      //   select {
      //     onchange: move |event| {
      //       path_id.set( event.value() );
      //     },
      //     option { selected: true, hidden: true, }
      //     for path in paths {
      //       option {
      //         value: path.id.to_string(),
      //         label: path.title,
      //         selected: path.id.to_string() == path_id.to_string(),
      //         onchange: move |_| {}
      //       }
      //     }
      //   }
      // }
    }
    div {
      class: "tiles",
      div {
        class: "uv-full small-text under-border",
        "Paths"
      }
      for path in paths {
        div {
          class: "tile",
          PathTile { path }
        }
      }
      div { class: "tile" }
      div { class: "tile" }
      div { class: "tile" }
      div { class: "tile" }
    }
  }
}

#[component]
pub fn AttributeSelector( attribute: AttributeType, min: i32, max: i32, total: i32 ) -> Element {
  let attributes = use_context::<AttributeSignal>();
  let mut signal = attributes.get( &attribute );
  let cur_max = cmp::min( max, total - attributes.sum() + signal() );
  rsx!(
    div { class: "uv-title", "{attribute}" }
    div {
      class: "uv-after-title",
      input {
        type: "number",
        min: min,
        max: cur_max,
        value: signal(),
        onchange: move |event| {
          let mut value = event.value().parse().unwrap_or( min );
          if value > cur_max { value = cur_max; }
          if value < min { value = min; }
          signal.set( value );
        }
      }
    }
  )
}

// #[component]
// fn Selector( restriction: SelectionRestriction ) -> Element {
//   let mut selection_id = use_signal( || "".to_string() );
//   rsx!(
//     match restriction {
//       SelectionRestriction::PathOnly => rsx!( div { class: "uv-title highlight", "Path" } ),
//       SelectionRestriction::FeatureOnly => rsx!( div { class: "uv-title highlight", "Feature" } ),
//       SelectionRestriction::PathFeature => rsx!( div {
//         class: "uv-title",
//         select {
//           onchange: move |event| { selection_id.set( event.value() ); },
//           option { selected: true, hidden: true, }
//           option {
//             value: "Path", label: "Path",
//             selected: "Path" == selection_id.to_string(),
//           }
//           option {
//             value: "Feature", label: "Feature",
//             selected: "Feature" == selection_id.to_string(),
//           }
//         }
//       } )
//     }
//   )
// }

// fn selection_stats( selections: &Vec<Selection> ) -> SelectionStats {
//   let mut stats = SelectionStats::default();
//   for selection in selections {
//     match ( &selection.class, selection.id ) {
//       ( Some( SelectionClass::Path ), Some( _ ) ) => stats.paths += 1,
//       ( Some( SelectionClass::FullFeature ), Some( _ ) ) => stats.features += 1,
//       ( Some( SelectionClass::HalfFeature ), Some( _ ) ) => stats.half_features += 1,
//       _ => ()
//     }
//   }
//   return stats;
// }

// #[derive(Default)]
// struct SelectionStats {
//   pub paths: i32,
//   pub features: i32,
//   pub half_features: i32,
// }