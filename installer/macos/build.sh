#!/bin/bash
set -euo pipefail
VERSION="1.0.0"
STAGE="build/stage"
OUT="build/out"
rm -rf "$STAGE" "$OUT"
mkdir -p "$STAGE/Library/Application Support/DNEA/bin"
mkdir -p "$STAGE/Library/LaunchDaemons"
mkdir -p "$OUT"

cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
cp target/release/dnea-daemon "$STAGE/Library/Application Support/DNEA/bin/"
cp target/release/dnea-nm-bridge "$STAGE/Library/Application Support/DNEA/bin/"
cp target/release/dnea "$STAGE/Library/Application Support/DNEA/bin/"
cp installer/macos/org.dnea.runtime.plist "$STAGE/Library/LaunchDaemons/"

pkgbuild --root "$STAGE" --scripts installer/macos/scripts \
  --identifier org.dnea.runtime --version "$VERSION" \
  --install-location / "$OUT/dnea-component.pkg"

productbuild --package "$OUT/dnea-component.pkg" \
  --distribution installer/macos/distribution.xml \
  --resources installer/macos/resources "$OUT/dnea-$VERSION.pkg"

productsign --sign "Developer ID Installer: DNEA Project" \
  "$OUT/dnea-$VERSION.pkg" "$OUT/dnea-$VERSION-signed.pkg"