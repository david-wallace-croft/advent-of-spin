use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Json, Response};
use spin_sdk::http_component;

#[derive(Deserialize)]
struct Input {
  characters: Vec<String>,
  objects: Vec<String>,
  place: String,
}

#[derive(Serialize)]
struct Output {
  story: String,
}

impl IntoBody for Output {
  fn into_body(self) -> Vec<u8> {
    serde_json::to_string(&self).unwrap().into_body()
  }
}

#[http_component]
fn handle_request(
  req: http::Request<Json<Input>>
) -> anyhow::Result<impl IntoResponse> {
  let (status, body): (StatusCode, Option<Output>) = match *req.method() {
    Method::POST => {
      let json_input: &Json<Input> = req.body();
      let output = Output {
        story: confabulate(
          &json_input.characters,
          &json_input.objects,
          &json_input.place,
        ),
      };
      (StatusCode::OK, Some(output))
    },
    _ => (StatusCode::METHOD_NOT_ALLOWED, None),
  };
  let response = Response::builder()
    .body(body)
    .header("Content-Type", "application/json")
    .status(status)
    .build();
  Ok(response)
}

fn confabulate(
  characters: &[String],
  objects: &[String],
  place: &str,
) -> String {
  let mut story = String::new();
  story.push_str(&format!("The story begins in {}.\n", place));
  for i in 0..characters.len() {
    story.push_str(&format!("{} is {}.\n", characters[i], objects[i]));
  }
  story
}
