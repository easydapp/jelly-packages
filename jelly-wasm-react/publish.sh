#!/bin/bash

pnpm i

pnpm run lint

rm -rf ./lib/

pnpm run package

# publish
npm publish --access public
