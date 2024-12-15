use self::route::Route;
use ::dioxus::prelude::*;
use ::tracing::Level;

mod components;
mod data;
mod route;
mod views;

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
  Ok(
    Route::static_routes()
      .into_iter()
      .map(|route| route.to_string())
      .collect::<Vec<_>>(),
  )
}

fn main() {
  dioxus_logger::init(Level::DEBUG).expect("Failed to initialize logger");

  // Using static site generation (SSG) because the challenge automated
  // validation test requires that /naughty-or-nice.html can be loaded directly
  LaunchBuilder::new()
    .with_cfg(server_only! {
      ServeConfig::builder()
        // turn on incremental site generation with the .incremental() method
        .incremental(IncrementalRendererConfig::new())
        .build()
        .unwrap()
    })
    .launch(|| {
      rsx! {
        Router::<Route> {}
      }
    })
}

#[component]
fn App() -> Element {
  rsx! {
    Router::<Route> {}
  }
}
