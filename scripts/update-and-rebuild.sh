#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
MANTRACHAIN_REV=${1:-main}

LATEST_MANTRACHAIN_VERSION="v5.0.0"

# if "$MANTRACHAIN_REV" is /v\d+/ then extract it as var
if [[ "$MANTRACHAIN_REV" =~ ^v[0-9]+ ]]; then
  MANTRACHAIN_VERSION="$MANTRACHAIN_REV"
else
  MANTRACHAIN_VERSION="$LATEST_MANTRACHAIN_VERSION"
fi

####################################
## Update and rebuild mantrachain-std ##
####################################

# update revision in proto-build main.rs
PROTO_BUILD_MAIN_RS="$SCRIPT_DIR/../packages/proto-build/src/main.rs"

# use @ as a separator to avoid confusion on input like "origin/main"
sed -i -- "s@const MANTRACHAIN_REV: \&str = \".*\";@const MANTRACHAIN_REV: \&str = \"$MANTRACHAIN_VERSION\";@g" "$PROTO_BUILD_MAIN_RS"

git diff

# rebuild mantrachain-std
cd "$SCRIPT_DIR/../packages/proto-build/" && cargo run
