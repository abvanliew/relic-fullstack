use dioxus::prelude::*;

// use crate::progression::component::builder::BuildContext;
// use crate::progression::component::qualifiers::BuildQualifiers;
use crate::progression::track::TrackContext;
use crate::rule::prelude::Tier;
use crate::path::{ Path};

#[component]
pub fn PathSelections( paths: Vec<Path>, tier: Tier, level: ReadOnlySignal<usize> ) -> Element {
  let ( _inherient, selectable ): (Vec<Path>, Vec<Path>) = paths.clone().into_iter()
  .partition( |p|  p.inherient.unwrap_or( false ) );
  let track = use_context::<TrackContext>();
  let stats = use_memo( move || track.character.stats( level() ) );
  rsx! {
    div { "PathSelections {stats:?}" }
    div { "Level {level}" }
    div {
      class: "tiles",
      div {
        class: "uv-full small-text dotted-underline spacer-medium",
        "{tier} Paths"
      }
      for path in selectable {
        PathSelector { path }
      }
    }
  }
}

#[component]
pub fn PathSelector( path: Path ) -> Element {
  // let build = use_context::<BuildContext>();
  // let stats = BuildQualifiers::use_memo_stats();
  // let mut current = build.paths;
  // let id = path.id.to_string();
  // let current_selected = current().contains( &id );
  // let previous_selected: bool = false;
  // let disabled: bool = current().len() >= stats().iniatite.path_max;
  rsx!(
    // div {
    //   class: match ( current_selected, previous_selected, disabled ) {
    //     ( _, true, _ ) => "tile prev-selected disabled",
    //     ( true, _, false ) => "tile selected",
    //     ( true, _, true ) => "tile selected disabled",
    //     ( _, _, true ) => "tile unselected disabled",
    //     _ => "tile unselected",
    //   },
    //   onclick: move |_| {
    //     if previous_selected || ( disabled && !current_selected ) { return; }
    //     let mut updated_paths = current().clone();
    //     match current_selected {
    //       true => updated_paths.remove( &id ),
    //       false => updated_paths.insert( id.to_owned() ),
    //     };
    //     current.set( updated_paths );
    //   },
    //   PathTile { path }
    // }
  )
}
