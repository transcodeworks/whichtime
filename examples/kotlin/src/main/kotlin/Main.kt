/**
 * WhichTime Kotlin Example
 *
 * This example demonstrates how to use the WhichTime library from Kotlin.
 *
 * To run this example:
 *     1. Build the UniFFI bindings: ./common/build-kotlin.sh
 *     2. Run: cd examples/kotlin && ./gradlew run
 */

package works.transcode.whichtime.example

import works.transcode.whichtime.*
import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter

fun millisToDateTime(millis: Long): String {
    val instant = Instant.ofEpochMilli(millis)
    val formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss")
        .withZone(ZoneId.systemDefault())
    return formatter.format(instant)
}

fun main() {
    println("=== WhichTime Kotlin Example ===\n")

    // Example 1: Simple parsing with the convenience function
    println("1. Parsing 'tomorrow at 3pm':")
    try {
        val results = parse("tomorrow at 3pm")
        for (result in results) {
            println("   Text: '${result.text}'")
            println("   Index: ${result.index} - ${result.endIndex}")
            result.dateMillis?.let { millis ->
                println("   Date: ${millisToDateTime(millis)}")
            }
            println("   Components: year=${result.start.year}, month=${result.start.month}, day=${result.start.day}")
            println()
        }
    } catch (e: WhichTimeException) {
        println("   Error: $e")
    }

    // Example 2: Using the parser object with different locales
    println("2. Parsing with different locales:")
    val parser = WhichTimeParser()

    val testPhrases = listOf(
        "next Monday" to WhichTimeLocale.EN,
        "demain matin" to WhichTimeLocale.FR,      // tomorrow morning (French)
        "nächsten Montag" to WhichTimeLocale.DE,   // next Monday (German)
        "mañana" to WhichTimeLocale.ES,            // tomorrow (Spanish)
        "明日" to WhichTimeLocale.JA,               // tomorrow (Japanese)
        "завтра" to WhichTimeLocale.RU,            // tomorrow (Russian)
    )

    for ((phrase, locale) in testPhrases) {
        print("   '$phrase' ($locale): ")
        try {
            val results = parser.parse(phrase, locale)
            if (results.isNotEmpty()) {
                val first = results[0]
                first.dateMillis?.let { millis ->
                    println("-> ${millisToDateTime(millis)}")
                } ?: println("-> Components: ${first.start}")
            } else {
                println("-> No date found")
            }
        } catch (e: WhichTimeException) {
            println("-> Error: $e")
        }
    }

    // Example 3: Getting just the date as a timestamp
    println("\n3. Getting date as Unix timestamp:")
    try {
        val timestamp = parseDate("December 25, 2024")
        timestamp?.let {
            println("   'December 25, 2024' -> ${it}ms (${millisToDateTime(it)})")
        }
    } catch (e: WhichTimeException) {
        println("   Error: $e")
    }

    // Example 4: Parsing various date formats
    println("\n4. Parsing various date formats:")
    val testInputs = listOf(
        "in 2 hours",
        "last Friday",
        "3 days ago",
        "next week",
        "January 15th, 2025",
        "2024-12-25",
        "5:30pm",
        "noon tomorrow",
    )

    for (text in testInputs) {
        try {
            val results = parse(text)
            if (results.isNotEmpty()) {
                val first = results[0]
                first.dateMillis?.let { millis ->
                    println("   '$text' -> ${millisToDateTime(millis)}")
                } ?: run {
                    val components = first.start
                    val parts = mutableListOf<String>()
                    components.year?.let { parts.add("year=$it") }
                    components.month?.let { parts.add("month=$it") }
                    components.day?.let { parts.add("day=$it") }
                    components.hour?.let { parts.add("hour=$it") }
                    components.minute?.let { parts.add("minute=$it") }
                    println("   '$text' -> ${parts.joinToString(", ").ifEmpty { "parsed" }}")
                }
            } else {
                println("   '$text' -> No date found")
            }
        } catch (e: WhichTimeException) {
            println("   '$text' -> Error: $e")
        }
    }

    // Example 5: Parsing date ranges
    println("\n5. Parsing date ranges:")
    try {
        val results = parse("from Monday to Friday")
        for (result in results) {
            println("   Text: '${result.text}'")
            println("   Start: day=${result.start.day}, weekday=${result.start.weekday}")
            result.end?.let { end ->
                println("   End: day=${end.day}, weekday=${end.weekday}")
            }
        }
    } catch (e: WhichTimeException) {
        println("   Error: $e")
    }

    println("\n=== Done ===")
}

