# Installation

This guide covers how to install whichtime for each supported platform.

## Rust

Add `whichtime` to your `Cargo.toml` for the recommended Rust API:

```toml
[dependencies]
whichtime = "0.1"
```

Then run:

```bash
cargo build
```

If you need the lower-level engine crate directly, install `whichtime-sys` instead:

```toml
[dependencies]
whichtime-sys = "0.1"
```

## Swift (iOS/macOS)

### Via SwiftPM

Add to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/transcodeworks/whichtime", from: "0.1.0")
]
```

Or in Xcode:
1. File → Add Package Dependencies
2. Enter: `https://github.com/transcodeworks/whichtime`
3. Add `WhichtimeCore` to your target

### Build from Source

```bash
cd common
./build-ios.sh
```

This produces:
- `target/ios/libwhichtime-rs.xcframework` — Universal framework for iOS and simulators
- `swift/Sources/UniFFI/WhichTime.swift` — Swift bindings

## Kotlin (Android/JVM)

Kotlin support is currently a preview/source-build integration.

### Build from Source

```bash
cd common
./build-kotlin.sh
```

This generates:
- Native libraries for your platform
- `kotlin/src/main/kotlin/works/transcode/whichtime/whichtime.kt` — Kotlin bindings

## Python

Python support is currently a preview/source-build integration.

### Build from Source

```bash
# Build the bindings
./common/build-python.sh

# Install in development mode
cd python
pip install -e .
```

This generates:
- `python/whichtime/whichtime.py` — Python bindings
- `python/whichtime/libwhichtime.dylib` — Native library (or `.so` on Linux)

## Verifying Installation

### Rust

```rust
use whichtime::WhichTime;

fn main() {
    let parser = WhichTime::new();
    println!("whichtime is ready!");
}
```

### Swift

```swift
import WhichtimeCore

let parser = WhichTimeParser()
print("whichtime is ready!")
```

### Kotlin

```kotlin
import works.transcode.whichtime.WhichTimeParser

fun main() {
    val parser = WhichTimeParser()
    println("whichtime is ready!")
}
```

### Python

```python
from whichtime import WhichTimeParser

parser = WhichTimeParser()
print("whichtime is ready!")
```

## Troubleshooting

### Rust: Build Errors

Ensure you have Rust 2024 edition (1.85+):

```bash
rustup update
```

### Swift: Missing XCFramework

If Xcode can't find the framework, rebuild:

```bash
cd common
./build-ios.sh
```

### Python: Import Errors

Ensure the native library is in the correct location:

```bash
ls python/whichtime/
# Should show: whichtime.py, libwhichtime.dylib (or .so)
```

### Kotlin: UnsatisfiedLinkError

Ensure the native library is in your library path:

```bash
# macOS/Linux
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/path/to/libwhichtime

# Or place the library in a standard location
```

