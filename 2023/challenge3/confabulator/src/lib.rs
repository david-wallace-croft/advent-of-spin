use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Json, Response};
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
  let mut prompt = r#"Tell an engaging Christmas story. "#.to_owned();
  prompt.push_str(&format!("The story should take place in {}.\n", place));
  let characters_length = characters.len();
  if characters_length == 1 {
    prompt.push_str("The story should include the character ");
    prompt.push_str(&characters[0]);
    prompt.push_str(".");
  } else if characters_length > 1 {
    prompt.push_str("The story should include the characters ");
    for i in 0..characters_length {
      if i == characters_length - 1 {
        prompt.push_str("and ");
      }
      prompt.push_str(&characters[i]);
      if i == characters_length - 1 {
        prompt.push_str(".");
      } else {
        prompt.push_str(", ");
      }
    }
  }
  let objects_length = objects.len();
  if objects_length == 1 {
    prompt.push_str("The story should include the object ");
    prompt.push_str(&objects[0]);
    prompt.push_str(".");
  } else if objects_length > 1 {
    prompt.push_str("The story should include the objects ");
    for i in 0..objects_length {
      if i == objects_length - 1 {
        prompt.push_str("and ");
      }
      prompt.push_str(&objects[i]);
      if i == objects_length - 1 {
        prompt.push_str(".");
      } else {
        prompt.push_str(", ");
      }
    }
  }
  format!(
    "{:?}",
    llm::infer(llm::InferencingModel::Llama2Chat, &prompt)
  )
}
