# CroftSoft Advent of Spin 2024 Challenge 2 Solution

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/advent-of-spin/blob/main/LICENSE.txt

![Screenshot 2024-12-08](./media/screenshot-2024-12-08-a.jpg)

## Build and Deploy

- Install the Command Line Interfaces (CLIs) for Dioxus and Fermyon Spin
- Build and bundle the front-end
```
cd ui/
dx bundle
```
- Compile the Wasm
```
cd string-reverse
npm install
npm run all
```
- Install wasm-tools
```
cargo install --locked wasm-tools
```
- View the WIT
```
wasm-tools component wit string-reverse/string-reverse.wasm
```
- Install the plugin
```
spin plugins install --url https://github.com/fermyon/spin-deps-plugin/releases/download/canary/spin-deps.json -y  
```
- Add the dependency
```
spin deps add string-reverse/string-reverse.wasm
```
- Generate the bindings
```
spin deps generate-bindings -L rust -o src/bindings -c challenge2
```
- Deploy to the Fermyon Spin cloud
```
cd ../
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
