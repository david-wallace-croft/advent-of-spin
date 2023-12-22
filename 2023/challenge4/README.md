# Advent of Spin 2023 Challenge 4

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/advent-of-spin/blob/main/LICENSE.txt

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
hurl --error-format long --test test.hurl
```

## Links

- https://github.com/fermyon/advent-of-spin/tree/main/2023/Challenge-3

## History

- Initial release: 2023-12-22
