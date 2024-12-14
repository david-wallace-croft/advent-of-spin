use super::super::data::wishlist::Wishlist;
use ::dioxus::prelude::*;
use ::tracing::*;

#[component]
pub fn WishlistForm() -> Element {
  rsx! {
    form {
      onsubmit: move |event| {
        debug!("Submitted! {event:?}");

        let wishlist: Option<Wishlist> = Wishlist::parse_wishlist(event);

        if let Some(wishlist) = wishlist {
          debug!("Wishlist: {wishlist:?}");

          let _future = use_resource(move || {
            let wishlist_clone = wishlist.clone();

            async {
              Wishlist::upload_wishlist(wishlist_clone).await;
            }
          });
        } else {
          debug!("Invalid Wishlist");
        }
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
