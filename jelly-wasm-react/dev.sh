#!/bin/bash

pnpm i

pnpm run lint

rm -rf ./lib/

pnpm run dev

