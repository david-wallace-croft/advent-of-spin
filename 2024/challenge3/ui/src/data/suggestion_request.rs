use ::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SuggestionRequest {
  pub age: u8,
  pub likes: String,
  pub name: String,
}
