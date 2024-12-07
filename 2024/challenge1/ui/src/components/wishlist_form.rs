use super::super::data::wishlist::Wishlist;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn WishlistForm() -> Element {
  rsx! {
    form {
      onsubmit: move |event| {
        tracing::debug!("Submitted! {event:?}");

        let wishlist: Option<Wishlist> = Wishlist::parse_wishlist(event);

        if let Some(wishlist) = wishlist {
          tracing::debug!("Wishlist: {wishlist:?}");
        } else {
          tracing::debug!("Invalid Wishlist");
        }
      },
      input {
        name: "name",
        placeholder: "Name",
        type: "text",
      }
      input {
        name: "item0",
        placeholder: "First Gift",
        type: "text",
      }
      input {
        name: "item1",
        placeholder: "Second Gift",
        type: "text",
      }
      input {
        name: "item2",
        placeholder: "Third Gift",
        type: "text",
      }
      button {
        r#type: "submit",
        "Add"
      }
    }
  }
}
