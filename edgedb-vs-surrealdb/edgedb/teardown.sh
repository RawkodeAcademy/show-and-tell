#!/usr/bin/env bash
set -euo pipefail

command -v edgedb > /dev/null 2>&1 || {
    echo "edgedb not found in PATH. Please install from https://www.edgedb.com/docs/intro/cli."
    exit 1
}

edgedb instance destroy --non-interactive --instance show_and_tell --force
