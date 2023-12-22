use serde::Deserialize;
use spin_sdk::http::conversions::FromBody;
use spin_sdk::http::{send, IntoResponse, Request};
use spin_sdk::http_component;

#[derive(Debug, Deserialize)]
struct BullsCowsOutput {
  bulls: u8,
  cows: u8,
  #[serde(alias = "gameId")]
  game_id: String,
  guesses: u8,
  solved: bool,
}

impl FromBody for BullsCowsOutput {
  fn from_body(body: Vec<u8>) -> Self {
    serde_json::from_slice(&body).unwrap()
  }
}

#[http_component]
// The trigger handler (in this case an HTTP handler) has to be async
// so we can `await` the outbound send.
async fn handle_request(_req: Request) -> anyhow::Result<impl IntoResponse> {
  // For this example, use the spin_sdk::http::RequestBuilder type
  // for the outbound request.
  let outbound_req =
    Request::get("https://bulls-n-cows.fermyon.app/api?guess=012");

  let response: http::Response<BullsCowsOutput> = send(outbound_req).await?;
  if response.status() != 200 {
    return Ok(
      http::Response::builder()
        .status(500)
        .body("Failed to fetch the test page")?,
    );
  }

  let response_body: &BullsCowsOutput = response.body();

  dbg!("{:?}", response_body);

  Ok(
    http::Response::builder()
      .status(200)
      .header("content-type", "text/plain")
      .body("Test")?,
  )
}
