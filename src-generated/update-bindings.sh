#!/bin/sh

bindgen \
    --output bindings.rs \
    --allowlist-function '^bitwuzla_(.*)$' \
    --allowlist-type '^Bitwuzla(.*)$' \
    --no-recursive-allowlist \
    --no-doc-comments \
    --raw-line 'use libc::FILE;' \
    --no-prepend-enum-name \
    --opaque-type BitwuzlaSort \
    --opaque-type BitwuzlaTerm \
    ../bitwuzla/include/bitwuzla/c/bitwuzla.h \
    -- -I../bitwuzla/include
