use ::dioxus::prelude::*;
use reqwest::{Error, Response};
use ::serde::{Deserialize, Serialize};
use ::std::collections::HashMap;
use ::std::rc::Rc;
use ::tracing::{debug, error};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Calculated {
  pub name: String,
  pub score: u8,
}

impl Calculated {
  pub fn parse_name(form_data_event: Event<FormData>) -> Option<String> {
    let form_data: Rc<FormData> = form_data_event.data();

    let values: HashMap<String, FormValue> = form_data.values();

    let name_option: Option<&FormValue> = values.get("name");

    let name: String = name_option?.as_value();

    if name.is_empty() {
      return None;
    }

    Some(name)
  }

  // TODO: Can this be a string slice?
  pub async fn request_calculation(name: String) {
    let client = reqwest::Client::new();

    let response_result: Result<Response, Error> = client
      .get(format!(
        "https://challenge2-xqnag9fm.fermyon.app/api/naughty-or-nice/{}",
        name
      ))
      .send()
      .await;

    debug!("Response result: {response_result:?}");

    let Ok(response) = response_result else {
      error!("Failed to get response");

      return;
    };

    debug!("Response: {response:?}");

    let calculated_result: Result<Calculated, Error> = response.json().await;

    let Ok(calculated) = calculated_result else {
      error!("Failed to parse calculated");

      return;
    };

    debug!("Calculated: {calculated:?}");
  }
}
