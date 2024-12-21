// use super::super::components::name_form::NameForm;
// use super::super::components::score::Score;
// use crate::data::calculated::Calculated;
// use ::dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const NAUGHTY_OR_NICE_CSS: Asset = asset!("/assets/naughty-or-nice.css");

// #[component]
// pub fn NaughtyOrNice() -> Element {
//   use_context_provider(|| Signal::new(Calculated::default()));

//   rsx! {
//     document::Link { rel: "icon", href: FAVICON }

//     document::Link { rel: "stylesheet", href: NAUGHTY_OR_NICE_CSS}

//     NameForm { }

//     Score { }
//   }
// }
