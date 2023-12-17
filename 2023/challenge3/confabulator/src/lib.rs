use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Json, Response};
use spin_sdk::llm::InferencingResult;
use spin_sdk::{http_component, llm};

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
  let prompt = make_prompt(characters, objects, place);
  fetch_story(&prompt)
}

fn fetch_story(prompt: &str) -> String {
  let result: Result<InferencingResult, spin_sdk::llm::Error> =
    llm::infer(llm::InferencingModel::Llama2Chat, &prompt);
  match result {
    Ok(result) => result.text,
    Err(error) => format!("Error: {:?}", error),
  }
}

fn make_include_prompt(
  items: &[String],
  plural: &str,
  singular: &str,
) -> String {
  let items_length = items.len();
  if items_length == 0 {
    return String::new();
  }
  if items_length == 1 {
    return format!("The story should include the {} {}.", singular, &items[0]);
  }
  let mut include_prompt: String =
    format!("The story should include the {} ", plural);
  for i in 0..items_length {
    if i == items_length - 1 {
      include_prompt.push_str("and ");
    }
    include_prompt.push_str(&items[i]);
    if i == items_length - 1 {
      include_prompt.push_str(".");
    } else {
      include_prompt.push_str(", ");
    }
  }
  include_prompt
}

fn make_prompt(
  characters: &[String],
  objects: &[String],
  place: &str,
) -> String {
  let mut prompt = "Tell an engaging Christmas story. ".to_owned();
  prompt.push_str(&format!("The story should take place in {}. ", place));
  prompt.push_str(&make_include_prompt(characters, "characters", "character"));
  prompt.push_str(&make_include_prompt(objects, "objects", "object"));
  prompt
}
