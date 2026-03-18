# Contributing to whichtime

Thanks for your interest in improving `whichtime`. Contributions are welcome across the core parser, locale coverage, bindings, examples, benchmarks, and documentation.

## Code of Conduct

Please be respectful, constructive, and considerate in issues and pull requests. Assume good intent, give actionable feedback, and help keep the project welcoming to contributors of all experience levels.

## Development Setup

### Prerequisites

- Rust 1.85+ with the 2024 edition toolchain
- Xcode 15+ for Swift and iOS work
- Android NDK and JDK 17+ for Kotlin and Android work
- Python 3 for Python bindings and examples

### Clone and Build

```bash
git clone https://github.com/transcodeworks/whichtime.git
cd whichtime

# Build the core library
cargo build -p whichtime-sys

# Build all workspace crates
cargo build --workspace
```

### Generate FFI Bindings

The repo includes helper scripts under `common/` for language-specific bindings:

```bash
cd common

./build-ios.sh
./build-kotlin.sh
./build-python.sh
```

For the manual UniFFI flow, see `README.md` and the docs site in `docs/`.

## Code Style

- Run `cargo fmt --all` before opening a pull request.
- Run `cargo clippy --workspace --all-targets` and fix warnings that are relevant to your change.
- Keep public APIs and behavior documented when you add or change them.
- Prefer focused commits and small, reviewable pull requests over large mixed changes.

## Testing

Before submitting a change, run the tests that apply to the area you touched:

```bash
# Core parser tests
cargo test -p whichtime-sys

# Full workspace tests
cargo test --workspace

# Benchmarks for performance-sensitive changes
cargo bench -p whichtime-sys
```

Locale coverage tests live under `common/whichtime-sys/tests/` and are organized by locale, for example `en_tests.rs`, `de_tests.rs`, and `ja_tests.rs`. If you change locale-specific parsing behavior, add or update tests in the matching test file.

## Pull Request Process

- Use a short, descriptive branch name such as `docs/phase-1-readme` or `feat/ru-relative-parser`.
- Include tests for behavior changes whenever practical.
- Update documentation and examples when public APIs, bindings, or supported expressions change.
- Explain the user-facing impact and any parser edge cases in the PR description.
- Call out performance trade-offs if you touch scanning, dictionaries, component storage, or refiner ordering.

## Adding a New Locale

High-level workflow:

1. Add locale dictionaries in `common/whichtime-sys/src/dictionaries/<locale>.rs`.
2. Add locale parsers in `common/whichtime-sys/src/parsers/<locale>/`.
3. Register the locale in `common/whichtime-sys/src/dictionaries/mod.rs`.
4. Register parser and refiner configuration in `common/whichtime-sys/src/lib.rs`.
5. Add locale tests in `common/whichtime-sys/tests/<locale>_tests.rs`.
6. Add example and documentation updates if the locale is user-facing.

For the parser pipeline and extension points, see `docs/guide/architecture.md`.

## Adding a New Parser

High-level workflow:

1. Implement the `Parser` trait in the appropriate module under `common/whichtime-sys/src/parsers/`.
2. Keep `should_apply()` cheap so parsers can be skipped quickly.
3. Register the parser in the locale configuration in `common/whichtime-sys/src/lib.rs`.
4. Add targeted tests that cover matching, non-matching, ambiguity, and locale behavior.
5. Update documentation or examples if the parser adds a new supported expression.

If your parser produces adjacent or overlapping matches, double-check how it interacts with the refiner pipeline described in `docs/guide/architecture.md`.

## Performance Expectations

`whichtime` is intentionally optimized around:

- Aho-Corasick token scanning
- PHF-based locale dictionaries
- `FastComponents` bitflag-backed component storage

If you touch any of those paths, run `cargo bench -p whichtime-sys` and mention notable regressions or improvements in your PR. Prefer changes that keep parser prefilters cheap, avoid unnecessary allocation, and preserve the current parsing pipeline behavior unless the change explicitly intends to alter it.
