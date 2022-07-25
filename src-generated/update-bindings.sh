#!/bin/sh

bindgen \
    --output bindings.rs \
    --whitelist-function '^bitwuzla_(.*)$' \
    --whitelist-type '^Bitwuzla(.*)$' \
    --no-recursive-whitelist \
    --no-doc-comments \
    --raw-line 'use libc::{FILE, size_t};' \
    --no-prepend-enum-name \
    ../bitwuzla/src/api/c/bitwuzla.h
