#!/bin/sh
set -e

export GIT_HASH=$(git rev-parse HEAD)
cargo build --release
