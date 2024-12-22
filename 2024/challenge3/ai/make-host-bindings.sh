#!/bin/bash

rm -rf bindings_host/

python3 -m wasmtime.bindgen \
  gift-suggestions-generator.wasm \
  --out-dir bindings_host
