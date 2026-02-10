$ErrorActionPreference = "Stop"

$env:GIT_HASH = git rev-parse HEAD

cargo build --release