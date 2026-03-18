#!/usr/bin/env zsh

set -e
set -u

# Build script for generating Python UniFFI bindings.
#
# Usage:
#   ./common/build-python.sh           # Generate Python bindings and build native module
#   ./common/build-python.sh --wheel   # Additionally build a wheel package
#
# Prerequisites:
#   - Python 3.8+ with pip
#   - maturin (pip install maturin) for wheel building

build_wheel=false
basename=whichtime_ffi
output_dir="python/whichtime"

for arg in "$@"
do
    case $arg in
        --wheel)
            build_wheel=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Ensure we're in the repository root
cd "$(dirname "$0")/.."

echo "=== WhichTime Python Build ==="

# Build the FFI library
echo "Building whichtime-ffi..."
cargo build -p whichtime-ffi --lib --release

# Determine library extension based on platform
lib_ext="so"
if [[ "$(uname)" == "Darwin" ]]; then
    lib_ext="dylib"
elif [[ "$(uname)" == "MINGW"* ]] || [[ "$(uname)" == "CYGWIN"* ]]; then
    lib_ext="dll"
fi

# Generate Python bindings
echo "Generating Python bindings..."
mkdir -p "$output_dir"

cargo run -p uniffi-bindgen --bin uniffi-bindgen -- generate \
  --library "target/release/lib${basename}.${lib_ext}" \
  --language python \
  --out-dir "$output_dir" \
  --config common/whichtime-ffi/src/uniffi.toml

# Copy the native library (UniFFI Python bindings load it by this name)
echo "Copying native library..."
if [[ "$(uname)" == "Darwin" ]]; then
    cp "target/release/lib${basename}.dylib" "$output_dir/lib${basename}.dylib"
else
    cp "target/release/lib${basename}.${lib_ext}" "$output_dir/lib${basename}.${lib_ext}"
fi

# Create __init__.py if it doesn't exist
if [[ ! -f "$output_dir/__init__.py" ]]; then
    echo "Creating __init__.py..."
    cat > "$output_dir/__init__.py" << 'EOF'
"""WhichTime - Natural language date parsing library."""

from .whichtime import *

__version__ = "0.1.0"
__all__ = [
    "WhichTimeParser",
    "WhichTimeLocale",
    "WhichTimeResult",
    "ParsedComponents",
    "WhichTimeError",
    "parse",
    "parse_date",
    "parse_with_locale",
    "parse_date_with_locale",
]
EOF
fi

echo "Python bindings generated at: $output_dir"

if $build_wheel; then
    echo ""
    echo "=== Building Python Wheel ==="
    
    # Check if maturin is installed
    if ! command -v maturin &> /dev/null; then
        echo "maturin is not installed. Installing..."
        pip install maturin
    fi
    
    # Navigate to python directory and build
    cd python
    maturin build --release
    
    echo "Wheel built in: target/wheels/"
fi

echo ""
echo "=== Build Complete ==="
echo "Python module: $output_dir"
echo ""
echo "To use the Python bindings:"
echo "  cd python && pip install -e ."
echo "  python -c 'from whichtime import parse; print(parse(\"tomorrow at 3pm\"))'"
