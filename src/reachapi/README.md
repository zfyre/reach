# ReachAPI

ReachAPI is a Rust library that provides simple interfaces to interact with research and information APIs. It includes functionality for searching academic papers, using AI services, and performing web searches.

## Features

- **ArXiv API Integration**: Search for academic papers using customizable filters
- **Google Search API Integration**: Perform web searches programmatically
- **Gemini API Integration**: Interact with Google's Gemini AI model
- **Unified Configuration System**: Manage API keys and search preferences

## Usage

Add ReachAPI to your Cargo.toml:

```toml
[dependencies]
reachapi = { path = "path/to/reachapi" }
```

### Searching ArXiv

```rust
use reachapi::{arxive_search, RawOuts};

async fn search_papers() -> Result<(), Box<dyn std::error::Error>> {
    // Search for papers with "Machine Learning" and limit to 5 results
    let results = arxive_search(Some("Machine Learning"), "5").await?;
    
    for result in results {
        match result {
            RawOuts::RawArxivOut(paper) => {
                println!("Title: {}", paper.title);
                println!("URL: {}", paper.url);
                println!("Summary: {}", paper.summary);
                println!("---");
            },
            _ => {}
        }
    }
    
    Ok(())
}
```

## Configuration

ReachAPI uses a configuration system with a `.reach_config` file stored in the user's home directory.

### API Configuration

Set up your API keys for Google Search, Search Engine ID, and Gemini:

```rust
use reachapi::{ApiConfig, ReachConfig};

fn configure_apis() -> Result<(), Box<dyn std::error::Error>> {
    ApiConfig::get_config_from_user()?;
    Ok(())
}
```

### ArXiv Configuration

Set up your ArXiv search preferences:

```rust
use reachapi::{ArxivConfig, ReachConfig};

fn configure_arxiv() -> Result<(), Box<dyn std::error::Error>> {
    ArxivConfig::get_config_from_user()?;
    Ok(())
}
```

## Error Handling

ReachAPI provides a unified error handling system via the `ReachApiError` enum, which can represent:

- I/O errors
- Network request errors
- Serialization/deserialization errors

Example error handling:

```rust
use reachapi::{arxive_search, ReachApiError};

async fn handle_errors() {
    match arxive_search(Some("Deep Learning"), "10").await {
        Ok(results) => {
            // Process results
        },
        Err(e) => match e {
            ReachApiError::IoError(io_err) => eprintln!("I/O Error: {}", io_err),
            ReachApiError::NetworkError(req_err) => eprintln!("Network Error: {}", req_err),
            ReachApiError::SerializationError(serde_err) => eprintln!("Serialization Error: {}", serde_err),
        }
    }
}
```

## Configuration Traits

ReachAPI defines two main configuration traits:

- `ReachConfigKeys`: For defining configuration keys
- `ReachConfig`: For implementing configuration handling

These traits can be implemented for custom configurations if needed.

## Data Types

### RawOuts

The `RawOuts` enum represents different types of API responses:

```rust
pub enum RawOuts {
    RawGeminiOut(String),
    RawArxivOut(ArxivOutput),
    RawGoogleOut((String, String)),
}
```

### ArxivOutput

Represents an ArXiv paper search result:

```rust
pub struct ArxivOutput {
    pub title: String,
    pub url: String,
    pub summary: String,
}
```
