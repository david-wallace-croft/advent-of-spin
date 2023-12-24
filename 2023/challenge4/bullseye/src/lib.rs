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
  rounds: Vec<String>,
}

struct Candidate(u8, u8, u8);

impl From<&Candidate> for String {
  fn from(candidate: &Candidate) -> Self {
    format!("{}{}{}", candidate.0, candidate.1, candidate.2)
  }
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
  let mut candidates = make_candidates();
  let candidate = candidates.pop().unwrap();
  let guess: String = (&candidate).into();
  let url = make_url(&candidate, None);
  dbg!(&url);
  let outbound_req = Request::get(url);
  let response: http::Response<BullsCowsOutput> = send(outbound_req).await?;
  if response.status() != 200 {
    let response = Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .build();
    return Ok(response);
  }
  let bulls_cows_output: &BullsCowsOutput = response.body();
  let BullsCowsOutput {
    bulls,
    cows,
    ..
  } = bulls_cows_output;
  let mut rounds = Vec::new();
  rounds.push(format!("{guess} -> ({bulls}, {cows})"));
  let bullseye_output = BullseyeOutput {
    rounds,
  };
  let response = Response::builder()
    .body(bullseye_output)
    .header("Content-Type", "application/json")
    .status(StatusCode::OK)
    .build();
  Ok(response)
}

fn make_candidates() -> Vec<Candidate> {
  let mut candidates = Vec::new();
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
        let candidate = Candidate(digit0 as u8, digit1 as u8, digit2 as u8);
        candidates.push(candidate);
      }
      already_used[digit1] = false;
    }
    already_used[digit0] = false;
  }
  candidates
}

fn make_url(
  candidate: &Candidate,
  game_id: Option<&str>,
) -> String {
  let guess: String = candidate.into();
  let mut url = format!("https://bulls-n-cows.fermyon.app/api?guess={guess}");
  if let Some(game_id) = game_id {
    url.push_str(&format!("&id={game_id}"));
  }
  url
}
