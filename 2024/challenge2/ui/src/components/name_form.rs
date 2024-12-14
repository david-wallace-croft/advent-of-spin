use super::super::data::calculated::Calculated;
use ::dioxus::prelude::*;
use ::tracing::*;

#[allow(non_snake_case)]
pub fn NameForm() -> Element {
  rsx! {
    form {
      onsubmit: move |event| {
        debug!("Submitted! {event:?}");

        let name_option: Option<String>
          = Calculated::parse_name(event);

        if let Some(name) = name_option {
          debug!("Name: {name:?}");

          let _future = use_resource(move || {
            let name_clone = name.clone();

            async {
              Calculated::request_calculation(name_clone).await;
            }
          });
        } else {
          debug!("Invalid name");
        }
      },
      input {
        name: "name",
        placeholder: "Name",
        r#type: "text",
      }
      button {
        r#type: "submit",
        "Add"
      }
    }
  }
}
