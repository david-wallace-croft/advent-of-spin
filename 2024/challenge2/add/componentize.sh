#!/bin/bash

jco componentize \
    ./add.js \
    --wit ./add.wit \
    --world-name example \
    --out add.wasm
#    --disable all