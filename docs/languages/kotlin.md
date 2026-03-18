# Kotlin

Using whichtime in Kotlin/Android/JVM applications.

## Installation

Kotlin bindings are currently a preview/source-build integration.

### Build from Source

```bash
cd common
./build-kotlin.sh
```

## Basic Usage

```kotlin
import works.transcode.whichtime.*

fun main() {
    // Parse with convenience function
    val results = parse("tomorrow at 3pm")
    results.forEach { result ->
        println("Found: ${result.text}")
        result.dateMillis?.let { millis ->
            val date = java.util.Date(millis)
            println("Date: $date")
        }
    }
    
    // Get just the first date
    parseDate("tomorrow at 3pm")?.let { timestamp ->
        val date = java.util.Date(timestamp)
        println("Date: $date")
    }
}
```

## Using the Parser Object

```kotlin
import works.transcode.whichtime.*

val parser = WhichTimeParser()

// Parse with specific locale
val results = parser.parse("demain matin", WhichTimeLocale.FR)

// Parse with English locale
val englishResults = parser.parseEn("tomorrow morning")

// Get first date with locale
parser.parseDate("morgen", WhichTimeLocale.DE)?.let { timestamp ->
    println("Timestamp: ${timestamp}ms")
}
```

## Working with Results

```kotlin
val results = parse("December 25, 2024 at 3pm")

results.firstOrNull()?.let { result ->
    // Access matched text and position
    println("Text: ${result.text}")
    println("Position: ${result.index}...${result.endIndex}")
    
    // Access components
    val c = result.start
    c.year?.let { println("Year: $it") }
    c.month?.let { println("Month: $it") }
    c.day?.let { println("Day: $it") }
    c.hour?.let { println("Hour: $it") }
    
    // Get as Date
    result.dateMillis?.let { millis ->
        val date = java.util.Date(millis)
        println("Date: $date")
    }
    
    // Check for range
    result.end?.let { end ->
        println("This is a range")
        end.day?.let { println("End day: $it") }
    }
}
```

## Error Handling

```kotlin
import works.transcode.whichtime.*

try {
    val results = parse("some text")
    if (results.isEmpty()) {
        println("No date found")
    }
} catch (e: WhichTimeException.LocaleNotFound) {
    println("Unknown locale: ${e.locale}")
} catch (e: WhichTimeException.InvalidDateTime) {
    println("Invalid date/time: ${e.message}")
} catch (e: WhichTimeException.ParseError) {
    println("Parse error: ${e.message}")
} catch (e: Exception) {
    println("Error: $e")
}
```

## Using Multiple Locales

```kotlin
val parser = WhichTimeParser()

val phrases = listOf(
    "tomorrow" to WhichTimeLocale.EN,
    "demain" to WhichTimeLocale.FR,
    "morgen" to WhichTimeLocale.DE,
    "mañana" to WhichTimeLocale.ES,
    "明日" to WhichTimeLocale.JA,
)

for ((phrase, locale) in phrases) {
    try {
        val results = parser.parse(phrase, locale)
        results.firstOrNull()?.dateMillis?.let { millis ->
            val date = java.util.Date(millis)
            println("$phrase ($locale): $date")
        }
    } catch (e: Exception) {
        println("$phrase: Error - $e")
    }
}
```

## Android Example

```kotlin
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import works.transcode.whichtime.*
import java.text.SimpleDateFormat
import java.util.*

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            DateParserScreen()
        }
    }
}

@Composable
fun DateParserScreen() {
    var input by remember { mutableStateOf("") }
    var parsedDate by remember { mutableStateOf<String?>(null) }
    
    Column(
        modifier = Modifier.padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        OutlinedTextField(
            value = input,
            onValueChange = { newValue ->
                input = newValue
                parsedDate = parseInput(newValue)
            },
            label = { Text("Enter a date expression") },
            modifier = Modifier.fillMaxWidth()
        )
        
        Text(
            text = parsedDate ?: "No date found",
            style = MaterialTheme.typography.bodyLarge
        )
    }
}

private fun parseInput(text: String): String? {
    return try {
        parseDate(text)?.let { millis ->
            val date = Date(millis)
            SimpleDateFormat("yyyy-MM-dd HH:mm", Locale.getDefault()).format(date)
        }
    } catch (e: Exception) {
        null
    }
}
```

## Java Interoperability

whichtime can be used from Java:

```java
import works.transcode.whichtime.*;
import java.util.Date;
import java.util.List;

public class Example {
    public static void main(String[] args) {
        WhichTimeParser parser = new WhichTimeParser();
        
        try {
            List<WhichTimeResult> results = parser.parse(
                "tomorrow at 3pm", 
                WhichTimeLocale.EN
            );
            
            for (WhichTimeResult result : results) {
                System.out.println("Found: " + result.getText());
                Long millis = result.getDateMillis();
                if (millis != null) {
                    Date date = new Date(millis);
                    System.out.println("Date: " + date);
                }
            }
        } catch (WhichTimeException e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
}
```

## API Reference

### Top-Level Functions

```kotlin
// Parse with English locale
fun parse(text: String): List<WhichTimeResult>
fun parseDate(text: String): Long?

// Parse with specific locale
fun parseWithLocale(text: String, locale: WhichTimeLocale): List<WhichTimeResult>
fun parseDateWithLocale(text: String, locale: WhichTimeLocale): Long?
```

### WhichTimeParser

```kotlin
class WhichTimeParser {
    fun parse(text: String, locale: WhichTimeLocale): List<WhichTimeResult>
    fun parseDate(text: String, locale: WhichTimeLocale): Long?
    fun parseEn(text: String): List<WhichTimeResult>
    fun parseDateEn(text: String): Long?
}
```

### WhichTimeResult

```kotlin
data class WhichTimeResult(
    val index: UInt,
    val endIndex: UInt,
    val text: String,
    val dateMillis: Long?,
    val start: ParsedComponents,
    val end: ParsedComponents?
)
```

### ParsedComponents

```kotlin
data class ParsedComponents(
    val year: Int?,
    val month: Int?,
    val day: Int?,
    val hour: Int?,
    val minute: Int?,
    val second: Int?,
    val millisecond: Int?,
    val weekday: Int?,
    val timezoneOffset: Int?
)
```

### WhichTimeLocale

```kotlin
enum class WhichTimeLocale {
    EN, DE, ES, FR, IT, JA, NL, PT, RU, SV, UK, ZH
}
```

