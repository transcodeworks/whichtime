# WhichTime Swift Example

This example shows how to use the generated Swift bindings on macOS or iOS-oriented projects.

## Install Options

### SwiftPM

Use the root `Package.swift` when integrating the project as a package dependency.

### Local XCFramework

Build the framework locally:

```bash
./common/build-ios.sh
```

That produces the XCFramework and generated Swift sources used by Xcode-based consumers.

## Run the Example

From the repository root:

```bash
./examples/swift/run-example.sh
```

The script builds the bindings, compiles `Example.swift`, and runs it against the local shared library.

## API at a Glance

- `parse(text:)` parses with the default English locale
- `parseDate(text:)` returns the first parsed date as Unix milliseconds
- `WhichTimeParser()` creates a reusable parser object
- `parser.parse(text:locale:)` parses with an explicit `WhichTimeLocale`

## Example Snippet

```swift
import Foundation

let results = try parse(text: "tomorrow at 3pm")
let parser = WhichTimeParser()
let frenchResults = try parser.parse(text: "demain matin", locale: .fr)

for result in results {
    if let millis = result.dateMillis {
        let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
        print(date)
    }
}
```

## Error Handling

The Swift APIs are `throws`, so wrap calls in `do` / `catch`:

```swift
do {
    let results = try parse(text: "tomorrow at 3pm")
    print(results.count)
} catch {
    print("whichtime failed: \(error)")
}
```

If the example fails before runtime, rebuild the local bindings with `./common/build-ios.sh --ffi-only` and rerun the script.

## Learn More

- [Installation guide](../../docs/guide/installation.md)
- [Getting started guide](../../docs/guide/getting-started.md)
- [Swift language docs](../../docs/languages/swift.md)
