# Advent of Spin

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/spin-prototype/blob/main/LICENSE.txt

## Installation for GitHub CodeSpaces

- Open a terminal
- Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustc --version
```
- Add the Wasm architecture
```
rustup target add wasm32-wasi
```
- Install Fermyon Spin
```
mkdir temp
cd temp
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
mkdir ~/bin
cp spin ~/bin/spin
cd ..
rm -rf temp
cd ~
source .profile
cd -
spin --version
```

## Links

- https://github.com/fermyon/advent-of-spin/tree/main/2023/Challenge-1

## History

- Initial release: 2023-12-04
