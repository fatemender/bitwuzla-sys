#!/bin/sh
echo "create libbitwuzla-merged.a\naddlib libbitwuzla.a\naddlib lib/libbitwuzlabb.a\naddlib lib/libbitwuzlabv.a\naddlib lib/libbitwuzlals.a\nsave\nend" | ar -M
