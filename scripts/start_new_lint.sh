#! /bin/bash

# set -x
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $(basename $0) NEW_LINT_NAME" >&2
    exit 1
fi

CLIPPY_LOWER="fill_me_in"
NEW_LOWER="$1"

source "$(dirname "$(readlink -f $0)")"/names.sh

DST="$(dirname "$(readlink -f $0)")"/..

# Cargo.toml

sed -i "s/\<fill_me_in\>/$NEW_LOWER/g" "$DST/Cargo.toml"

# README.md

(
    echo "# $NEW_LOWER"
    echo
    cat "$DST/src/fill_me_in.rs" |
    sed -n 's,^[[:space:]]*///[[:space:]]*\(.*\)$,\1,;T;p'
) > "$DST/README.md"

# src/lib.rs

sed -i "
    s/\<fill_me_in\>/$NEW_LOWER/g;
    s/\<FILL_ME_IN\>/$NEW_UPPER/g;
    s/\<FillMeIn\>/$NEW_CAMEL/g
" "$DST/src/lib.rs"

# src/fill_me_in.rs

sed -i "
    s/\<crate::consts\>/clippy_utils::consts/g
    s/\<crate::utils\>/clippy_utils/g
    s/\<declare_clippy_lint\>/declare_lint/g
    s/\<declare_tool_lint\>/declare_lint/g
    s/\<restriction\|pedantic\|style\|complexity\|correctness\|perf\|cargo\|nursery\>/Warn/g
    s/\<$CLIPPY_LOWER\>/$NEW_LOWER/g
    s/\<$CLIPPY_UPPER\>/$NEW_UPPER/g
    s/\<$CLIPPY_CAMEL\>/$NEW_CAMEL/g
" "$DST/src/fill_me_in.rs"

mv "$DST/src/fill_me_in.rs" "$DST/src/$NEW_LOWER.rs"

# ui/main.rs

sed -i "s/\<clippy::$CLIPPY_LOWER\>/$NEW_LOWER/g" "$DST/ui/main.rs"

mv "$DST/ui/main.rs" "$DST/ui/$NEW_LOWER.rs"

# ui/main.stderr

sed -i "s/\<clippy::$CLIPPY_KEBAB\>/$NEW_KEBAB/g" "$DST/ui/main.stderr"

mv "$DST/ui/main.stderr" "$DST/ui/$NEW_LOWER.stderr"
