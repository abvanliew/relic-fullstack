// use std::cmp;
// use std::collections::{HashSet, HashMap};
// use dioxus::prelude::*;

// use crate::path::Path;
// use crate::path::components::PathTile;
// // use crate::progression::component::TrainingSignal;
// use crate::progression::prelude::*;
// use crate::rule::prelude::*;
// use crate::skill::Skill;
// use crate::skill::component::SkillTile;

// use crate::character;
// use crate::character::prelude::*;
// use crate::character::attribute::{AttributeSignal, Attribute::{self, Capability, Defense}};
// use crate::character::prelude::Capability::{Manipulation, Physique, Spirit, Warfare};
// use crate::character::prelude::Defense::{Dodge, Fortitude, Insight, Resolve, Tenacity};

// #[derive(Debug, Clone)]
// struct PathContext {
//   pub ids: Signal<HashSet<String>>
// }

// #[derive(Debug, Clone)]
// struct SkillContext {
//   pub ids: Signal<HashSet<String>>
// }

// #[derive(Debug, Clone, Default)]
// pub struct FlowResources {
//   pub flow: Bonus<u32>,
//   pub pools: HashMap<character::ResourcePool, Bonus<u32>>,
// }

// #[component]
// fn CharacterProgression( paths: Vec<Path> ) -> Element {
//   let full_features = vec![
//     FeatureTest { title: "HP +1".into(), summary: None },
//     FeatureTest { title: "+1 Attribute".into(), summary: None },
//   ];
//   let half_features = vec![
//     FeatureTest { title: "+1 Expertise".into(), summary: None },
//   ];
//   let mut level = use_signal( || 1 );
//   let mut minimize_paths = use_signal( || false );
//   let mut attrib_signal = AttributeSignal::use_context_provider();
//   let training_signals = TrainingSignal::use_context_provider();
//   let growth = training_growth_track();
//   let mut growth_modifiers: Signal<TrainingModifiers> = use_signal( || TrainingModifiers::default() );
//   // let stats = CharacterGrowth::default().stats( level() );
//   let stats = character_growth_track().stats( level() );
//   use_effect( move || attrib_signal.cap( stats.max_ranks.try_into().unwrap_or( 0 ) ) );
//   let path_ids_selected: Signal<HashSet<String>> = use_signal( || HashSet::new() );
//   use_context_provider( || PathContext { ids: path_ids_selected } );
//   let skill_ids_selected: Signal<HashSet<String>> = use_signal( || HashSet::new() );
//   use_context_provider( || SkillContext { ids: skill_ids_selected } );
//   let mut skills_full: Signal<Vec<Skill>> = use_signal( || Vec::new() );
//   let mut pool_modifiers: Signal<Vec<PoolModifier>> = use_signal( || Vec::new() );
//   let path_copy = paths.clone();
//   let mut flow_resources: Signal<HashMap<Flow,FlowResources>> = use_signal( || HashMap::new() );
//   use_effect( move || {
//     let mut skills: Vec<Skill> = Vec::new();
//     let mut modifiers: Vec<PoolModifier> = Vec::new();
//     let mut resources: HashMap<Flow,FlowResources> = HashMap::new();
//     for id in path_ids_selected() {
//       let Some( path ) = find_path( path_copy.clone(), id.clone() ) else { continue; };
//       skills.extend( path.skills_spells_full() );
//       modifiers.extend( path.resource_pool_modifiers() );
//     }
//     for modifier in modifiers.clone() {
//       let flow = modifier.resource.flow();
//       let entry = resources.entry( flow ).or_default();
//       entry.flow += modifier.flow;
//       let pool = entry.pools.entry( modifier.resource ).or_default();
//       *pool += modifier.pool;
//     }
//     skills.dedup_by_key(|p| p.id.to_string() );
//     skills_full.set( skills );
//     pool_modifiers.set( modifiers );
//     flow_resources.set( resources );
//     // growth_modifiers.set( growth.modifiers( training_signals ) );
//   } );
//   let path_count: i32 = path_ids_selected().len().try_into().unwrap_or( i32::MAX );
//   let full_count: i32 = skill_ids_selected().len().try_into().unwrap_or( i32::MAX );
//   let ( sum_cap, sum_def ) = attrib_signal.cap_def();
//   let remaining = stats.attributes - (sum_cap + sum_def).try_into().unwrap_or( 0 );
//   let path_min = stats.iniatite.path_min;
//   let path_max = stats.iniatite.path_max;
//   let path_optional = path_max - path_min;
//   let full_min = stats.iniatite.features - path_max + path_min;
//   let full_max = stats.iniatite.features;
//   let half_min = stats.iniatite.half_features;
//   let half_max = half_min + 2 * ( full_max );
//   let path_selected = path_count < path_max.try_into().unwrap_or( 0 );
//   let path_enabled = path_selected;
//   let has_innate = flow_resources().contains_key( &Flow::Innate );
//   let has_resonance = flow_resources().contains_key( &Flow::Resonance );
//   let has_magic = flow_resources().contains_key( &Flow::Magic );
//   let total_hp = stats.hp + (growth_modifiers().hp.unwrap_or( 0 )).try_into().unwrap_or( 0 );
//   let total_con = 4 + growth_modifiers().con.unwrap_or( 0 );
//   rsx! {
//     div {
//       class: "grid dim-keywords",
//       div { class: "uv-title", "Level" }
//       div {
//         class: "uv-after-title",
//         select {
//           onchange: move |event| { level.set( event.value().parse().unwrap_or( 1 ) ); },
//           for lvl in 1..=12 {
//             option { value: lvl, label: lvl, selected: level() == lvl, }
//           }
//         }
//       }
//     }
//     div {
//       class: "tiles",
//       div {
//         class: "uv-full small-text dotted-underline spacer-medium",
//         onclick: move |_| { minimize_paths.set( !minimize_paths() ); },
//         "Paths - Required {path_min}, Optional {path_optional} [{path_count}, {path_selected}, {minimize_paths}]"
//       }
//       for path in paths {
//         PathSelector { path, enabled: path_enabled }
//       }
//     }
//     TrainingRanks { level: level(), has_innate, has_resonance, has_magic }
//     div {
//       class: "tiles",
//       div {
//         class: "uv-full small-text dotted-underline spacer-medium",
//         "Full Features: {full_min} - {full_max} [{full_count}]"
//       }
//       for skill in skills_full() {
//         SkillSelector { skill }
//       }
//       for feature in full_features {
//         FeatureSelector { feature }
//       }
//       div {
//         class: "uv-full small-text dotted-underline spacer-medium",
//         "Half Features: {half_min} - {half_max}"
//       }
//       for feature in half_features {
//         FeatureSelector { feature }
//       }
//     }
//     div {
//       class: "grid dim-keywords",
//       AttributeDistribution {
//         remaining: remaining.try_into().unwrap_or( 0 ),
//         min: 0,
//         max: stats.max_ranks.try_into().unwrap_or( 0 ),
//         total: stats.attributes.try_into().unwrap_or( 0 )
//       }
//       div { class: "uv-full spacer-medium" }
//       div { class: "uv-title highlight", "HP" }
//       div { class: "uv-after-title", "{total_hp}" }
//       div { class: "uv-title highlight", "Con" }
//       div { class: "uv-after-title", "{total_con}" }
//       div { class: "uv-full", "{growth_modifiers:?}" }
//       ResourceFlowPools { resources: flow_resources }
//     }
//   }
// }

// fn find_path( paths: Vec<Path>, id: String ) -> Option<Path> {
//   for path in paths {
//     if path.id.to_string().eq( &id ) { return Some( path.clone() ) }
//   }
//   return None;
// }

// #[component]
// pub fn ResourceFlowPools( resources: ReadOnlySignal<HashMap<Flow,FlowResources>> ) -> Element {
//   rsx!(
//     for flow in Flow::ordered() {
//       if let Some( flow_resource ) = resources().get( &flow ) {
//         div { class: "uv-full spacer-medium" }
//         div { class: "uv-title highlight", "{flow} Flow" }
//         div { class: "uv-after-title highlight", "{flow_resource.flow}" }
//         for resource in character::ResourcePool::ordered() {
//           if let Some( pool ) = flow_resource.pools.get( &resource ) {
//             div { class: "uv-title", "{resource}" }
//             div { class: "uv-after-title", "{pool}" }
//           }
//         }
//       }
//     }
//   )
// }

// #[component]
// pub fn TrainingRanks( level: usize, has_innate: bool, has_resonance: bool, has_magic: bool ) -> Element {
//   let growth = training_growth_track();
//   let training = use_context::<TrainingSignal>();
//   let sum = training.sum();
//   let ranks_shown = 6 *( ( level + 5 ) / 6 );
//   rsx!(
//     div {
//       class: "grid dim-training",
//       div { class: "uv-full dotted-underline small-text spacer-medium", "Training Ranks: {level} [{sum}]" }
//       div { class: "uv-expert", "Expert" }
//       div { class: "uv-adept", "Adept" }
//       div { class: "uv-endurance", "Endurance" }
//       div { class: if has_innate { "uv-innate" } else { "uv-innate disabled" }, "Innate" }
//       div { class: if has_resonance { "uv-resonance" } else { "uv-resonance disabled" }, "Resonance" }
//       div { class: if has_magic { "uv-magic" } else { "uv-magic disabled" }, "Magic" }
//       for i in 1..=ranks_shown {
//         TrainingSelector {
//           class: TrainingClass::Expert,
//           rank: i.into(), growth: growth.clone(), level, available: true,
//         }
//         TrainingSelector {
//           class: TrainingClass::Adept,
//           rank: i.into(), growth: growth.clone(), level, available: true,
//         }
//         TrainingSelector {
//           class: TrainingClass::Endurance,
//           rank: i.into(), growth: growth.clone(), level, available: true,
//         }
//         TrainingSelector {
//           class: TrainingClass::Innate,
//           rank: i.into(), growth: growth.clone(), level,
//           available: has_innate,
//         }
//         TrainingSelector {
//           class: TrainingClass::Resonance,
//           rank: i.into(), growth: growth.clone(), level,
//           available: has_resonance,
//         }
//         TrainingSelector {
//           class: TrainingClass::Magic,
//           rank: i.into(), growth: growth.clone(), level,
//           available: has_magic,
//         }
//       }
//       if ranks_shown < 18 {
//         div { class: "uv-expert", "..." }
//         div { class: "uv-adept", "..." }
//         div { class: "uv-endurance", "..." }
//         div { class: "uv-resonance", "..." }
//         div { class: "uv-innate", "..." }
//         div { class: "uv-magic", "..." }
//       }
//     }
//   )
// }

// #[component]
// pub fn TrainingSelector( class: TrainingClass, rank: usize, growth: TrainingGrowth, level: usize, available: bool ) -> Element {
//   let mut training = use_context::<TrainingSignal>();
//   let selected_rank = training.get( &class );
//   let min: u32 = 0;
//   let max: u32 = (level.try_into().unwrap_or( 0 ) - training.sum() + selected_rank).into();
//   let top = selected_rank == rank.try_into().unwrap_or( 0 );
//   let uv = match class {
//     TrainingClass::Expert => "uv-expert",
//     TrainingClass::Adept => "uv-adept",
//     TrainingClass::Endurance => "uv-endurance",
//     TrainingClass::Resonance => "uv-resonance",
//     TrainingClass::Innate => "uv-innate",
//     TrainingClass::Magic => "uv-magic",
//   };
//   let selected = rank <= selected_rank.try_into().unwrap_or( 0 );
//   let enabled = available && rank.try_into().unwrap_or( 0 ) <= max;
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
//         training.set( &class, new_value.try_into().unwrap_or( 0 ) )
//       },
//       TrainingPanel { class: class.clone(), rank: rank.try_into().unwrap_or( 0 ) }
//     }
//   )
// }

// #[component]
// pub fn AttributeDistribution( remaining: i32, min: i32, max: i32, total: i32 ) -> Element {
//   rsx!(
//     div { class: "uv-title", "Remaining" }
//     div { class: "uv-after-title", "{remaining}" }
//     div { class: "uv-full highlight spacer-medium", "Capabilities" }
//     AttributeSelector { attribute: Capability(Physique), min, max, total }
//     AttributeSelector { attribute: Capability(Warfare), min, max, total }
//     AttributeSelector { attribute: Capability(Spirit), min, max, total }
//     AttributeSelector { attribute: Capability(Manipulation), min, max, total }
//     div { class: "uv-full highlight spacer-medium", "Defenses" }
//     AttributeSelector { attribute: Defense(Tenacity), min, max, total }
//     AttributeSelector { attribute: Defense(Fortitude), min, max, total }
//     AttributeSelector { attribute: Defense(Resolve), min, max, total }
//     AttributeSelector { attribute: Defense(Insight), min, max, total }
//     AttributeSelector { attribute: Defense(Dodge), min, max, total }
//   )
// }

// #[component]
// pub fn AttributeSelector( attribute: Attribute, min: i32, max: i32, total: i32 ) -> Element {
//   let attributes = use_context::<AttributeSignal>();
//   let mut signal = attributes.get( &attribute );
//   let cur_max = cmp::min( max, total - attributes.sum() + signal() );
//   rsx!(
//     div { class: "uv-title", "{attribute}" }
//     div {
//       class: "uv-after-title",
//       input {
//         type: "number",
//         min: min,
//         max: cur_max,
//         value: signal(),
//         onchange: move |event| {
//           let mut value = event.value().parse().unwrap_or( min );
//           if value > cur_max { value = cur_max; }
//           if value < min { value = min; }
//           signal.set( value );
//         }
//       }
//     }
//   )
// }

// #[component]
// pub fn PathSelector( path: Path, enabled: bool ) -> Element {
//   let paths = use_context::<PathContext>();
//   let id = path.id.to_string();
//   let mut signal = paths.ids;
//   let mut selected = use_signal( || false );
//   rsx!(
//     div {
//       class: match ( selected(), enabled ) {
//         ( true, true ) => "tile selected",
//         ( true, false ) => "tile selected disabled",
//         ( false, true ) => "tile unselected",
//         ( false, false ) => "tile hidden",
//       },
//       onclick: move |_| {
//         if !enabled && !selected() { return; }
//         let mut updated_paths = signal().clone();
//         match selected() {
//           true => updated_paths.remove( &id ),
//           false => updated_paths.insert( id.to_owned() ),
//         };
//         signal.set( updated_paths );
//         selected.set( !selected() );
//       },
//       PathTile { path }
//     }
//   )
// }

// #[component]
// pub fn SkillSelector( skill: Skill ) -> Element {
//   let skills = use_context::<SkillContext>();
//   let id = skill.id.to_string();
//   let mut skill_list = skills.ids;
//   let mut selected = use_signal( || false );
//   rsx!(
//     div {
//       class: match selected() {
//         true => "tile selected",
//         false => "tile unselected",
//       },
//       onclick: move |_| {
//         let mut value = skill_list().clone();
//         match selected() {
//           true => value.remove( &id ),
//           false => value.insert( id.to_owned() ),
//         };
//         skill_list.set( value );
//         selected.set( !selected() );
//       },
//       SkillTile { skill }
//     }
//   )
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct FeatureTest {
//   pub title: String,
//   pub summary: Option<String>,
// }

// #[component]
// pub fn FeatureSelector( feature: FeatureTest ) -> Element {
//   let min = 0;
//   let max = 5;
//   let mut count: Signal<i32> = use_signal( || min );
//   rsx!(
//     div {
//       class: if count() > 0 { "tile selected" } else { "tile unselected" },
//       onclick: move |_| { if count() < max { count.set( count() + 1 ); } },
//       div { "{feature.title}" }
//       div {
//         input {
//           type: "number",
//           min: min,
//           max: max,
//           value: count(),
//           onclick: move |event| { event.stop_propagation(); },
//           onchange: move |event| {
//             let mut value = event.value().parse().unwrap_or( min );
//             if value > max { value = max; }
//             if value < min { value = min; }
//             count.set( value );
//           }
//         }
//       }
//       div {
//         onclick: move |event| {
//           event.stop_propagation();
//           count.set( min );
//         },
//         "Clear"
//       }
//     }
//   )
// }
