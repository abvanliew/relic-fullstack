use dioxus::prelude::*;

use crate::character::prelude::Character;
use crate::character::component::*;
use crate::server::prelude::list_creatures;

#[component]
pub fn CharacterSheet() -> Element {
  let response: Resource<Result<Vec<Character>, ServerFnError>> = use_resource( move || list_creatures() );
  match &*response.read_unchecked() {
    Some( Ok( characters ) ) => { rsx! {
      for character in characters {
        CharacterDetails { character: character.to_owned() }
      }
    } }
    Some( Err( err ) ) => {
      rsx! { "An error occurred when loading sheet: {err}" }
    }
    None => { rsx! { "Loading sheet" } }
  }
}
