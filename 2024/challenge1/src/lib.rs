use ::serde_json::Value;
use ::spin_sdk::http::{IntoResponse, Method, Request, Response};
use ::spin_sdk::http_component;
use ::spin_sdk::key_value::{Error, Store};

#[http_component]
fn handle(request: Request) -> anyhow::Result<impl IntoResponse> {
  println!("Handling request to {:?}", request.header("spin-full-url"));

  let method: &Method = request.method();

  let response: Response = match *method {
    Method::Get => handle_wishlist_get(request),
    Method::Post => handle_wishlist_post(request),
    _ => Response::builder().status(405).build(),
  };

  Ok(response)
}

/// Handles wishlist retrieval requests using the HTTP GET method.
fn handle_wishlist_get(req: Request) -> Response {
  // Extract the wishlist name from the request path.
  // Return an HTTP status code 400 Bad Request if the path is empty.

  let name = req.path().trim_start_matches('/');

  if name.is_empty() {
    return Response::builder()
      .status(400)
      .body("missing wishlist name")
      .build();
  }

  // Open the default key-value store

  let store_result: Result<Store, Error> = Store::open_default();

  // Return an HTTP status code 500 Internal Server Error if the store operation
  // fails

  let Ok(store) = store_result else {
    return Response::builder()
      .status(500)
      .body("failed to open store")
      .build();
  };

  // Retrieve the wishlist from the key-value store

  let items_result: Result<Option<Vec<u8>>, Error> = store.get(name);

  // Return an HTTP status code 500 Internal Server Error if the store
  // operation

  let Ok(items_option) = items_result else {
    return Response::builder()
      .status(500)
      .body("Error fetching wishlist")
      .build();
  };

  let Some(items) = items_option else {
    return Response::builder()
      .status(404)
      .body("wishlist not found")
      .build();
  };

  // Convert the bytes for items into a JSON array

  let items_result: Result<Vec<Value>, serde_json::Error> =
    serde_json::from_slice(&items);

  let Ok(items) = items_result else {
    return Response::builder()
      .status(500)
      .body("Error parsing wishlist")
      .build();
  };

  // Return the wishlist items as a JSON array

  Response::builder()
    .header("Content-Type", "application/json")
    .body(serde_json::to_vec(&items).unwrap())
    .build()
}

/// Handles wishlist creation requests using the HTTP POST method.
fn handle_wishlist_post(req: Request) -> Response {
  // Parse the request body as a JSON string

  let body_bytes: &[u8] = req.body().as_ref();

  let body_result: Result<Value, serde_json::Error> =
    serde_json::from_slice(body_bytes);

  let Ok(body) = body_result else {
    return Response::builder()
      .status(400)
      .body("Error parsing request body")
      .build();
  };

  // Extract the "name" field from the JSON object.
  // Return an HTTP status code 400 Bad Request if the field is missing.

  let name_option: Option<&str> = body["name"].as_str();

  let Some(name) = name_option else {
    return Response::builder()
      .status(400)
      .body("name field missing")
      .build();
  };

  // Extract the "items" field from the JSON object as a string array.
  // Return an HTTP status code 400 Bad Request if the field is missing.

  let items_option: Option<&Vec<Value>> = body["items"].as_array();

  let Some(items) = items_option else {
    return Response::builder()
      .status(400)
      .body("items field missing")
      .build();
  };

  // Convert the items to a byte array

  let items_result: serde_json::Result<Vec<u8>> = serde_json::to_vec(items);

  let Ok(items_vec) = items_result else {
    return Response::builder()
      .status(400)
      .body("Error converting items to bytes")
      .build();
  };

  let items: &[u8] = items_vec.as_slice();

  // Open the default key-value store

  let store_result: Result<Store, Error> = Store::open_default();

  // Return an HTTP status code 500 Internal Server Error if the store operation
  // fails

  let Ok(store) = store_result else {
    return Response::builder()
      .status(500)
      .body("failed to open store")
      .build();
  };

  // Store the wishlist in the key-value store.
  // Return an HTTP status code 500 Internal Server Error if the store operation
  // fails.

  let result: Result<(), Error> = store.set(name, items);

  let Ok(()) = result else {
    return Response::builder()
      .status(500)
      .body("Error storing wishlist")
      .build();
  };

  // Return an HTTP status of 201 ACCEPTED with no response body

  Response::builder().status(201).build()
}
