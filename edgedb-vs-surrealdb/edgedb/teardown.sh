#!/usr/bin/env bash
set -euo pipefail

SERVER_INSTANCE=${1:-show_and_tell}

command -v edgedb > /dev/null 2>&1 || {
    echo "edgedb not found in PATH. Please install from https://www.edgedb.com/docs/intro/cli."
    exit 1
}

edgedb instance destroy --non-interactive --instance $SERVER_INSTANCE --force
