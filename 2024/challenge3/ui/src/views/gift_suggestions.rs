use super::super::components::gift_suggestion_form::GiftSuggestionForm;
use super::super::components::gift_suggestion_table::GiftSuggestionTable;
use super::super::data::suggestion::Suggestion;
use ::dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const GIFT_SUGGESTIONS_CSS: Asset = asset!("/assets/gift-suggestions.css");

#[component]
pub fn GiftSuggestions() -> Element {
  use_context_provider(|| Signal::new(Suggestion::default()));

  rsx! {
    document::Link { rel: "icon", href: FAVICON }

    document::Link { rel: "stylesheet", href: GIFT_SUGGESTIONS_CSS}

    GiftSuggestionForm { }

    GiftSuggestionTable { }
  }
}
