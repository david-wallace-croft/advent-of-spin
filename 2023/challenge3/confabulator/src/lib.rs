use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Json, Response};
use spin_sdk::http_component;

#[derive(Deserialize)]
struct Input {
  capacity: usize,
  kids: Vec<usize>,
  weight: Vec<usize>,
}

#[derive(Serialize)]
struct Output {
  kids: usize,
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
        kids: knapsack(
          json_input.capacity,
          &json_input.kids,
          &json_input.weight,
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

fn knapsack(
  capacity: usize,
  kids: &[usize],
  weight: &[usize],
) -> usize {
  let mut knapsack = vec![0; capacity + 1];
  for i in 0..kids.len() {
    for j in (weight[i]..=capacity).rev() {
      knapsack[j] = knapsack[j].max(knapsack[j - weight[i]] + kids[i]);
    }
  }
  knapsack[capacity]
}
