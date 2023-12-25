use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::{FromBody, IntoBody};
use spin_sdk::http::{send, IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::fmt::{self, Display, Formatter};
use std::thread;
use std::time::Duration;

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

impl Display for Candidate {
  fn fmt(
    &self,
    formatter: &mut Formatter<'_>,
  ) -> fmt::Result {
    write!(formatter, "{}{}{}", self.0, self.1, self.2)
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
  let mut game_id_option: Option<String> = None;
  let mut rounds = Vec::new();
  while let Some(candidate) = candidates.pop() {
    let bulls_cows_output: BullsCowsOutput =
      send_guess(&candidate, &game_id_option).await?;
    let BullsCowsOutput {
      bulls,
      cows,
      game_id,
      guesses,
      solved,
    } = bulls_cows_output;
    game_id_option = Some(game_id);
    let round = format!("{guesses}: {candidate} -> ({bulls}, {cows})");
    dbg!(&round);
    rounds.push(round);
    if solved {
      break;
    }
    thread::sleep(Duration::from_millis(10_000));
  }
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
  game_id: &Option<String>,
) -> String {
  let mut url =
    format!("https://bulls-n-cows.fermyon.app/api?guess={candidate}");
  if let Some(game_id) = game_id {
    url.push_str(&format!("&id={game_id}"));
  }
  url
}

async fn send_guess(
  guess: &Candidate,
  game_id: &Option<String>,
) -> anyhow::Result<BullsCowsOutput> {
  let url = make_url(guess, game_id);
  dbg!(&url);
  let outbound_req = Request::get(url);
  let response: http::Response<BullsCowsOutput> = send(outbound_req).await?;
  match response.status() {
    StatusCode::OK => {
      let bulls_cows_output: &BullsCowsOutput = response.body();
      Ok(bulls_cows_output.clone())
    },
    _ => Err(anyhow::anyhow!("Unexpected response status code")),
  }
}
