mod character;
mod component;
mod equipment;
mod operator;
mod panels;
mod path;
mod rule;
mod server;
mod skill;

use dioxus::prelude::*;
use panels::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

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
  #[route("/sheet")]
  CharacterSheet {},
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    Router::<Route> {}
  }
}
