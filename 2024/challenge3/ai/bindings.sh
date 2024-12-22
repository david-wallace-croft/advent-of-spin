#!/bin/bash

rm -rf gift-suggestions-generator/

componentize-py \
  -d ./wit/ \
  -w gift-suggestions-generator \
  componentize \
  -m spin_sdk=spin-imports \
  app \
  -o gift-suggestions-generator.wasm
