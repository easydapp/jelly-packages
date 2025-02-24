#!/bin/bash

pnpm i

rm -rf ./dist/
rm -rf ./lib/

pnpm run build-wasm

pnpm run dev
