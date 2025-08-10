use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

#[derive(Serialize)]
struct ScrapedData {
    url: String,
    selector: String,
    results: Vec<ScrapedElement>,
    timestamp: String,
}

#[derive(Serialize)]
struct ScrapedElement {
    text: String,
    attributes: HashMap<String, String>,
}

struct ScraperConfig {
    url: String,
    selector: String,
    output_format: String,
    delay_ms: u64,
    user_agent: String,
    respect_robots: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("webscraper")
        .version("0.1.0")
        .about("A flexible web scraper with CSS selector support")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("The URL to scrape")
                .required(true),
        )
        .arg(
            Arg::new("selector")
                .short('s')
                .long("selector")
                .value_name("CSS_SELECTOR")
                .help("CSS selector to extract elements (e.g., 'h1', '.price', 'a.link')")
                .required(true),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format: 'text' or 'json'")
                .default_value("text"),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("MILLISECONDS")
                .help("Delay between requests in milliseconds")
                .default_value("1000"),
        )
        .arg(
            Arg::new("user-agent")
                .long("user-agent")
                .value_name("USER_AGENT")
                .help("Custom User-Agent header")
                .default_value("webscraper/0.1.0 (Rust)"),
        )
        .arg(
            Arg::new("ignore-robots")
                .long("ignore-robots")
                .help("Ignore robots.txt rules")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config = ScraperConfig {
        url: matches.get_one::<String>("url").unwrap().clone(),
        selector: matches.get_one::<String>("selector").unwrap().clone(),
        output_format: matches.get_one::<String>("format").unwrap().clone(),
        delay_ms: matches
            .get_one::<String>("delay")
            .unwrap()
            .parse()
            .unwrap_or(1000),
        user_agent: matches.get_one::<String>("user-agent").unwrap().clone(),
        respect_robots: !matches.get_flag("ignore-robots"),
    };

    match scrape_website(&config).await {
        Ok(data) => {
            output_results(&data, &config.output_format)?;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn scrape_website(config: &ScraperConfig) -> Result<ScrapedData> {
    // Validate URL
    let parsed_url = Url::parse(&config.url)
        .map_err(|_| anyhow!("Invalid URL format: {}", config.url))?;

    // Check robots.txt if enabled
    if config.respect_robots {
        if let Err(e) = check_robots_txt(&parsed_url, &config.user_agent).await {
            eprintln!("Warning: {}", e);
        }
    }

    // Create HTTP client with custom User-Agent
    let client = Client::builder()
        .user_agent(&config.user_agent)
        .timeout(Duration::from_secs(30))
        .build()?;

    // Add delay before request
    if config.delay_ms > 0 {
        sleep(Duration::from_millis(config.delay_ms)).await;
    }

    // Fetch the webpage
    println!("Fetching: {}", config.url);
    let response = client
        .get(&config.url)
        .send()
        .await
        .map_err(|e| anyhow!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "HTTP error: {} - {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown")
        ));
    }

    let html_content = response
        .text()
        .await
        .map_err(|e| anyhow!("Failed to read response body: {}", e))?;

    // Parse HTML
    let document = Html::parse_document(&html_content);

    // Parse CSS selector
    let selector = Selector::parse(&config.selector)
        .map_err(|_| anyhow!("Invalid CSS selector: {}", config.selector))?;

    // Extract elements
    let mut results = Vec::new();
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        
        let mut attributes = HashMap::new();
        for (name, value) in element.value().attrs() {
            attributes.insert(name.to_string(), value.to_string());
        }

        if !text.is_empty() || !attributes.is_empty() {
            results.push(ScrapedElement { text, attributes });
        }
    }

    if results.is_empty() {
        return Err(anyhow!(
            "No elements found matching selector '{}' on {}",
            config.selector,
            config.url
        ));
    }

    Ok(ScrapedData {
        url: config.url.clone(),
        selector: config.selector.clone(),
        results,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

async fn check_robots_txt(url: &Url, user_agent: &str) -> Result<()> {
    let robots_url = format!("{}://{}/robots.txt", url.scheme(), url.host_str().unwrap_or(""));
    
    let client = Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(10))
        .build()?;

    match client.get(&robots_url).send().await {
        Ok(response) if response.status().is_success() => {
            let robots_content = response.text().await?;
            
            // Simple robots.txt parsing - check for Disallow rules
            let lines: Vec<&str> = robots_content.lines().collect();
            let mut relevant_user_agent = false;
            let mut disallowed_paths = Vec::new();

            for line in lines {
                let line = line.trim();
                if line.starts_with("User-agent:") {
                    let agent = line.split(':').nth(1).unwrap_or("").trim();
                    relevant_user_agent = agent == "*" || agent.to_lowercase() == user_agent.to_lowercase();
                } else if relevant_user_agent && line.starts_with("Disallow:") {
                    let path = line.split(':').nth(1).unwrap_or("").trim();
                    if !path.is_empty() {
                        disallowed_paths.push(path);
                    }
                }
            }

            // Check if the current URL path is disallowed
            let url_path = url.path();
            for disallowed in &disallowed_paths {
                if url_path.starts_with(disallowed) {
                    return Err(anyhow!(
                        "Access to {} is disallowed by robots.txt (rule: Disallow: {})",
                        url_path,
                        disallowed
                    ));
                }
            }

            println!("✓ robots.txt check passed");
        }
        Ok(_) => {
            println!("⚠ robots.txt not found or inaccessible, proceeding anyway");
        }
        Err(_) => {
            println!("⚠ Could not fetch robots.txt, proceeding anyway");
        }
    }

    Ok(())
}

fn output_results(data: &ScrapedData, format: &str) -> Result<()> {
    match format.to_lowercase().as_str() {
        "json" => {
            let json_output = serde_json::to_string_pretty(data)?;
            println!("{}", json_output);
        }
        "text" | _ => {
            println!("=== Web Scraping Results ===");
            println!("URL: {}", data.url);
            println!("Selector: {}", data.selector);
            println!("Timestamp: {}", data.timestamp);
            println!("Found {} element(s):\n", data.results.len());

            for (i, element) in data.results.iter().enumerate() {
                println!("--- Element {} ---", i + 1);
                if !element.text.is_empty() {
                    println!("Text: {}", element.text);
                }
                if !element.attributes.is_empty() {
                    println!("Attributes:");
                    for (key, value) in &element.attributes {
                        println!("  {}: {}", key, value);
                    }
                }
                println!();
            }
        }
    }

    Ok(())
}
