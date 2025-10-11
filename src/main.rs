mod asset;
mod character;
mod equipment;
mod operator;
mod panels;
mod path;
mod progression;
mod rule;
mod server;
mod skill;

use dioxus::prelude::*;
use panels::*;
use server::prelude::GameLibrarySignal;

const FAVICON: Asset = asset!("assets/favicon.ico");
const MAIN_CSS: Asset = asset!("assets/main.css");

fn main() {
  dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
  #[layout(Navbar)]
  #[route("/")]
  Home {},
  #[nest("/skills")]
    #[route("/")]
    SkillList {},
    #[route("/:id")]
    SingleSkill { id: String },
  #[end_nest]
  #[nest("/paths")]
    #[route("/")]
    PathList {},
  #[end_nest]
  #[route("/builder")]
  CharacterBuilder {},
  #[route("/keywords")]
  KeywordList {},
  #[nest("/sheets")]
    #[route("/")]
    CharacterSheetList {},
    #[route("/:id")]
    SingleChracterSheet { id: String },
  #[end_nest]
  #[route("/all_skills")]
  FullSkillList {},
  #[route("/inherent")]
  InherentSkills {},
  #[route("/rules")]
  MainRules {},
}

#[component]
fn App() -> Element {
  GameLibrarySignal::use_context_provider();
  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    Router::<Route> {}
  }
}
