#!/bin/bash

rm -rf bindings_guest

componentize-py \
  -d wit/world.wit \
  -w gift-suggestions-generator \
  bindings \
  bindings_guest
