#! /bin/bash

# set -x
set -euo pipefail

if [[ $# -ne 0 ]]; then
    echo "$0: expect no arguments" >&2
    exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
    echo "$0: repository contains uncommitted changes; please run in a clean repository" >&2
    exit 1
fi

RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RESET="\033[0m"

SRC="$(mktemp -d --tmpdir rust-clippy.XXXXXX)"
git clone 'https://github.com/rust-lang/rust-clippy' "$SRC"

curl 'https://rust-lang.github.io/rust-clippy/master/lints.json' |
jq -r '.[] .id' |
while read CLIPPY_LINT_NAME; do
    echo -n "$CLIPPY_LINT_NAME: "
    true &&
        (TEST_START_FROM_CLIPPY_LINT_SRC="$SRC" "$(dirname $0)"/start_from_clippy_lint.sh "$CLIPPY_LINT_NAME" 2>/dev/null ||
            (echo -e "${RED}start_from_clippy_lint.sh failed${RESET}" && false)) &&
        (cargo build 2>/dev/null ||
            (echo -e "${YELLOW}cargo build failed${RESET}" && false)) &&
        (echo -e "${GREEN}ok${RESET}")
    git checkout . --quiet
    git clean -dfx src ui --quiet
done
