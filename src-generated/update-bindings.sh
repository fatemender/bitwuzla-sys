#!/bin/sh

bindgen \
    --output bindings.rs \
    --whitelist-function '^bitwuzla_(.*)$' \
    --whitelist-type '^Bitwuzla(.*)$' \
    --no-recursive-whitelist \
    --no-doc-comments \
    --raw-line 'use libc::{FILE, size_t};' \
    ../bitwuzla/src/api/c/bitwuzla.h

#    --whitelist-function '^boolector_(.*)$' \
#    --whitelist-type '^Btor(.*)$' \
#    --whitelist-type '^Boolector(.*)$' \
#    --blacklist-type '^BtorOpt(.*)$' \
#    --no-recursive-whitelist \
#    --raw-line 'use libc::FILE;' \
#    --no-doc-comments \
#    ../bitwuzla/src/api/c/bitwuzla.h

#bindgen \
#    --output options.rs \
#    --generate types \
#    --whitelist-type '^BtorOpt(.*)$' \
#    --blacklist-type '^BtorOpt$' \
#    --blacklist-type '^BtorOptHelp$' \
#    --no-recursive-whitelist \
#    --no-doc-comments \
#    --no-prepend-enum-name \
#    ../boolector/src/btoropt.h \
#    -- \
#    -I../boolector/src
