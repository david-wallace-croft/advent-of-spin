use self::components::wishlist_form::WishlistForm;
use self::components::wishlists::Wishlists;
use ::dioxus::prelude::*;
use ::tracing::Level;

mod components;
mod data;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const IMAGE_SRC_SANTA_CLAUS: Asset = asset!("/assets/santa-claus.jpg");
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

    img {
      class: "app-santa-claus",
      src: IMAGE_SRC_SANTA_CLAUS
    }

    div {
    p {
    "Solution to the "
    a {
      href:
        "https://github.com/fermyon/advent-of-spin/tree/main/2024/Challenge-1",
      target: "_blank",
      "Fermyon Advent of Spin 2024 Challenge 1"
    }
    br { }
    "Both front-end and back-end use Rust compiled to WebAssembly (Wasm)"
    br { }
    "Author: "
    a {
      href: "https://www.croftsoft.com/people/david/research/rust-wasm/",
      target: "_blank",
      "David Wallace Croft, M.Sc."
    }
    br { }
    "Back-end: "
    a {
      href: "https://www.fermyon.com/spin",
      target: "_blank",
      "Fermyon Spin 3.0"
    }
    br { }
    "Front-end: "
    a {
      href: "https://dioxuslabs.com/",
      target: "_blank",
      "Dioxus 0.6"
    }
    br { }
    "Image: "
    a {
      href: "https://gemini.google.com/",
      target: "_blank",
      "Google Gemini / Imagen 3"
    }
    }
    }

    WishlistForm {}

    Wishlists {}
  }
}
