#!/bin/bash

# search version.*"0\.0\. and change those 8 package version to next
# search jellypack.*0\.0\. and change all deps to next version

echo "========= 1. jelly-types ========="
cd jelly-types
sh ./publish.sh
cd -
echo ""

echo "========= 2. jelly-executor ========="
cd jelly-executor
cargo test && cargo clippy && cargo build
cd -
echo ""

echo "========= 3. jelly-model ========="
cd jelly-model
cargo test && cargo clippy && cargo build
cd -
echo ""

echo "========= 4. jelly-compiler ========="
cd jelly-compiler
sh ./publish.sh
cd -
echo ""

echo "========= 5. jelly-wasm ========="
cd jelly-wasm
cargo test && cargo clippy && cargo build
sh ./publish.sh
cd -
echo ""

echo "========= 6. jelly-runtime ========="
cd jelly-runtime
sh ./publish.sh
cd -
echo ""

echo "========= 7. jelly-wasm-api ========="
cd jelly-wasm-api
sh ./publish.sh
cd -
echo ""

echo "========= 8. jelly-wasm-react ========="
cd jelly-wasm-react
sh ./publish.sh
cd -
echo ""

echo "========= done ========="
say done
