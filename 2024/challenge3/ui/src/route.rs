use super::components::navbar::Navbar;
use super::views::gift_suggestions::GiftSuggestions;
use super::views::home::Home;
// use super::views::naughty_or_nice::NaughtyOrNice;
use ::dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
  #[layout(Navbar)]
  #[route("/")]
  Home {},
  // Normally this would be just /gift-suggestions without the .html
  // but passing the challenge automated validation test requires it
  #[route("/gift-suggestions.html")]
  GiftSuggestions {},
  // Normally this would be just /naughty-or-nice without the .html
  // but passing the challenge automated validation test requires it
  // #[route("/naughty-or-nice.html")]
  // NaughtyOrNice {},
}
