#!/bin/bash
set -e
if [ "$1" = "purge" ]; then
  rm -rf /var/lib/dnea
fi
# 30-day grace otherwise (systemd-tmpfiles handles this in real deployments).
exit 0