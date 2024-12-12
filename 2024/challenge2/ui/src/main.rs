use self::route::Route;
use ::dioxus::prelude::*;
use ::tracing::Level;

mod components;
mod data;
mod route;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
  dioxus_logger::init(Level::DEBUG).expect("Failed to initialize logger");

  launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "icon", href: FAVICON }

    document::Link { rel: "stylesheet", href: MAIN_CSS }

    Router::<Route> {}
  }
}
