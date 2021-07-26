#!/usr/bin/env bash
#
# Script to retrieve the latest proto specifications.
# Version of the proto can be changed with LC_PROTO_VESION variable.

set -eu
set -o pipefail

LC_PROTO_VESION="v0.3.1"

LC_PROTO_URL="https://github.com/limpidchart/lc-proto.git"
PROTOS_SRC_DIR="render"
PROTOS_TMP_DST_DIR="./proto-tmp"
PROTOS_DST_DIR="./proto"
NOTE_FILE="README.md"

# Prepare tmp dir.
rm -rf "${PROTOS_TMP_DST_DIR}"
mkdir "${PROTOS_TMP_DST_DIR}"

# Prepare dst dir.render
rm -rf "${PROTOS_DST_DIR}"
mkdir "${PROTOS_DST_DIR}"

# Clone the needed lc-proto version.
git clone --depth 1 --branch "${LC_PROTO_VESION}" "${LC_PROTO_URL}" "${PROTOS_TMP_DST_DIR}"

# Copy only needed files from lc-proto.
cp -r "${PROTOS_TMP_DST_DIR}/${PROTOS_SRC_DIR}" "${PROTOS_DST_DIR}"

# Create a note about source of created protos.
cat << 'EOF' > "${PROTOS_DST_DIR}/${NOTE_FILE}"
# lc-proto
Those files were retrieved from [github.com/limpidchart/lc-proto](https://github.com/limpidchart/lc-proto).

Please use `./scripts/get_lc_proto.sh` if you need to re-download or update those definitions.
EOF

# Cleanup.
rm -rf "${PROTOS_TMP_DST_DIR}"
