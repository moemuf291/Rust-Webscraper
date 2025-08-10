#!/bin/bash

# Example usage scripts for the Rust Web Scraper
# Make sure to build the project first: cargo build --release

echo "=== Rust Web Scraper Examples ==="
echo

# Example 1: Basic scraping with text output
echo "Example 1: Scraping h1 elements from example.com"
cargo run -- --url "https://example.com" --selector "h1"
echo

# Example 2: JSON output
echo "Example 2: Scraping with JSON output"
cargo run -- --url "https://httpbin.org/html" --selector "h1" --format json
echo

# Example 3: Scraping with custom delay
echo "Example 3: Scraping with 2-second delay"
cargo run -- --url "https://httpbin.org/html" --selector "p" --delay 2000
echo

# Example 4: Custom User-Agent
echo "Example 4: Using custom User-Agent"
cargo run -- --url "https://httpbin.org/user-agent" --selector "body" --user-agent "MyCustomBot/1.0"
echo

echo "=== Examples completed ==="
