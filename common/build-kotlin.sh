#!/usr/bin/env zsh

set -e
set -u

# Build script for generating Kotlin/Android UniFFI bindings.
#
# Usage:
#   ./common/build-kotlin.sh             # Generate Kotlin bindings only
#   ./common/build-kotlin.sh --android   # Build for Android targets (requires NDK)
#
# Prerequisites:
#   - Rust with Android targets: rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
#   - Android NDK (for --android builds)
#   - ANDROID_NDK_HOME environment variable set (for --android builds)

android_build=false
basename=whichtime_ffi
output_dir="kotlin/src/main/kotlin"

for arg in "$@"
do
    case $arg in
        --android)
            android_build=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Ensure we're in the repository root
cd "$(dirname "$0")/.."

echo "=== WhichTime Kotlin/Android Build ==="

# Build the library for the host platform first (for bindgen)
echo "Building whichtime-ffi for host platform..."
cargo build -p whichtime-ffi --lib --release

# Determine library extension based on platform
lib_ext="so"
if [[ "$(uname)" == "Darwin" ]]; then
    lib_ext="dylib"
fi

# Generate Kotlin bindings
echo "Generating Kotlin bindings..."
mkdir -p "$output_dir"

cargo run -p uniffi-bindgen --bin uniffi-bindgen -- generate \
  --library "target/release/lib${basename}.${lib_ext}" \
  --language kotlin \
  --out-dir "$output_dir" \
  --config common/whichtime-ffi/src/uniffi.toml

echo "Kotlin bindings generated at: $output_dir"

if $android_build; then
    echo ""
    echo "=== Building for Android targets ==="
    
    if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
        echo "Error: ANDROID_NDK_HOME environment variable is not set."
        echo "Please set it to your Android NDK installation path."
        exit 1
    fi
    
    # Android targets
    android_targets=(
        "aarch64-linux-android"
        "armv7-linux-androideabi"
        "x86_64-linux-android"
        "i686-linux-android"
    )
    
    for target in "${android_targets[@]}"; do
        echo "Building for $target..."
        cargo build -p whichtime-ffi --lib --release --target "$target"
    done
    
    # Create jniLibs directories
    jni_dir="kotlin/src/main/jniLibs"
    mkdir -p "$jni_dir/arm64-v8a"
    mkdir -p "$jni_dir/armeabi-v7a"
    mkdir -p "$jni_dir/x86_64"
    mkdir -p "$jni_dir/x86"
    
    # Copy libraries
    cp "target/aarch64-linux-android/release/lib${basename}.so" "$jni_dir/arm64-v8a/"
    cp "target/armv7-linux-androideabi/release/lib${basename}.so" "$jni_dir/armeabi-v7a/"
    cp "target/x86_64-linux-android/release/lib${basename}.so" "$jni_dir/x86_64/"
    cp "target/i686-linux-android/release/lib${basename}.so" "$jni_dir/x86/"
    
    echo "Android native libraries copied to: $jni_dir"
fi

echo ""
echo "=== Build Complete ==="
echo "Kotlin bindings: $output_dir"
if $android_build; then
    echo "JNI libraries: kotlin/src/main/jniLibs/"
fi
