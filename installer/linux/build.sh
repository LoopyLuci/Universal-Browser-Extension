#!/bin/bash
set -euo pipefail
cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
mkdir -p build/out
nfpm package --config installer/linux/nfpm.yaml --packager deb --target build/out/
nfpm package --config installer/linux/nfpm.yaml --packager rpm --target build/out/
ls -la build/out/