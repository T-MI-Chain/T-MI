#!/bin/bash
BIN=./target/release/tmi
LIVE_WS=wss://rpc.tmi.io
LOCAL_WS=ws://localhost:9944

# Kill the tmi client before exiting
trap 'kill "$(jobs -p)"' EXIT

runtimes=(
  "westend"
  "kusama"
  "tmi"
)

for RUNTIME in "${runtimes[@]}"; do
  echo "[+] Checking runtime: ${RUNTIME}"

  release_transaction_version=$(
    git show "origin/release:runtime/${RUNTIME}/src/lib.rs" | \
      grep 'transaction_version'
  )

  current_transaction_version=$(
    grep 'transaction_version' "./runtime/${RUNTIME}/src/lib.rs"
  )

  echo "[+] Release: ${release_transaction_version}"
  echo "[+] Ours: ${current_transaction_version}"

  if [ ! "$release_transaction_version" = "$current_transaction_version" ]; then
    echo "[+] Transaction version for ${RUNTIME} has been bumped since last release."
    exit 0
  fi

  if [ "$RUNTIME" = 'tmi' ]; then
    LIVE_WS="wss://rpc.tmi.io"
  else
    LIVE_WS="wss://${RUNTIME}-rpc.tmi.io"
  fi

  # Start running the local tmi node in the background
  $BIN --chain="$RUNTIME-local"  &
  jobs

  changed_extrinsics=$(
    tmi-js-metadata-cmp "$LIVE_WS" "$LOCAL_WS" \
      | sed 's/^ \+//g' | grep -e 'idx: [0-9]\+ -> [0-9]\+'
  )

  if [ -n "$changed_extrinsics" ]; then
    echo "[!] Extrinsics indexing/ordering has changed in the ${RUNTIME} runtime! If this change is intentional, please bump transaction_version in lib.rs. Changed extrinsics:"
    echo "$changed_extrinsics"
    exit 1
  fi

  echo "[+] No change in extrinsics ordering for the ${RUNTIME} runtime"
  kill "$(jobs -p)"; sleep 5
done

