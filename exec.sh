#!/bin/bash

trap cleanup INT EXIT

cleanup() {
    [[ -d /proc/$serverPid ]]&& kill "$serverPid"
}

hash cargo || {
    printf 'Error: %s\n' "'cargo' not found in PATH" >&2
    
    exit 1
}

cargo run -rp dyrah_server &
serverPid=$!

sleep .5

cargo run -rp dyrah_client

wait "$serverPid"