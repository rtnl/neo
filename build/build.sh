#!/bin/sh
set -e

export GIT_HASH=$(git rev-parse HEAD)
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
