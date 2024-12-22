#!/bin/bash

rm gift-suggestions-generator.wasm

# --stub-wasi
# -m spin_sdk=spin-imports

componentize-py \
  -d ./wit/world.wit \
  -w gift-suggestions-generator \
  componentize \
  --stub-wasi \
  app \
  -o gift-suggestions-generator.wasm
