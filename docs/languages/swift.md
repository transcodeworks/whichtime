# Swift

Using whichtime in Swift/iOS/macOS applications.

## Installation

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

### Building from Source

```bash
cd common
./build-ios.sh
```

This produces an XCFramework at `target/ios/libwhichtime_ffi-rs.xcframework`.

## Basic Usage

```swift
import WhichtimeCore

// Parse with convenience function
let results = try parse(text: "tomorrow at 3pm")
for result in results {
    print("Found: \(result.text)")
    if let millis = result.dateMillis {
        let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
        print("Date: \(date)")
    }
}

// Get just the first date
if let timestamp = try parseDate(text: "tomorrow at 3pm") {
    let date = Date(timeIntervalSince1970: Double(timestamp) / 1000.0)
    print("Date: \(date)")
}
```

## Using the Parser Object

```swift
import WhichtimeCore

let parser = WhichTimeParser()

// Parse with specific locale
let results = try parser.parse(text: "demain matin", locale: .fr)

// Parse with English locale
let englishResults = try parser.parseEn(text: "tomorrow morning")

// Get first date with locale
if let timestamp = try parser.parseDate(text: "morgen", locale: .de) {
    print("Timestamp: \(timestamp)ms")
}
```

## Working with Results

```swift
let results = try parse(text: "December 25, 2024 at 3pm")

if let result = results.first {
    // Access matched text and position
    print("Text: \(result.text)")
    print("Position: \(result.index)...\(result.endIndex)")
    
    // Access components
    let c = result.start
    if let year = c.year { print("Year: \(year)") }
    if let month = c.month { print("Month: \(month)") }
    if let day = c.day { print("Day: \(day)") }
    if let hour = c.hour { print("Hour: \(hour)") }
    
    // Get as Date
    if let millis = result.dateMillis {
        let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
        print("Date: \(date)")
    }
    
    // Check for range
    if let end = result.end {
        print("This is a range")
        if let endDay = end.day {
            print("End day: \(endDay)")
        }
    }
}
```

## Error Handling

```swift
do {
    let results = try parse(text: "some text")
    if results.isEmpty {
        print("No date found")
    }
} catch WhichTimeError.localeNotFound(let locale) {
    print("Unknown locale: \(locale)")
} catch WhichTimeError.invalidDateTime(let message) {
    print("Invalid date/time: \(message)")
} catch WhichTimeError.parseError(let message) {
    print("Parse error: \(message)")
} catch {
    print("Error: \(error)")
}
```

## Using Multiple Locales

```swift
let parser = WhichTimeParser()

let phrases: [(String, WhichTimeLocale)] = [
    ("tomorrow", .en),
    ("demain", .fr),
    ("morgen", .de),
    ("mañana", .es),
    ("明日", .ja),
]

for (phrase, locale) in phrases {
    do {
        let results = try parser.parse(text: phrase, locale: locale)
        if let first = results.first, let millis = first.dateMillis {
            let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
            print("\(phrase) (\(locale)): \(date)")
        }
    } catch {
        print("\(phrase): Error - \(error)")
    }
}
```

## SwiftUI Example

```swift
import SwiftUI
import WhichtimeCore

struct ContentView: View {
    @State private var input = ""
    @State private var parsedDate: Date?
    
    var body: some View {
        VStack(spacing: 20) {
            TextField("Enter a date expression", text: $input)
                .textFieldStyle(.roundedBorder)
                .onChange(of: input) { _, newValue in
                    parseInput(newValue)
                }
            
            if let date = parsedDate {
                Text("Parsed: \(date, style: .date) \(date, style: .time)")
            } else {
                Text("No date found")
                    .foregroundStyle(.secondary)
            }
        }
        .padding()
    }
    
    private func parseInput(_ text: String) {
        do {
            if let millis = try parseDate(text: text) {
                parsedDate = Date(timeIntervalSince1970: Double(millis) / 1000.0)
            } else {
                parsedDate = nil
            }
        } catch {
            parsedDate = nil
        }
    }
}
```

## API Reference

### Top-Level Functions

```swift
// Parse with English locale
func parse(text: String) throws -> [WhichTimeResult]
func parseDate(text: String) throws -> Int64?

// Parse with specific locale
func parseWithLocale(text: String, locale: WhichTimeLocale) throws -> [WhichTimeResult]
func parseDateWithLocale(text: String, locale: WhichTimeLocale) throws -> Int64?
```

### WhichTimeParser

```swift
class WhichTimeParser {
    init()
    
    func parse(text: String, locale: WhichTimeLocale) throws -> [WhichTimeResult]
    func parseDate(text: String, locale: WhichTimeLocale) throws -> Int64?
    func parseEn(text: String) throws -> [WhichTimeResult]
    func parseDateEn(text: String) throws -> Int64?
}
```

### WhichTimeResult

```swift
struct WhichTimeResult {
    let index: UInt32
    let endIndex: UInt32
    let text: String
    let dateMillis: Int64?
    let start: ParsedComponents
    let end: ParsedComponents?
}
```

### ParsedComponents

```swift
struct ParsedComponents {
    let year: Int32?
    let month: Int32?
    let day: Int32?
    let hour: Int32?
    let minute: Int32?
    let second: Int32?
    let millisecond: Int32?
    let weekday: Int32?
    let timezoneOffset: Int32?
}
```

### WhichTimeLocale

```swift
enum WhichTimeLocale {
    case en, de, es, fr, it, ja, nl, pt, ru, sv, uk, zh
}
```

