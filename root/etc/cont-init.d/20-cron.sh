#!/usr/bin/with-contenv sh

if [ -z "$CRON" ]; then
	echo "
Not running in cron mode
"
	exit 0
fi

if [ ! -d /data ]; then
	echo "
ERROR: '/data' directory must be mounted
"
	exit 1
fi
if [ ! -d /config ]; then
	echo "
ERROR: '/config' directory must be mounted
"
	exit 1
fi
if [ ! -f /config/config.toml ]; then
	echo "
ERROR: '/config/config.toml' file must exist
"
	exit 1
fi

# Set up the cron schedule.
echo "
Initializing cron

$CRON
"
echo "$CRON /app/gitout /config/config.toml /data" >/tmp/crontab.tmp
crontab -u abc /tmp/crontab.tmp
rm /tmp/crontab.tmp
