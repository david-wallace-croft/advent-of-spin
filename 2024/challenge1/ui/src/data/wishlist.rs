use ::dioxus::prelude::*;
use ::serde::{Deserialize, Serialize};
use ::std::collections::HashMap;
use ::std::rc::Rc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Wishlist {
  pub name: String,
  pub items: Vec<String>,
}

impl Wishlist {
  pub(crate) fn parse_wishlist(
    form_data_event: Event<FormData>
  ) -> Option<Wishlist> {
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
}
