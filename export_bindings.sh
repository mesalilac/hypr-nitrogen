#!/bin/sh

set -e

# cargo install --git https://github.com/Aleph-Alpha/ts-rs --branch feat/cli ts-rs-cli

cd ./src-tauri/

ts-rs export --index
