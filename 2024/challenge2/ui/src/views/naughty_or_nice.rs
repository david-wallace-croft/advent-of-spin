use super::super::components::score::Score;
use ::dioxus::prelude::*;

const NAUGHTY_OR_NICE_CSS: Asset =
  asset!("/assets/styling/naughty-or-nice.css");

#[component]
pub fn NaughtyOrNice() -> Element {
  rsx! {
    document::Link { rel: "stylesheet", href: NAUGHTY_OR_NICE_CSS}

    p { "Are you on the naughty or nice list?" }

    Score { }
  }
}
