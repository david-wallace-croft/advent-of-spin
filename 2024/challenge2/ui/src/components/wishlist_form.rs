use super::super::data::wishlist::Wishlist;
use ::dioxus::prelude::*;
use ::tracing::*;

#[component]
pub fn WishlistForm() -> Element {
  rsx! {
    form {
      onsubmit: move |event| {
        debug!("Submitted! {event:?}");

        let wishlist_option: Option<Wishlist> = Wishlist::parse_wishlist(event);

        Wishlist::upload_wishlist(wishlist_option)
      },
      input {
        name: "name",
        placeholder: "Name",
        r#type: "text",
      }
      input {
        name: "item0",
        placeholder: "First Gift",
        r#type: "text",
      }
      input {
        name: "item1",
        placeholder: "Second Gift",
        r#type: "text",
      }
      input {
        name: "item2",
        placeholder: "Third Gift",
        r#type: "text",
      }
      button {
        r#type: "submit",
        "Add"
      }
    }
  }
}
