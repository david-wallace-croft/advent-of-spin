use super::super::components::wishlist_form::WishlistForm;
use super::super::components::wishlists::Wishlists;
use super::super::data::wishlist::Wishlist;
use ::dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const IMAGE_SRC_SANTA_CLAUS: Asset = asset!("/assets/santa-claus.jpg");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn Home() -> Element {
  use_context_provider(|| Signal::new(Vec::<Wishlist>::new()));

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
        "https://github.com/fermyon/advent-of-spin/tree/main/2024/Challenge-2",
      target: "_blank",
      "Fermyon Advent of Spin 2024 Challenge 2"
    }
    br { }
    "Both front-end and back-end use Rust compiled to WebAssembly (Wasm)"
    br { }
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
    br { }
    "Repository: "
    a {
      href:
        "https://github.com/david-wallace-croft/advent-of-spin/tree/main/2024",
      target: "_blank",
      "https://github.com/david-wallace-croft/advent-of-spin/tree/main/2024"
    }
    br { }
    }
    }

    WishlistForm {}

    Wishlists {}
  }
}
