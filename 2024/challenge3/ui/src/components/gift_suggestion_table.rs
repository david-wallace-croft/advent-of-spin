use super::super::data::suggestion::Suggestion;
use ::dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use ::dioxus::prelude::*;
use ::std::cell::Ref;

#[component]
pub fn GiftSuggestionTable() -> Element {
  let suggestion_signal: Signal<Suggestion> =
    consume_context::<Signal<Suggestion>>();

  let suggestion: GenerationalRef<Ref<Suggestion>> =
    suggestion_signal.read_unchecked();

  rsx! {
    table {
    thead {
    tr {
    th { "Name" }
    th { "Suggestion" }
    }
    }
    tbody {
    tr {
    td { "{&suggestion.name}" }
    td { "{&suggestion.gift_suggestions}" }
    }
    }
    }
  }
}
