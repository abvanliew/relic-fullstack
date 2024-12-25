mod panels;
mod relic;
mod server;

use dioxus::prelude::*;
use panels::{Navbar, Home, Skills, SingleSkill};

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
    Skills {},
    #[route("/:id")]
    SingleSkill { id: String },
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    Router::<Route> {}
  }
}
