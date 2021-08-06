#! /bin/bash

CLIPPY_UPPER="${CLIPPY_LOWER^^}"
NEW_UPPER="${NEW_LOWER^^}"

camelize() {
    UNDERSCORE=1
    while read -n1 X; do
        if [[ "$X" = '_' ]]; then
            UNDERSCORE=1
        elif [[ -n "$UNDERSCORE" ]]; then
            echo -n "${X^}"
            UNDERSCORE=
        else
            echo -n "$X"
        fi
    done
}

CLIPPY_CAMEL="$(echo -n "$CLIPPY_LOWER" | camelize)"
NEW_CAMEL="$(echo -n "$NEW_LOWER" | camelize)"

CLIPPY_KEBAB="$(echo -n "$CLIPPY_LOWER" | tr '_' '-')"
NEW_KEBAB="$(echo -n "$NEW_LOWER" | tr '_' '-')"
