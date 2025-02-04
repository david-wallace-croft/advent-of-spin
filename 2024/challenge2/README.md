# CroftSoft Advent of Spin 2024 Challenge 2 Solution

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/advent-of-spin/blob/main/LICENSE.txt

![Screenshot NavBar](./media/screenshot-navbar-2025-01-07-a.png)
![Screenshot NameForm](./media/screenshot-nameform-2025-01-07-a.png)

## Build and Deploy

- Install the Command Line Interfaces (CLIs) for Dioxus and Fermyon Spin
- Build and bundle the front-end
  - With Static Site Generation (SSG) and client-side hydration 
```
cd ui/
rm -rf target/dx
dx bundle --ssg
```
- Copy the SSG index.html files to the public/ directory
```
cp -r static/* target/dx/ui/release/web/public/
```
- Compile the Wasm
```
cd ..
cd calculator
npm install
npm run build
```
- Add wit_bindgen
```
cargo add wit_bindgen
```
- Install wasm-tools
```
cargo install --locked wasm-tools
```
- View the WIT
```
wasm-tools component wit calculator.wasm
```
- Install the plugin
```
spin plugins install --url https://github.com/fermyon/spin-deps-plugin/releases/download/canary/spin-deps.json -y  
```
- Add the dependency
```
cd ../
spin deps add calculator/calculator.wasm
```
- When prompted
  - Select "croftsoft:naughty-or-nice@0.1.0"
  - Select "calculator"
  - Select "challenge2"
- Generate the bindings
```
spin deps generate-bindings -L rust -o src/bindings -c challenge2
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

- https://github.com/fermyon/advent-of-spin/tree/main/2024/Challenge-2

## History

- Initial release: 2024-12-10
