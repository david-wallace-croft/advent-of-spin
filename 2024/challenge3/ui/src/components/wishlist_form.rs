use super::super::data::wishlist::Wishlist;
use ::dioxus::prelude::*;
use ::tracing::*;

#[component]
pub fn WishlistForm() -> Element {
  let wishlists_result_signal = consume_context::<Signal<Vec<Wishlist>>>();

  rsx! {
    form {
      onsubmit: move |event| {
        debug!("Submitted! {event:?}");

        handle_submit(event, wishlists_result_signal)
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

async fn handle_submit(
  event: dioxus_core::Event<FormData>,
  wishlists_result_signal: Signal<Vec<Wishlist>>,
) {
  let wishlist_option: Option<Wishlist> = Wishlist::parse_wishlist(event);

  Wishlist::upload_wishlist(wishlist_option).await;

  Wishlist::update_wishlists(wishlists_result_signal).await;
}
