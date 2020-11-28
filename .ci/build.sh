#!/bin/bash
set -exo pipefail

echo "starting build for TARGET $TARGET"

export CRATE_NAME=run-script

SUFFIX=""

echo "$TARGET" | grep -E '^x86_64-pc-windows-gnu$' >/dev/null && SUFFIX=".exe"


echo "$TARGET" | grep -E '^x86_64-unknown-netbsd$' >/dev/null && echo "$TARGET not supported" && exit 0

# build binary
cross +nightly build --target $TARGET --release

# to check how they are built
file "target/$TARGET/release/${CRATE_NAME}$SUFFIX"

# if this commit has a tag, upload artifact to release
strip "target/$TARGET/release/${CRATE_NAME}$SUFFIX" || true # if strip fails, it's fine
mkdir -p release
cp "target/$TARGET/release/${CRATE_NAME}$SUFFIX" "release/${CRATE_NAME}-$TARGET$SUFFIX"

echo 'build success!'
exit 0
