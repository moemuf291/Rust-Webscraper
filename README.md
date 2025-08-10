# Rust Web Scraper

A powerful and flexible command-line web scraper built in Rust with CSS selector support, multiple output formats, and respectful scraping practices.

## Features

- ✅ **Custom URL Input**: Specify any URL to scrape via CLI argument
- ✅ **CSS Selector Support**: Extract elements using CSS selectors (e.g., `h1`, `.price`, `a.link`)
- ✅ **Multiple Output Formats**: Plain text and JSON output
- ✅ **Comprehensive Error Handling**: Network errors, invalid selectors, empty results
- ✅ **Rate Limiting**: Configurable delays between requests
- ✅ **Custom User-Agent**: Set custom User-Agent headers
- ✅ **robots.txt Respect**: Automatically checks and respects robots.txt rules
- ✅ **Timeout Protection**: 30-second request timeout to prevent hanging

## Installation

### Prerequisites

First, install Rust on your system:

1. **Windows**: Download and run the installer from [rustup.rs](https://rustup.rs/)
2. **macOS/Linux**: Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Building the Project

```bash
# Clone or navigate to the project directory
cd webscraper

# Build the project
cargo build --release

# The executable will be available at target/release/webscraper.exe (Windows) or target/release/webscraper (Unix)
```

## Usage

### Basic Usage

```bash
# Scrape all h1 elements from a website
cargo run -- --url "https://example.com" --selector "h1"

# Or using the built executable
./target/release/webscraper --url "https://example.com" --selector "h1"
```

### Advanced Usage

```bash
# Scrape with JSON output
cargo run -- -u "https://news.ycombinator.com" -s ".titleline > a" -f json

# Scrape with custom delay and user-agent
cargo run -- -u "https://example.com" -s ".price" -d 2000 --user-agent "MyBot/1.0"

# Ignore robots.txt (use responsibly!)
cargo run -- -u "https://example.com" -s "p" --ignore-robots
```

### Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--url` | `-u` | URL to scrape (required) | - |
| `--selector` | `-s` | CSS selector (required) | - |
| `--format` | `-f` | Output format: `text` or `json` | `text` |
| `--delay` | `-d` | Delay between requests (ms) | `1000` |
| `--user-agent` | - | Custom User-Agent header | `webscraper/0.1.0 (Rust)` |
| `--ignore-robots` | - | Ignore robots.txt rules | `false` |

## Examples

### Example 1: Scraping News Headlines

```bash
# Scrape Hacker News titles
cargo run -- -u "https://news.ycombinator.com" -s ".titleline > a" -f json
```

### Example 2: Scraping Product Information

```bash
# Scrape product prices (example selector)
cargo run -- -u "https://example-shop.com/products" -s ".price" -d 2000
```

### Example 3: Scraping Links

```bash
# Extract all links from a page
cargo run -- -u "https://example.com" -s "a[href]" -f json
```

## Output Formats

### Text Format (Default)
```
=== Web Scraping Results ===
URL: https://example.com
Selector: h1
Timestamp: 2025-08-10T04:14:08.123Z
Found 3 element(s):

--- Element 1 ---
Text: Welcome to Example.com
Attributes:
  class: main-title
  id: header

--- Element 2 ---
Text: About Us
```

### JSON Format
```json
{
  "url": "https://example.com",
  "selector": "h1",
  "results": [
    {
      "text": "Welcome to Example.com",
      "attributes": {
        "class": "main-title",
        "id": "header"
      }
    }
  ],
  "timestamp": "2025-08-10T04:14:08.123Z"
}
```

## Error Handling

The scraper handles various error conditions gracefully:

- **Network Errors**: Connection timeouts, DNS failures, etc.
- **HTTP Errors**: 404, 500, and other HTTP status codes
- **Invalid URLs**: Malformed URL syntax
- **Invalid CSS Selectors**: Syntax errors in selectors
- **Empty Results**: No elements matching the selector
- **robots.txt Violations**: Warns about disallowed paths

## Rate Limiting & Ethical Scraping

- Default 1-second delay between requests
- Configurable delay via `--delay` option
- Respects robots.txt by default
- Uses identifiable User-Agent header
- 30-second timeout to avoid hanging connections

## CSS Selector Examples

| Selector | Description |
|----------|-------------|
| `h1` | All h1 elements |
| `.price` | Elements with class "price" |
| `#main` | Element with id "main" |
| `a[href]` | All links with href attribute |
| `.product .title` | Elements with class "title" inside class "product" |
| `div > p` | Paragraphs directly inside divs |
| `[data-id]` | Elements with data-id attribute |

## Dependencies

- `clap`: Command-line argument parsing
- `reqwest`: HTTP client
- `scraper`: HTML parsing and CSS selector support
- `serde`: Serialization framework
- `serde_json`: JSON serialization
- `tokio`: Async runtime
- `url`: URL parsing and validation
- `robotstxt`: robots.txt parsing
- `anyhow`: Error handling
- `chrono`: Date and time handling

## License

This project is open source. Use responsibly and respect website terms of service and robots.txt files.

## Contributing

Feel free to submit issues and enhancement requests!
