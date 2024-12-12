use super::components::navbar::Navbar;
use super::views::home::Home;
use super::views::naughty_or_nice::NaughtyOrNice;
use ::dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
  #[layout(Navbar)]
  #[route("/")]
  Home {},
  #[route("/naughty-or-nice.html")]
  NaughtyOrNice {},
}
