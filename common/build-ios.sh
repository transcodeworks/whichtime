#!/usr/bin/env zsh

set -e
set -u

# Build script for producing a libwhichtime-rs.xcframework and UniFFI Swift bindings.
# This is intentionally modeled after Ferrostar's common/build-ios.sh, but simplified
# for Whichtime and your current needs.
#
# Usage:
#   ./common/build-ios.sh           # Build iOS + macOS + simulator libs, UniFFI Swift, and XCFramework
#   ./common/build-ios.sh --release # Additionally zip the XCFramework and update Package.swift
#   ./common/build-ios.sh --ffi-only # Generate FFI bindings only (for local development)
#

release=false
ffi_only=false

for arg in "$@"
do
    case $arg in
        --release)
            release=true
            shift
            ;;
        --ffi-only)
            ffi_only=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Ensure we're in the repository root
cd "$(dirname "$0")/.."

fat_simulator_lib_dir="target/ios-simulator-fat/release"
fat_macos_lib_dir="target/macos-fat/release"

generate_ffi() {
  local basename="$1"
  echo "Generating Swift UniFFI bindings for ${basename}"
  
  mkdir -p target/uniffi-xcframework-staging

  if $ffi_only; then
    echo "Using native library for FFI generation"
    local lib_ext="so"
    if [[ "$(uname)" == "Darwin" ]]; then
      lib_ext="dylib"
    fi
    cargo run -p uniffi-bindgen --bin uniffi-bindgen -- generate \
      --library "target/release/lib${basename}.${lib_ext}" \
      --language swift \
      --out-dir target/uniffi-xcframework-staging \
      --config common/whichtime-ffi/src/uniffi.toml
  else
    echo "Using iOS library for FFI generation"
    cargo run -p uniffi-bindgen --bin uniffi-bindgen -- generate \
      --library "target/aarch64-apple-ios/release/lib${basename}.a" \
      --language swift \
      --out-dir target/uniffi-xcframework-staging \
      --config common/whichtime-ffi/src/uniffi.toml
  fi

  # Place generated Swift bindings in the swift/Sources/UniFFI tree
  mkdir -p swift/Sources/UniFFI
  mv target/uniffi-xcframework-staging/*.swift swift/Sources/UniFFI/

  # Generate headers and modulemap for XCFramework
  if ! $ffi_only; then
    cargo run -p uniffi-bindgen --bin uniffi-bindgen-swift -- \
      "target/aarch64-apple-ios/release/lib${basename}.a" \
      target/uniffi-xcframework-staging \
      --headers \
      --modulemap \
      --module-name "${basename}FFI" \
      --modulemap-filename "module.modulemap"
  fi
}

create_fat_simulator_lib() {
  local basename="$1"
  echo "Creating a fat simulator library for ${basename}"
  mkdir -p "${fat_simulator_lib_dir}"
  lipo -create \
    "target/x86_64-apple-ios/release/lib${basename}.a" \
    "target/aarch64-apple-ios-sim/release/lib${basename}.a" \
    -output "${fat_simulator_lib_dir}/lib${basename}.a"
}

create_fat_macos_lib() {
  local basename="$1"
  echo "Creating a fat macOS library for ${basename}"
  mkdir -p "${fat_macos_lib_dir}"
  lipo -create \
    "target/x86_64-apple-darwin/release/lib${basename}.a" \
    "target/aarch64-apple-darwin/release/lib${basename}.a" \
    -output "${fat_macos_lib_dir}/lib${basename}.a"
}

build_xcframework() {
  local basename="$1"
  echo "Generating XCFramework for ${basename}"
  rm -rf target/ios
  xcodebuild -create-xcframework \
    -library "target/aarch64-apple-ios/release/lib${basename}.a" -headers target/uniffi-xcframework-staging \
    -library "target/ios-simulator-fat/release/lib${basename}.a" -headers target/uniffi-xcframework-staging \
    -library "target/macos-fat/release/lib${basename}.a" -headers target/uniffi-xcframework-staging \
    -output "target/ios/lib${basename}-rs.xcframework"

  if $release; then
    echo "Building xcframework archive and updating Package.swift"
    ditto -c -k --sequesterRsrc --keepParent \
      "target/ios/lib${basename}-rs.xcframework" \
      "target/ios/lib${basename}-rs.xcframework.zip"
    local checksum
    checksum=$(swift package compute-checksum "target/ios/lib${basename}-rs.xcframework.zip")
    local version
    version=$(cargo metadata --format-version 1 | jq -r --arg pkg_name "$basename" '.packages[] | select(.name==$pkg_name) .version')

    # Update releaseTag and releaseChecksum in the root Package.swift
    sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1$version\2/g" Package.swift
    sed -i "" -E "s/(let releaseChecksum = \")[^\"]+(\")/\1$checksum\2/g" Package.swift
  fi
}

basename=whichtime_ffi

if $ffi_only; then
  echo "Building in FFI-only mode for the current platform"
  cargo build -p whichtime-ffi --lib --release
else
  echo "Building for iOS device"
  cargo build -p whichtime-ffi --lib --release --target aarch64-apple-ios
fi

generate_ffi "$basename"

if $ffi_only; then
  echo "FFI-only build completed. Skipping XCFramework generation."
  exit 0
fi

echo "Building for iOS simulators"
cargo build -p whichtime-ffi --lib --release --target aarch64-apple-ios-sim
cargo build -p whichtime-ffi --lib --release --target x86_64-apple-ios

echo "Building for macOS"
cargo build -p whichtime-ffi --lib --release --target aarch64-apple-darwin
cargo build -p whichtime-ffi --lib --release --target x86_64-apple-darwin

create_fat_simulator_lib "$basename"
create_fat_macos_lib "$basename"
build_xcframework "$basename"

echo ""
echo "=== Build Complete ==="
echo "Swift bindings: swift/Sources/UniFFI/"
echo "XCFramework: target/ios/lib${basename}-rs.xcframework"
echo ""
echo "Supported platforms:"
echo "  - iOS (arm64)"
echo "  - iOS Simulator (arm64, x86_64)"
echo "  - macOS (arm64, x86_64)"

