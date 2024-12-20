use self::bindings0::deps::components::advent_of_spin::generator;
use self::bindings1::deps::croftsoft::naughty_or_nice::calculator;
use ::serde_json::Value;
use ::spin_sdk::http::Params;
use ::spin_sdk::http::Router;
use ::spin_sdk::http::{IntoResponse, Request, Response};
use ::spin_sdk::http_component;
use ::spin_sdk::key_value::{Error, Store};
use ::std::borrow::Cow;

mod bindings0;
mod bindings1;

#[http_component]
fn handle_route(request: Request) -> Response {
  println!("Handling request to {:?}", request.header("spin-full-url"));

  let mut router = Router::new();

  router.get("/api/naughty-or-nice/:name", naughty_or_nice_get);

  router.get("/api/wishlists", wishlists_get);

  router.post("/api/wishlists", wishlists_post);

  router.post("/api/generate-gift-suggestions", gift_suggestions_post);

  router.handle(request)
}

fn gift_suggestions_post(
  request: Request,
  _params: Params,
) -> anyhow::Result<impl IntoResponse> {
  // Parse the request body as a JSON string

  let body_bytes: &[u8] = request.body().as_ref();

  let body_result: Result<Value, serde_json::Error> =
    serde_json::from_slice(body_bytes);

  let Ok(body) = body_result else {
    return Ok(
      Response::builder()
        .status(400)
        .body("Error parsing request body")
        .build(),
    );
  };

  // Extract the "name" field from the JSON object.
  // Return an HTTP status code 400 Bad Request if the field is missing.

  let name_option: Option<&str> = body["name"].as_str();

  let Some(name) = name_option else {
    // TODO: Maybe this should be a 422 Unprocessable Entity instead
    return Ok(
      Response::builder()
        .status(400)
        .body("name field missing")
        .build(),
    );
  };

  let _result = generator::suggest(name, 3, "gifts");

  // TODO

  // Return an HTTP status of 200 OK

  Ok(Response::builder().status(200).build())
}

fn naughty_or_nice_get(
  _request: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let name: &str = params.get("name").unwrap_or("Grinch");

  let decoded_name: Cow<str> =
    urlencoding::decode(name).unwrap_or(Cow::from(name.to_string()));

  let score = calculator::calculate(&decoded_name);

  let value: Value = serde_json::json!({
    "name": decoded_name,
    "score": score,
  });

  let json_byte_vec_result: Result<Vec<u8>, serde_json::Error> =
    serde_json::to_vec(&value);

  let Ok(json_byte_vec) = json_byte_vec_result else {
    return Ok(
      Response::builder()
        .status(500)
        .body("Error converting value to bytes")
        .build(),
    );
  };

  Ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .body(json_byte_vec)
      .build(),
  )
}

/// Handles wishlist retrieval requests using the HTTP GET method.
fn wishlists_get(
  _req: Request,
  _params: Params,
) -> anyhow::Result<impl IntoResponse> {
  // Open the default key-value store

  let store_result: Result<Store, Error> = Store::open_default();

  // Return an HTTP status code 500 Internal Server Error if the store operation
  // fails

  let Ok(store) = store_result else {
    return Ok(
      Response::builder()
        .status(500)
        .body("Failed to open store")
        .build(),
    );
  };

  // Retrieve all wishlist keys from the key-value store

  let keys_result: Result<Vec<String>, Error> = store.get_keys();

  let Ok(keys) = keys_result else {
    return Ok(
      Response::builder()
        .status(500)
        .body("Error fetching wishlist keys")
        .build(),
    );
  };

  // Fetch the values for each key in the key-value store

  let mut wishlists: Vec<Value> = Vec::new();

  for key in keys {
    let items_result: Result<Option<Vec<u8>>, Error> = store.get(&key);

    let Ok(items_option) = items_result else {
      return Ok(
        Response::builder()
          .status(500)
          .body("Error fetching wishlist")
          .build(),
      );
    };

    let Some(items) = items_option else {
      return Ok(
        Response::builder()
          .status(404)
          .body("Wishlist not found")
          .build(),
      );
    };

    let items_result: Result<Vec<Value>, serde_json::Error> =
      serde_json::from_slice(&items);

    let Ok(items) = items_result else {
      return Ok(
        Response::builder()
          .status(500)
          .body("Error parsing wishlist")
          .build(),
      );
    };

    wishlists.push(serde_json::json!({
      "name": key,
      "items": items,
    }));
  }

  // Return the wishlists as a byte array

  let json_byte_vec_result: Result<Vec<u8>, serde_json::Error> =
    serde_json::to_vec(&wishlists);

  let Ok(json_byte_vec) = json_byte_vec_result else {
    return Ok(
      Response::builder()
        .status(500)
        .body("Error converting wishlists to bytes")
        .build(),
    );
  };

  Ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .body(json_byte_vec)
      .build(),
  )
}

/// Handles wishlist creation requests using the HTTP POST method
fn wishlists_post(
  request: Request,
  _params: Params,
) -> anyhow::Result<impl IntoResponse> {
  // Parse the request body as a JSON string

  let body_bytes: &[u8] = request.body().as_ref();

  let body_result: Result<Value, serde_json::Error> =
    serde_json::from_slice(body_bytes);

  let Ok(body) = body_result else {
    return Ok(
      Response::builder()
        .status(400)
        .body("Error parsing request body")
        .build(),
    );
  };

  // Extract the "name" field from the JSON object.
  // Return an HTTP status code 400 Bad Request if the field is missing.

  let name_option: Option<&str> = body["name"].as_str();

  let Some(name) = name_option else {
    // TODO: Maybe this should be a 422 Unprocessable Entity instead
    return Ok(
      Response::builder()
        .status(400)
        .body("name field missing")
        .build(),
    );
  };

  // Extract the "items" field from the JSON object as a string array.
  // Return an HTTP status code 400 Bad Request if the field is missing.

  let items_option: Option<&Vec<Value>> = body["items"].as_array();

  let Some(items) = items_option else {
    // TODO: Maybe this should be a 422 Unprocessable Entity instead
    return Ok(
      Response::builder()
        .status(400)
        .body("items field missing")
        .build(),
    );
  };

  // Convert the items to a byte array

  let items_result: serde_json::Result<Vec<u8>> = serde_json::to_vec(items);

  let Ok(items_vec) = items_result else {
    return Ok(
      Response::builder()
        .status(400)
        .body("Error converting items to bytes")
        .build(),
    );
  };

  let items: &[u8] = items_vec.as_slice();

  // Open the default key-value store

  let store_result: Result<Store, Error> = Store::open_default();

  // Return an HTTP status code 500 Internal Server Error if the store operation
  // fails

  let Ok(store) = store_result else {
    return Ok(
      Response::builder()
        .status(500)
        .body("failed to open store")
        .build(),
    );
  };

  // Store the wishlist in the key-value store.
  // Return an HTTP status code 500 Internal Server Error if the store operation
  // fails.

  let result: Result<(), Error> = store.set(name, items);

  let Ok(()) = result else {
    return Ok(
      Response::builder()
        .status(500)
        .body("Error storing wishlist")
        .build(),
    );
  };

  // Return an HTTP status of 201 ACCEPTED with no response body

  Ok(Response::builder().status(201).build())
}
