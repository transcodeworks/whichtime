#!/usr/bin/env bash
set -e

# WhichTime Swift Example Runner
#
# This script builds and runs the Swift example.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "=== Building WhichTime with UniFFI ==="
./common/build-ios.sh --ffi-only

echo ""
echo "=== Compiling Swift Example ==="

SWIFT_BINDINGS="swift/Sources/UniFFI/WhichTime.swift"
HEADER_DIR="target/uniffi-xcframework-staging"
LIB_DIR="target/release"
EXAMPLE_FILE="$SCRIPT_DIR/Example.swift"

# Check if files exist
if [[ ! -f "$SWIFT_BINDINGS" ]]; then
    echo "Error: Swift bindings not found at $SWIFT_BINDINGS"
    echo "Run: ./common/build-ios.sh --ffi-only"
    exit 1
fi

if [[ ! -f "$LIB_DIR/libwhichtime_ffi.dylib" ]]; then
    echo "Error: Library not found at $LIB_DIR/libwhichtime_ffi.dylib"
    exit 1
fi

if [[ ! -f "$EXAMPLE_FILE" ]]; then
    echo "Error: Example file not found at $EXAMPLE_FILE"
    exit 1
fi

# Compile and run
swiftc \
    -import-objc-header "$HEADER_DIR/whichtimeFFI.h" \
    -L "$LIB_DIR" \
    -lwhichtime_ffi \
    -o /tmp/whichtime_example \
    "$SWIFT_BINDINGS" \
    "$EXAMPLE_FILE"

echo ""
echo "=== Running Example ==="
DYLD_LIBRARY_PATH="$LIB_DIR" /tmp/whichtime_example
