# Advent of Spin 2023 Challenge 1

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/spin-prototype/blob/main/LICENSE.txt

## Usage

- Install the Fermyon Spin command line utility "spin"
  - https://developer.fermyon.com/spin/v2/install
- spin build
- spin up
  - If running on Windows, you might have to do some workarounds
    - See https://github.com/fermyon/spin/issues/2112
    - Build spin-fileserver in a sibling directory
      - cd ../spin-fileserver
      - cargo build --release
      - cd ../spin-prototype
    - Update the path to spin_static_fs.wasm in spin.toml
    - spin up --direct-mounts

## Automated Testing

- Use hurl to run automated tests
```
hurl --test test.hurl
```

## Manual Testing

- Demonstrates using a static file server
- http://localhost:3000/
- From a Key-Value store example
  - Stores the value "helicopter" for the key "advent"
```
curl -i -X POST \
  -H "Content-Type: application/json" \
  -d "{\"value\":\"helicopter\"}" \
  localhost:3000/data?advent
```
- curl -i -X GET localhost:3000/data?advent
  - Retrieves the value "{\"value\":\"helicopter\"}" for the key "advent"
- curl -i -X DELETE localhost:3000/data?advent
  - Deletes the value for the key "advent"
- curl -i -X GET localhost:3000/data?advent
  - Fails to retrieve the deleted value for the key "advent"

## Links

- https://github.com/fermyon/advent-of-spin/tree/main/2023/Challenge-1
- https://github.com/fermyon/spin/tree/main/examples/rust-key-value

## History

- Initial release: 2023-12-04
