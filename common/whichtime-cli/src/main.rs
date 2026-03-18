use chrono::Utc;
use whichtime::WhichTime;

fn main() {
    // Collect all command-line arguments into a single string
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: whichtime <date string>");
        eprintln!("Example: whichtime tomorrow at noon");
        std::process::exit(1);
    }

    // Join all arguments after the program name into a single string
    let input = args[1..].join(" ");

    // Create a whichtime parser
    let parser = WhichTime::default();

    // Parse the input
    match parser.parse_date(&input, None) {
        Ok(Some(dt)) => {
            // Convert to UTC and format as ISO8601
            let utc_dt = dt.with_timezone(&Utc);
            println!("{}", utc_dt.to_rfc3339());
        }
        Ok(None) => {
            eprintln!("Error: Could not parse date from '{}'", input);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error parsing date: {}", e);
            std::process::exit(1);
        }
    }
}
