#!/bin/bash

pnpm i

rm -rf ./lib/

pnpm run build

# publish
npm publish --access public
