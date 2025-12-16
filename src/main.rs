mod asset;
mod character;
mod common;
mod equipment;
mod keyword;
mod operator;
mod pages;
mod panels;
mod path;
mod progression;
mod rules;
mod server;
mod skill;

use dioxus::prelude::*;
use pages::*;
use panels::*;

use crate::server::prelude::{KeywordCache, PathCache, SkillCache};

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

  #[route("/rules")]
  MainRules {},

  #[route("/builder")]
  CharacterBuilder {},

  #[nest("/paths")]
    #[route("/")]
    PathsPage {},

    #[route("/:id")]
    SinglePath { id: String },
  #[end_nest]

  #[nest("/skills")]
    #[route("/")]
    SkillsPage {},

    #[route("/:id")]
    SingleSkillPage { id: String },
  #[end_nest]
  
  #[nest("/sheets")]
    #[route("/")]
    CharacterSheetPage {},
    #[route("/:id")]
    SingleChracterSheet { id: String },
}

#[component]
fn App() -> Element {
  KeywordCache::use_context_provider();
  SkillCache::use_context_provider();
  PathCache::use_context_provider();
  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    Router::<Route> {}
  }
}
