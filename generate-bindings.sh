#!/usr/bin/env zsh

set -euo pipefail

# Regenerate UniFFI bindings for all supported languages and place them
# in their respective, git-tracked folders.
#
# Languages:
# - Swift:   swift/Sources/UniFFI (via common/build-ios.sh --ffi-only)
# - Kotlin:  kotlin/
# - Python:  python/
#
# JS/TS uses wasm-pack and the code in web/, which is managed separately.

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
COMMON_DIR="${ROOT_DIR}/common"

echo "=== Building Rust FFI library (release) ==="
(
  cd "${ROOT_DIR}"
  cargo build --release -p whichtime-ffi
)

echo
echo "=== Regenerating Swift bindings (swift/Sources/UniFFI) ==="
(
  cd "${ROOT_DIR}"
  ./common/build-ios.sh --ffi-only
)

echo
echo "=== Regenerating Kotlin bindings (kotlin/) ==="
(
  cd "${ROOT_DIR}"
  cargo run -p uniffi-bindgen --bin uniffi-bindgen -- \
    generate \
    --library "target/release/libwhichtime_ffi.dylib" \
    --language kotlin \
    --out-dir "${ROOT_DIR}/kotlin" \
    --config common/whichtime-ffi/src/uniffi.toml

  # Normalize generated filename to 'bindgen.kt' for easier imports.
  if [ -f "${ROOT_DIR}/kotlin/uniffi/whichtime/whichtime.kt" ]; then
    rm -f "${ROOT_DIR}/kotlin/uniffi/whichtime/bindgen.kt"
    mv "${ROOT_DIR}/kotlin/uniffi/whichtime/whichtime.kt" \
       "${ROOT_DIR}/kotlin/uniffi/whichtime/bindgen.kt"
  fi
)

echo
echo "=== Regenerating Python bindings (python/) ==="
(
  cd "${ROOT_DIR}"
  cargo run -p uniffi-bindgen --bin uniffi-bindgen -- \
    generate \
    --library "target/release/libwhichtime_ffi.dylib" \
    --language python \
    --out-dir "${ROOT_DIR}/python" \
    --config common/whichtime-ffi/src/uniffi.toml

  # Normalize generated filename to 'bindgen.py' for easier imports.
  if [ -f "${ROOT_DIR}/python/whichtime.py" ]; then
    rm -f "${ROOT_DIR}/python/bindgen.py"
    mv "${ROOT_DIR}/python/whichtime.py" "${ROOT_DIR}/python/bindgen.py"
  fi
)

echo
echo "All bindings regenerated."


