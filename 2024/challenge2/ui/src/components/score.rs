use super::super::data::calculated::Calculated;
use ::dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use ::dioxus::prelude::*;
use ::std::cell::Ref;

async fn get_calculated() -> Result<Calculated, anyhow::Error> {
  let response: reqwest::Response = reqwest::get(
    "https://challenge2-xqnag9fm.fermyon.app/api/naughty-or-nice/Krampus",
  )
  .await?;

  let calculated: Calculated = response.json().await?;

  Ok(calculated)
}

#[allow(non_snake_case)]
pub fn Score() -> Element {
  let calculated_resource: Resource<Result<Calculated, anyhow::Error>> =
    use_resource(move || get_calculated());

  let calculated_option: GenerationalRef<
    Ref<Option<Result<Calculated, anyhow::Error>>>,
  > = calculated_resource.read_unchecked();

  match &*calculated_option {
    Some(Ok(calculated)) => rsx! {
      table {
      thead {
      tr {
      th { "Name" }
      th { "Score" }
      }
      }
      tbody {
      tr {
      td { "{&calculated.name}" }
      td { "{&calculated.score}" }
      }
      }
      },
    },
    Some(Err(err)) => rsx! { "Error loading calculated: {err}" },
    None => {
      rsx! {"Loading calculated..."}
    },
  }
}
