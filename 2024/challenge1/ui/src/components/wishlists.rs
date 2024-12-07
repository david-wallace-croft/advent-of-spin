use ::dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use ::dioxus::prelude::*;
use ::serde::{Deserialize, Serialize};
use ::std::cell::Ref;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct Wishlist {
  name: String,
  items: Vec<String>,
}

async fn get_wishlists() -> Result<Vec<Wishlist>, anyhow::Error> {
  let response: reqwest::Response =
    reqwest::get("https://challenge1-fbgn5xod.fermyon.app/api/wishlists")
      .await?;

  let wishlists = response.json().await?;

  Ok(wishlists)
}

#[allow(non_snake_case)]
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

// rsx! {
//   form {
//     input {
//       type: "text",
//       placeholder: "Name",
//       name: "name",
//     }
//     input {
//       type: "text",
//       placeholder: "Items",
//       name: "items",
//     }
//     button {
//       type: "button",
//       "Add"
//     }
//   }
