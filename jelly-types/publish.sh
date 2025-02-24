#!/bin/bash

pnpm i

rm -rf ./dist/
rm -rf ./lib/

pnpm run build-wasm

pnpm run build

# publish
npm publish --access public
