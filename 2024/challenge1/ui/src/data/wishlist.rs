use ::dioxus::prelude::*;
use ::serde::{Deserialize, Serialize};
use ::std::collections::HashMap;
use ::std::rc::Rc;
use ::tracing::{debug, error};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Wishlist {
  pub name: String,
  pub items: Vec<String>,
}

impl Wishlist {
  pub fn parse_wishlist(form_data_event: Event<FormData>) -> Option<Wishlist> {
    let form_data: Rc<FormData> = form_data_event.data();

    let values: HashMap<String, FormValue> = form_data.values();

    let name_option: Option<&FormValue> = values.get("name");

    let name: String = name_option?.as_value();

    if name.is_empty() {
      return None;
    }

    let items: Vec<String> = values
      .iter()
      .filter_map(|(key, value)| {
        if key.starts_with("item") {
          Some(value.as_value())
        } else {
          None
        }
      })
      .collect();

    Some(Wishlist {
      name,
      items,
    })
  }

  pub async fn upload_wishlist(wishlist: Wishlist) {
    let wishlist_json_result: serde_json::Result<String> =
      serde_json::to_string(&wishlist);

    let Ok(wishlist_json) = wishlist_json_result else {
      error!("Failed to serialize wishlist");

      return;
    };

    let client = reqwest::Client::new();

    debug!("Uploading wishlist: {wishlist_json}");

    let _response = client
      .post("https://challenge1-fbgn5xod.fermyon.app/api/wishlists")
      .header("Content-Type", "application/json")
      .body(wishlist_json)
      .send()
      .await;
  }
}
