#!/bin/bash
set -e
if ! id dnea >/dev/null 2>&1; then
  adduser --system --group --home /var/lib/dnea --shell /usr/sbin/nologin dnea
fi
mkdir -p /var/lib/dnea
chown dnea:dnea /var/lib/dnea
chmod 750 /var/lib/dnea
sudo -u dnea /usr/lib/dnea/dnea-daemon --init-key --config /etc/dnea/dnea.toml || true

for dir in \
  /etc/opt/chrome/native-messaging-hosts \
  /etc/chromium/native-messaging-hosts \
  /etc/opt/edge/native-messaging-hosts \
  /usr/lib/mozilla/native-messaging-hosts; do
  mkdir -p "$dir"
  ext_prefix="chrome-extension"
  ext_id="abcdefghijklmnopabcdefghijklmnop"
  case "$dir" in
    *mozilla*) ext_prefix="moz-extension"; ext_id="dnea@dnea.org" ;;
  esac
  cat > "$dir/org.dnea.runtime.json" <<EOF
{
  "name": "org.dnea.runtime",
  "description": "DNEA runtime bridge",
  "path": "/usr/lib/dnea/dnea-nm-bridge",
  "type": "stdio",
  "allowed_origins": ["$ext_prefix://$ext_id/"],
  "x-dnea-installed": { "version": "1.0.0" }
}
EOF
done

systemctl daemon-reload
systemctl enable --now dnea.service || true

if [ -f /etc/default/dnea ] && grep -q "DNEA_POLICY=1" /etc/default/dnea; then
  mkdir -p /etc/opt/chrome/policies/managed /etc/chromium/policies/managed
  cat > /etc/opt/chrome/policies/managed/dnea.json <<EOF
{ "ExtensionInstallForcelist": ["abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx"] }
EOF
  cp /etc/opt/chrome/policies/managed/dnea.json /etc/chromium/policies/managed/dnea.json
  mkdir -p /etc/firefox/policies
  if [ ! -f /etc/firefox/policies/policies.json ]; then
    cat > /etc/firefox/policies/policies.json <<EOF
{ "policies": { "Extensions": { "Install": ["dnea@dnea.org"] } } }
EOF
  fi
fi
exit 0