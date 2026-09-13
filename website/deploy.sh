#!/usr/bin/env bash
#
# This script is used to maintain dysk official doc at https://dystroy.org/dysk
#
# Obviously, it requires some rights over the server to be ran

set -Eeuo pipefail
cd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

command -v ddoc >/dev/null || { echo "ddoc not found — see https://dystroy.org/ddoc" >&2; exit 1; }

# build the site
ddoc

# deploy it on dystroy.org
scp -r site/* dys@dystroy.org:~/prod/www.dystroy.org/dysk/
