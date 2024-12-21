use super::super::data::suggestion::Suggestion;
use super::super::data::suggestion_request::SuggestionRequest;
use ::dioxus::prelude::*;
use ::tracing::*;

#[component]
pub fn GiftSuggestionForm() -> Element {
  let suggestion_signal: Signal<Suggestion> =
    consume_context::<Signal<Suggestion>>();

  rsx! {
    form {
      onsubmit: move |event| {
        debug!("Submitted! {event:?}");

        let name_option: Option<String>
          = Suggestion::parse_name(&event);

        let age_option: Option<u8>
          = Suggestion::parse_age(&event);

        let likes_option: Option<String>
          = Suggestion::parse_likes(&event);

        update_suggestion(suggestion_signal.clone(), name_option, age_option, likes_option)
      },
      input {
        name: "name",
        placeholder: "Name",
        r#type: "text",
      }
      input {
        name: "age",
        placeholder: "Age",
        r#type: "number",
      }
      input {
        name: "likes",
        placeholder: "Likes",
        r#type: "text",
      }
      button {
        r#type: "submit",
        "Suggest"
      }
    }
  }
}

async fn update_suggestion(
  mut suggestion_signal: Signal<Suggestion>,
  name_option: Option<String>,
  age_option: Option<u8>,
  likes_option: Option<String>,
) {
  let Some(name) = name_option else {
    return;
  };

  debug!("Name: {name:?}");

  let Some(age) = age_option else {
    return;
  };

  debug!("Age: {age:?}");

  let Some(likes) = likes_option else {
    return;
  };

  debug!("Likes: {likes:?}");

  let suggestion_request: SuggestionRequest = SuggestionRequest {
    age,
    likes: likes.to_string(),
    name: name.to_string(),
  };

  let suggestion_option: Option<Suggestion> =
    Suggestion::request_suggestion(suggestion_request).await;

  if let Some(suggestion) = suggestion_option {
    debug!("Suggestion: {suggestion:?}");

    let suggestion_clone = suggestion.clone();

    *suggestion_signal.write() = suggestion_clone;
  }
}
