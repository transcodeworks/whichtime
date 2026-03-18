import Foundation

@main
struct WhichTimeExample {
  static func main() {
    print("=== WhichTime Swift Example ===\n")

    // Example 1: Simple parsing
    print("1. Parsing 'tomorrow at 3pm':")
    do {
      let results = try parse(text: "tomorrow at 3pm")
      for result in results {
        print("   Text: '\(result.text)'")
        print("   Index: \(result.index) - \(result.endIndex)")
        if let millis = result.dateMillis {
          let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
          print("   Date: \(date)")
        }
        print()
      }
    } catch {
      print("   Error: \(error)")
    }

    // Example 2: Different locales
    print("2. Parsing with different locales:")
    let parser = WhichTimeParser()

    let phrases: [(String, WhichTimeLocale)] = [
      ("next Monday", .en),
      ("demain matin", .fr),
      ("mañana", .es),
    ]

    for (phrase, locale) in phrases {
      print("   '\(phrase)' (\(locale)):", terminator: " ")
      do {
        let results = try parser.parse(text: phrase, locale: locale)
        if let first = results.first, let millis = first.dateMillis {
          let date = Date(timeIntervalSince1970: Double(millis) / 1000.0)
          print("-> \(date)")
        } else {
          print("-> No date found")
        }
      } catch {
        print("-> Error: \(error)")
      }
    }

    // Example 3: Get timestamp
    print("\n3. Getting Unix timestamp:")
    do {
      if let timestamp = try parseDate(text: "December 25, 2024") {
        print("   'December 25, 2024' -> \(timestamp)ms")
      }
    } catch {
      print("   Error: \(error)")
    }

    // Example 4: Accessing parsed components
    print("\n4. Accessing parsed components:")
    do {
      let results = try parse(text: "March 15, 2025 at 2:30pm")
      if let result = results.first {
        print("   Components:")
        if let year = result.start.year { print("     Year: \(year)") }
        if let month = result.start.month { print("     Month: \(month)") }
        if let day = result.start.day { print("     Day: \(day)") }
        if let hour = result.start.hour { print("     Hour: \(hour)") }
        if let minute = result.start.minute { print("     Minute: \(minute)") }
      }
    } catch {
      print("   Error: \(error)")
    }

    print("\n=== Done ===")
  }
}
