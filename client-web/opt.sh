#!/bin/bash

set -e

cd dist/pkg
wasm-opt client_web_bg.wasm -o client_web_bg.wasm -O
