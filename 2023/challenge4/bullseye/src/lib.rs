use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::{FromBody, IntoBody};
use spin_sdk::http::{send, IntoResponse, Request, Response};
use spin_sdk::http_component;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BullsCowsOutput {
  bulls: u8,
  cows: u8,
  #[serde(alias = "gameId")]
  game_id: String,
  guesses: u8,
  solved: bool,
}

#[derive(Debug, Serialize)]
struct BullseyeOutput {
  bulls_cows_output: BullsCowsOutput,
  guess: String,
}

impl FromBody for BullsCowsOutput {
  fn from_body(body: Vec<u8>) -> Self {
    serde_json::from_slice(&body).unwrap()
  }
}

impl IntoBody for BullseyeOutput {
  fn into_body(self) -> Vec<u8> {
    serde_json::to_string(&self).unwrap().into_body()
  }
}

#[http_component]
async fn handle_request(_req: Request) -> anyhow::Result<impl IntoResponse> {
  let guess = "012".to_string();
  let outbound_req = Request::get(format!(
    "https://bulls-n-cows.fermyon.app/api?guess={guess}"
  ));
  let response: http::Response<BullsCowsOutput> = send(outbound_req).await?;
  if response.status() != 200 {
    let response = Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .build();
    return Ok(response);
  }
  let response_body: &BullsCowsOutput = response.body();
  let bullseye_output = BullseyeOutput {
    bulls_cows_output: response_body.clone(),
    guess,
  };
  let response = Response::builder()
    .body(bullseye_output)
    .header("Content-Type", "application/json")
    .status(StatusCode::OK)
    .build();
  Ok(response)
}

fn make_candidates() -> Vec<[u8; 3]> {
  let mut candidates: Vec<[u8; 3]> = Vec::new();
  let mut already_used: [bool; 5] = [false; 5];
  for digit0 in 0..5 {
    already_used[digit0] = true;
    for digit1 in 0..5 {
      if already_used[digit1] {
        continue;
      }
      already_used[digit1] = true;
      for digit2 in 0..5 {
        if already_used[digit2] {
          continue;
        }
        let candidate: [u8; 3] = [
          digit0 as u8,
          digit1 as u8,
          digit2 as u8,
        ];
        candidates.push(candidate);
      }
      already_used[digit1] = false;
    }
    already_used[digit0] = false;
  }
  candidates
}
