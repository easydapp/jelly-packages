#!/bin/bash

pnpm i

pnpm run test
pnpm run lint

rm -rf ./lib/

pnpm run dev
