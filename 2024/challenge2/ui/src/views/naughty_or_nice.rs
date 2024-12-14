use super::super::components::name_form::NameForm;
use super::super::components::score::Score;
use ::dioxus::prelude::*;

const NAUGHTY_OR_NICE_CSS: Asset =
  asset!("/assets/styling/naughty-or-nice.css");

#[component]
pub fn NaughtyOrNice() -> Element {
  rsx! {
    document::Link { rel: "stylesheet", href: NAUGHTY_OR_NICE_CSS}

    NameForm { }

    Score { }
  }
}
