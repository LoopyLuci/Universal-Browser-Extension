#!/bin/bash
set -e
for dir in \
  /etc/opt/chrome/native-messaging-hosts \
  /etc/chromium/native-messaging-hosts \
  /etc/opt/edge/native-messaging-hosts \
  /usr/lib/mozilla/native-messaging-hosts; do
  manifest="$dir/org.dnea.runtime.json"
  if [ -f "$manifest" ] && ! grep -q "x-dnea-installed" "$manifest"; then
    echo "Refusing to overwrite foreign host manifest at $manifest" >&2
    exit 1
  fi
done
exit 0