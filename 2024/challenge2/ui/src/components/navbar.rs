use super::super::route::Route;
use ::dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/navbar.css");

#[component]
pub fn Navbar() -> Element {
  rsx! {
      document::Link { rel: "stylesheet", href: NAVBAR_CSS }

      div {
          id: "navbar",
          Link {
              to: Route::Home{},
              "Home"
          }
          Link {
              to: Route::NaughtyOrNice {},
              "Naughty or Nice"
          }
      }

      Outlet::<Route> {}
  }
}
