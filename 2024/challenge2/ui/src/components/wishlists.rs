use super::super::data::wishlist::Wishlist;
use ::dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use ::dioxus::prelude::*;
use ::std::cell::Ref;

async fn get_wishlists() -> Result<Vec<Wishlist>, anyhow::Error> {
  let response: reqwest::Response =
    reqwest::get("https://challenge2-xqnag9fm.fermyon.app/api/wishlists")
      .await?;

  let wishlists = response.json().await?;

  Ok(wishlists)
}

#[component]
pub fn Wishlists() -> Element {
  let wishlists_resource: Resource<Result<Vec<Wishlist>, anyhow::Error>> =
    use_resource(move || get_wishlists());

  let wishlists_option: GenerationalRef<
    Ref<Option<Result<Vec<Wishlist>, anyhow::Error>>>,
  > = wishlists_resource.read_unchecked();

  match &*wishlists_option {
    Some(Ok(wishlists)) => rsx! {
      table {
      thead {
      tr {
      th { "Name" }
      th { "Items" }
      }
      }
      tbody {
      for wishlist in wishlists {
        tr {
        td { "{&wishlist.name}" }
        td { "{&wishlist.items:?}" }
        }
      }
      }
      },
    },
    Some(Err(err)) => rsx! { "Error loading wishlists: {err}" },
    None => {
      rsx! {"Loading wishlists..."}
    },
  }
}
