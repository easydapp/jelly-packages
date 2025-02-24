#!/bin/bash

pnpm i

pnpm run test
pnpm run lint

rm -rf ./dist/
rm -rf ./lib/

pnpm run build-wasm

pnpm run dev
