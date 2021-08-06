#! /bin/bash

TEST_START_FROM_CLIPPY_LINT_SRC="$TEST_START_FROM_CLIPPY_LINT_SRC"

# set -x
set -euo pipefail

if [[ $# -lt 1 || $# -ge 3 ]]; then
    echo "Usage: $(basename $0) CLIPPY_LINT_NAME [NEW_LINT_NAME]" >&2
    exit 1
fi

REV=ae72f1adb9cbf16141f880e9e955723a5fdabf00

CLIPPY_LOWER="$1"
if [[ $# -ge 2 ]]; then
    NEW_LOWER="$2"
else
    NEW_LOWER="$CLIPPY_LOWER"
fi

source "$(dirname "$(readlink -f $0)")"/names.sh

DST="$(dirname "$(readlink -f $0)")"/..

if [[ -n "$TEST_START_FROM_CLIPPY_LINT_SRC" ]]; then
    SRC="$TEST_START_FROM_CLIPPY_LINT_SRC"
else
    SRC="$(mktemp -d --tmpdir rust-clippy.XXXXXX)"
    git clone 'https://github.com/rust-lang/rust-clippy' "$SRC"
fi

cd "$SRC"
git checkout "$REV" --quiet

if [[ ! -f "clippy_lints/src/$CLIPPY_LOWER.rs" ]]; then
    echo "$0: could not find '$CLIPPY_LOWER'" >&2
    exit 1
fi

# Cargo.toml

sed -i "s/\<fill_me_in\>/$NEW_LOWER/g" "$DST/Cargo.toml"

# README.md

(
    echo "# $NEW_LOWER"
    echo
    cat "clippy_lints/src/$CLIPPY_LOWER.rs" |
    sed -n 's,^[[:space:]]*///[[:space:]]*\(.*\)$,\1,;T;p'
) > "$DST/README.md"

# src/lib.rs

sed -i "
    s/\<fill_me_in\>/$NEW_LOWER/g;
    s/\<FILL_ME_IN\>/$NEW_UPPER/g;
    s/\<FillMeIn\>/$NEW_CAMEL/g
" "$DST/src/lib.rs"

# src/fill_me_in.rs

cat "clippy_lints/src/$CLIPPY_LOWER.rs" |
sed "
    s/\<crate::consts\>/clippy_utils::consts/g
    s/\<crate::utils\>/clippy_utils/g
    s/\<declare_clippy_lint\>/declare_lint/g
    s/\<declare_tool_lint\>/declare_lint/g
    s/\<restriction\|pedantic\|style\|complexity\|correctness\|perf\|cargo\|nursery\>/Warn/g
    s/\<$CLIPPY_LOWER\>/$NEW_LOWER/g
    s/\<$CLIPPY_UPPER\>/$NEW_UPPER/g
    s/\<$CLIPPY_CAMEL\>/$NEW_CAMEL/g
" |
cat > "$DST/src/fill_me_in.rs"

mv "$DST/src/fill_me_in.rs" "$DST/src/$NEW_LOWER.rs"

# ui/main.rs

cat "tests/ui/$CLIPPY_LOWER.rs" |
sed "s/\<clippy::$CLIPPY_LOWER\>/$NEW_LOWER/g" |
cat > "$DST/ui/main.rs"

mv "$DST/ui/main.rs" "$DST/ui/$NEW_LOWER.rs"

# ui/main.stderr

cat "tests/ui/$CLIPPY_LOWER.stderr" |
sed "s/\<clippy::$CLIPPY_KEBAB\>/$NEW_KEBAB/g" |
cat > "$DST/ui/main.stderr"

mv "$DST/ui/main.stderr" "$DST/ui/$NEW_LOWER.stderr"
