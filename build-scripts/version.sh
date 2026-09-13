#!/usr/bin/env bash
# Print the version of dysk, as declared in Cargo.toml.

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/_common.sh"
dysk_version
