# CroftSoft Advent of Spin 2024 Challenge 3 Solution

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/advent-of-spin/blob/main/LICENSE.txt

![Screenshot 2024-12-08](./media/screenshot-2024-12-08-a.jpg)

## Setup

- Install Dioxus
```
cargo install dioxus-cli
```
- Configure Spin
```
spin plugins update

spin plugins install cloud -v 0.10.0

spin plugins install --url \
  https://github.com/fermyon/spin-deps-plugin/releases/download/canary/spin-deps.json \
  -y  
```
- Configure Wasm
```
cargo add wit_bindgen

cargo add serde -F serde_derive

cargo install --locked wasm-tools
```
- Configure Python
```
cd 2024/challenge3/ai

python -m venv .venv

source .venv/bin/activate

pip install --upgrade pip

pip install -r requirements.txt

pip install componentize-py

deactivate

cd ..
```

## Build and Deploy

- Build and bundle the front-end
  - With Static Site Generation (SSG) and client-side hydration 
```
cd ui/

rm -rf static/

rm -rf target/

dx bundle --ssg
```
- Copy the generated files to the distribution directory
```
rm -rf public/

cp -r target/dx/ui/release/web/public/* public/

cp -r static/* public/

cd ..
```
- Make the Wasm component
```
cd ai

source .venv/bin/activate

componentize-py \
  -d ./wit/ \
  -w gift-suggestions-generator \
  componentize \
  -m spin_sdk=spin-imports \
  app \
  -o gift-suggestions-generator.wasm

deactivate

cd ..
```
- View the WIT
```
wasm-tools component wit ai/gift-suggestions-generator.wasm
```
- Add the dependency
```
spin deps add ai/gift-suggestions-generator.wasm
```
- When prompted
  - Select "?:?@0.1.0"
  - Select "?"
  - Select "challenge3"
- Generate the bindings
```
spin deps generate-bindings -L rust -o src/bindings -c challenge3
```
- Build
```
spin build
```
- Deploy to the Fermyon Spin cloud
```
spin deploy
```

## Automated Testing

- Start the server locally
```
spin up
```
- Use hurl to run automated tests
```
hurl --test test.hurl
```

## Links

- https://github.com/fermyon/advent-of-spin/tree/main/2024/Challenge-3

## History

- Initial release: 2024-12-10
