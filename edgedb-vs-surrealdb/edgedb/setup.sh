#!/usr/bin/env bash
set -euo pipefail

SERVER_INSTANCE=${1:-show_and_tell}

command -v edgedb > /dev/null 2>&1 || {
    echo "edgedb not found in PATH. Please install from https://www.edgedb.com/docs/intro/cli."
    exit 1
}

edgedb project init --non-interactive --server-instance $SERVER_INSTANCE || {
    edgedb instance create --non-interactive $SERVER_INSTANCE
    edgedb migration apply --instance $SERVER_INSTANCE
}
