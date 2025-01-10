# CroftSoft Advent of Spin Solutions

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/advent-of-spin/blob/main/LICENSE.txt

## Links

- CroftSoft Advent of Spin 2023 Challenge Solutions
  - Solutions in the Rust programming language
  - Slideshow presentations describing the solutions
  - Includes a link to a YouTube video recording
  - https://github.com/david-wallace-croft/advent-of-spin/tree/main/2023
- CroftSoft Spin Prototype
  - https://github.com/david-wallace-croft/spin-prototype
- CroftSoft Advent of Spin 2024 Challenge Solutions
  - Slideshow presentation
  - https://github.com/david-wallace-croft/advent-of-spin/tree/main/2024

## Installation of Spin on GitHub CodeSpaces

- Open a terminal
- Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustc --version
```
- Add the Wasm architecture
```
rustup target add wasm32-wasip1
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
cd
source .profile
cd -
spin --version
```

## History

- Initial release: 2023-12-04
