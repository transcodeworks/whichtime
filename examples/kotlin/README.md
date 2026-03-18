# WhichTime Kotlin Example

This example demonstrates the Kotlin/JVM bindings generated from the Rust core library.

## Gradle Setup

Build the bindings first:

```bash
./common/build-kotlin.sh
```

The example Gradle project already includes:

- JDK 17 toolchain
- JNA for loading the native library
- the generated source directory at `../../kotlin/src/main/kotlin`
- `jna.library.path` pointing to `../../target/release`

## Run the Example

```bash
cd examples/kotlin
./gradlew run
```

## API at a Glance

- `parse(text)` parses with the default English locale
- `parseDate(text)` returns the first parsed date as Unix milliseconds
- `WhichTimeParser()` creates a reusable parser object
- `parser.parse(text, locale)` parses with an explicit `WhichTimeLocale`

## Example Snippet

```kotlin
import works.transcode.whichtime.*

val results = parse("tomorrow at 3pm")
val parser = WhichTimeParser()
val germanResults = parser.parse("nächsten Montag", WhichTimeLocale.DE)

results.forEach { result ->
    println(result.text)
}
```

## Error Handling

The generated bindings can throw binding-level exceptions, so wrap parser calls in `try` / `catch`:

```kotlin
try {
    val results = parse("tomorrow at 3pm")
    println(results.size)
} catch (error: Exception) {
    println("whichtime failed: $error")
}
```

If the example cannot find the native library, rerun `./common/build-kotlin.sh` and confirm `target/release` contains the platform library used by `jna.library.path`.

## Android Note

For Android work, build and package the native libraries for your target ABI, then wire them into `jniLibs`. The example project is focused on JVM usage, but the generated Kotlin bindings are shared with Android builds.

## Learn More

- [Installation guide](../../docs/guide/installation.md)
- [Getting started guide](../../docs/guide/getting-started.md)
- [Kotlin language docs](../../docs/languages/kotlin.md)

