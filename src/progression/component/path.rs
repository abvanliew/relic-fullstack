use dioxus::prelude::*;

use crate::{path::{components::PathTile, Path}, progression::component::builder::BuildContext};

#[component]
pub fn PathSelections( paths: Vec<Path> ) -> Element {
  let ( _inherient, selectable ): (Vec<Path>, Vec<Path>) = paths.clone().into_iter()
  .partition( |p| match p.inherient { Some( true ) => true, _ => false } );
  rsx! {
    div {
      class: "tiles",
      div {
        class: "uv-full small-text dotted-underline spacer",
        "Inherient Paths"
      }
      for path in selectable {
        PathSelector { path }
      }
    }
  }
}

#[component]
pub fn PathSelector( path: Path ) -> Element {
  let build = use_context::<BuildContext>();
  let mut current = build.current_paths();
  let previous = build.previous_paths;
  let stats = build.level_stats;
  let id = path.id.to_string();
  let current_selected = current().contains( &id );
  let previous_selected = previous().contains( &id );
  let total = current().len() + previous().len();
  let disabled: bool = total >= stats().iniatite.path_max.try_into().unwrap();
  rsx!(
    div {
      class: match ( current_selected, previous_selected, disabled ) {
        ( _, true, _ ) => "tile prev-selected disabled",
        ( true, _, false ) => "tile selected",
        ( true, _, true ) => "tile selected disabled",
        ( _, _, true ) => "tile unselected disabled",
        _ => "tile unselected",
      },
      onclick: move |_| {
        if previous_selected || ( disabled && !current_selected ) { return; }
        let mut updated_paths = current().clone();
        match current_selected {
          true => updated_paths.remove( &id ),
          false => updated_paths.insert( id.to_owned() ),
        };
        current.set( updated_paths );
      },
      PathTile { path }
    }
  )
}
