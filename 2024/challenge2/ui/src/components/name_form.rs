use super::super::data::calculated::Calculated;
use ::dioxus::prelude::*;
use ::tracing::*;

#[component]
pub fn NameForm() -> Element {
  let calculated_signal: Signal<Calculated> =
    consume_context::<Signal<Calculated>>();

  rsx! {
    form {
      onsubmit: move |event| {
        debug!("Submitted! {event:?}");

        let name_option: Option<String>
          = Calculated::parse_name(event);

        update_calculated(calculated_signal.clone(), name_option)
      },
      input {
        name: "name",
        placeholder: "Name",
        r#type: "text",
      }
      button {
        r#type: "submit",
        "Calculate"
      }
    }
  }
}

async fn update_calculated(
  mut calculated_signal: Signal<Calculated>,
  name_option: Option<String>,
) {
  let Some(name) = name_option else {
    return;
  };

  debug!("Name: {name:?}");

  let calculated_option: Option<Calculated> =
    Calculated::request_calculation(&name).await;

  if let Some(calculated) = calculated_option {
    debug!("Calculated: {calculated:?}");

    let calculated_clone = calculated.clone();

    *calculated_signal.write() = calculated_clone;
  }
}
