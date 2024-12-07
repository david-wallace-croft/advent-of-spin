use dioxus::prelude::*;

use components::wishlists::Wishlists;

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
  launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "icon", href: FAVICON }

    document::Link { rel: "stylesheet", href: MAIN_CSS }

    Wishlists {}
  }
}
