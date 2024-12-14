use super::super::data::wishlist::Wishlist;
use ::dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use ::dioxus::prelude::*;
use ::std::cell::Ref;

#[component]
pub fn Wishlists() -> Element {
  let wishlists_signal = consume_context::<Signal<Vec<Wishlist>>>();

  let wishlists_signal_clone = wishlists_signal.clone();

  // TODO: Is use_resource() the right function to use here?
  let _ =
    use_resource(move || Wishlist::update_wishlists(wishlists_signal_clone));

  let wishlists: GenerationalRef<Ref<Vec<Wishlist>>> =
    wishlists_signal.read_unchecked();

  rsx! {
    table {
    thead {
    tr {
    th { "Name" }
    th { "Items" }
    }
    }
    tbody {
    for wishlist in wishlists.iter() {
      tr {
      td { "{&wishlist.name}" }
      td { "{&wishlist.items:?}" }
      }
    }
    }
    },
  }
}
