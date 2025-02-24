#!/bin/bash

cargo test

cargo clippy

wasm-pack build --target web --release --scope jellypack
