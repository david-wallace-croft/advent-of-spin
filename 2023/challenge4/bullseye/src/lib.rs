use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::{FromBody, IntoBody};
use spin_sdk::http::{send, IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::fmt::{self, Display, Formatter};
use std::thread;
use std::time::Duration;

const REQUEST_DELAY_MILLIS: u64 = 10_000;
const SLOT_COUNT: usize = 3;
const SYMBOL_COUNT: usize = 5;

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

struct Hint {
  bulls: u8,
  cows: u8,
}

#[derive(Debug)]
struct Permutation {
  symbols: [u8; SLOT_COUNT],
}

impl Permutation {
  fn has_all_unique_symbols(&self) -> bool {
    let mut already_has_symbol = [false; SYMBOL_COUNT];
    for symbol in &self.symbols {
      let symbol_index = *symbol as usize;
      if already_has_symbol[symbol_index] {
        return false;
      }
      already_has_symbol[symbol_index] = true;
    }
    true
  }
}

impl Display for Permutation {
  fn fmt(
    &self,
    formatter: &mut Formatter<'_>,
  ) -> fmt::Result {
    for symbol in &self.symbols {
      write!(formatter, "{symbol}")?;
    }
    Ok(())
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
  let mut guesses = make_permutations();
  dbg!(&guesses);
  let mut game_id_option: Option<String> = None;
  let mut rounds = Vec::new();
  while let Some(guess) = guesses.pop() {
    let bulls_cows_output: BullsCowsOutput =
      send_guess(&game_id_option, &guess).await?;
    let BullsCowsOutput {
      bulls,
      cows,
      game_id,
      guesses,
      solved,
    } = bulls_cows_output;
    game_id_option = Some(game_id);
    let round = format!("{guesses}: {guess} -> ({bulls}, {cows})");
    dbg!(&round);
    rounds.push(round);
    if solved {
      break;
    }
    thread::sleep(Duration::from_millis(REQUEST_DELAY_MILLIS));
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

fn make_hint(
  guess: &Permutation,
  secret: &Permutation,
) -> Hint {
  let mut hint = Hint {
    bulls: 0,
    cows: 0,
  };
  let mut incorrect_guess_symbols = Vec::new();
  let mut unmatched_secret_symbols = Vec::new();
  for index in 0..SLOT_COUNT {
    let guess_value: u8 = guess.symbols[index];
    let secret_value: u8 = secret.symbols[index];
    if guess_value == secret_value {
      hint.bulls += 1;
    } else {
      incorrect_guess_symbols.push(guess_value);
      unmatched_secret_symbols.push(secret_value);
    }
  }
  for incorrect_guess in incorrect_guess_symbols {
    if let Some(index) =
      unmatched_secret_symbols
        .iter()
        .position(|unmatched_secret_value| {
          *unmatched_secret_value == incorrect_guess
        })
    {
      hint.cows += 1;
      unmatched_secret_symbols.swap_remove(index);
    }
  }
  hint
}

fn make_permutations() -> Vec<Permutation> {
  let mut permutations: Vec<Permutation> = Vec::new();
  for permutation_index in (0..SYMBOL_COUNT.pow(SLOT_COUNT as u32)).rev() {
    let mut symbols = [0; SLOT_COUNT];
    for slot_index in 0..SLOT_COUNT {
      symbols[SLOT_COUNT - slot_index - 1] = ((permutation_index
        / SYMBOL_COUNT.pow(slot_index as u32))
        % SYMBOL_COUNT) as u8;
    }
    let permutation = Permutation {
      symbols,
    };
    if permutation.has_all_unique_symbols() {
      permutations.push(permutation);
    }
  }
  permutations
}

fn make_url(
  game_id: &Option<String>,
  guess: &Permutation,
) -> String {
  let mut url = format!("https://bulls-n-cows.fermyon.app/api?guess={guess}");
  if let Some(game_id) = game_id {
    url.push_str(&format!("&id={game_id}"));
  }
  url
}

async fn send_guess(
  game_id: &Option<String>,
  guess: &Permutation,
) -> anyhow::Result<BullsCowsOutput> {
  let url = make_url(game_id, guess);
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
