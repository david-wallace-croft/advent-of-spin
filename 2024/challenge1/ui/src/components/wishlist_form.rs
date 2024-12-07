use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn WishlistForm() -> Element {
  rsx! {
    form {
      input {
        type: "text",
        placeholder: "Name",
        name: "name",
      }
      input {
        type: "text",
        placeholder: "First Gift",
        name: "item0",
      }
      input {
        type: "text",
        placeholder: "Second Gift",
        name: "item1",
      }
      input {
        type: "text",
        placeholder: "Third Gift",
        name: "item2",
      }
      button {
        type: "button",
        "Add"
      }
    }
  }
}
