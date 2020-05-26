#!/usr/bin/with-contenv sh

if [ -n "$HEALTHCHECK_ID" ]; then
	curl -sS -X POST -o /dev/null "https://hc-ping.com/$HEALTHCHECK_ID/start"
fi

# If gitout fails we want to avoid triggering the health check.
set -e

# shellcheck disable=SC2086
/app/gitout $GITOUT_ARGS /config/config.toml /data

if [ -n "$HEALTHCHECK_ID" ]; then
	curl -sS -X POST -o /dev/null --fail "https://hc-ping.com/$HEALTHCHECK_ID"
fi
