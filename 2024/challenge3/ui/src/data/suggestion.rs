use ::dioxus::prelude::*;
use ::reqwest::{Error, Response};
use ::serde::{Deserialize, Serialize};
use ::std::collections::HashMap;
use ::std::rc::Rc;
use ::tracing::{debug, error};
use super::super::constants;
use super::suggestion_request::SuggestionRequest;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Suggestion {
  pub gift_suggestions: String,
  pub name: String,
}

impl Suggestion {
  pub fn parse_age(form_data_event: &Event<FormData>) -> Option<u8> {
    let form_data: Rc<FormData> = form_data_event.data();

    let values: HashMap<String, FormValue> = form_data.values();

    let age_option: Option<&FormValue> = values.get("age");

    let age_string: String = age_option?.as_value();

    if age_string.is_empty() {
      return None;
    }

    let age_result: Result<u8, _> = age_string.parse();

    let Ok(age) = age_result else {
      return None;
    };

    Some(age)
  }

  pub fn parse_likes(form_data_event: &Event<FormData>) -> Option<String> {
    let form_data: Rc<FormData> = form_data_event.data();

    let values: HashMap<String, FormValue> = form_data.values();

    let likes_option: Option<&FormValue> = values.get("likes");

    let likes: String = likes_option?.as_value();

    if likes.is_empty() {
      return None;
    }

    Some(likes)
  }

  pub fn parse_name(form_data_event: &Event<FormData>) -> Option<String> {
    let form_data: Rc<FormData> = form_data_event.data();

    let values: HashMap<String, FormValue> = form_data.values();

    let name_option: Option<&FormValue> = values.get("name");

    let name: String = name_option?.as_value();

    if name.is_empty() {
      return None;
    }

    Some(name)
  }

  pub async fn request_suggestion(suggestion_request: SuggestionRequest) -> Option<Suggestion> {
    let suggestion_request_json_result: serde_json::Result<String> =
      serde_json::to_string(&suggestion_request);

    let Ok(suggestion_request_json) = suggestion_request_json_result else {
      error!("Failed to serialize suggestion request");

      return None;
    };

    let client = reqwest::Client::new();

    // TODO: attach body
    let response_result: Result<Response, Error> = client
      .post(format!(
        "{}/generate-gift-suggestions",
        constants::API_URL,
      ))
      .header("Content-Type", "application/json")
      .body(suggestion_request_json)
      .send()
      .await;

    debug!("Response result: {response_result:?}");

    let Ok(response) = response_result else {
      error!("Failed to get response");

      return None;
    };

    debug!("Response: {response:?}");

    let suggestion_result: Result<Suggestion, Error> = response.json().await;

    let Ok(suggestion) = suggestion_result else {
      error!("Failed to parse suggestion");

      return None;
    };

    debug!("Suggestion: {suggestion:?}");

    Some(suggestion)
  }
}

impl Default for Suggestion {
  fn default() -> Self {
    Self {
      name: "Loading...".to_string(),
      gift_suggestions: "Loading...".to_string(),
    }
  }
}
