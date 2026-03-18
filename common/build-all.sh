#!/usr/bin/env zsh

set -e
set -u

# Master build script for WhichTime FFI bindings.
#
# Usage:
#   ./common/build-all.sh                    # Build all (Swift, Kotlin, Python)
#   ./common/build-all.sh --swift            # Build Swift/iOS only
#   ./common/build-all.sh --kotlin           # Build Kotlin only
#   ./common/build-all.sh --python           # Build Python only
#   ./common/build-all.sh --swift --kotlin   # Build specific targets

build_swift=false
build_kotlin=false
build_python=false
build_all=true

for arg in "$@"
do
    case $arg in
        --swift)
            build_swift=true
            build_all=false
            shift
            ;;
        --kotlin)
            build_kotlin=true
            build_all=false
            shift
            ;;
        --python)
            build_python=true
            build_all=false
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# If no specific targets were selected, build all
if $build_all; then
    build_swift=true
    build_kotlin=true
    build_python=true
fi

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "========================================"
echo "WhichTime FFI Bindings Build"
echo "========================================"
echo ""

if $build_swift; then
    echo ">>> Building Swift/iOS bindings..."
    "$SCRIPT_DIR/build-ios.sh" --ffi-only
    echo ""
fi

if $build_kotlin; then
    echo ">>> Building Kotlin bindings..."
    "$SCRIPT_DIR/build-kotlin.sh"
    echo ""
fi

if $build_python; then
    echo ">>> Building Python bindings..."
    "$SCRIPT_DIR/build-python.sh"
    echo ""
fi

echo "========================================"
echo "Build Complete!"
echo "========================================"
echo ""
echo "Generated bindings:"
if $build_swift; then
    echo "  - Swift:  swift/Sources/UniFFI/"
fi
if $build_kotlin; then
    echo "  - Kotlin: kotlin/src/main/kotlin/"
fi
if $build_python; then
    echo "  - Python: python/whichtime/"
fi
