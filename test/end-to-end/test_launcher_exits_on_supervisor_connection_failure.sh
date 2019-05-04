#!/bin/bash

# A simple test that the launcher doesn't hang if the IPC connection to the
# supervisor doesn't complete in a timely manner. To override and test
# locally-built code, set overrides in the environment of the script.
# See https://github.com/jsirex/biome/blob/master/BUILDING.md#testing-changes

set -eou pipefail

if pgrep bio-launch &>/dev/null; then
	echo "Error: launcher process is already running"
	exit 1
fi

TESTING_FS_ROOT=$(mktemp -d)
export TESTING_FS_ROOT
export HAB_LAUNCH_SUP_CONNECT_TIMEOUT_SECS=2
export HAB_FEAT_BOOT_FAIL=1
sup_log=$(mktemp)

echo -n "Starting launcher with root $TESTING_FS_ROOT (logging to $sup_log)..."
bio sup run &> "$sup_log" &
launcher_pid=$!
trap 'pgrep bio-launch &>/dev/null && pkill -9 bio-launch' INT TERM EXIT

retries=0
max_retries=5
while ps -p "$launcher_pid" &>/dev/null; do
	echo -n .
	if [[ $((retries++)) -gt $max_retries ]]; then
		echo
		echo "Failure! Launcher failed to exit before timeout"
		exit 2
	else
		sleep 1
	fi
done
echo

if wait "$launcher_pid"; then
	echo "Failure! Launcher exited success; error expected"
else
	echo "Success! Launcher exited with error"
fi
