# Rsearch

A knowledge graph-based research tool that leverages AI to iteratively build and explore knowledge graphs from web content.

## Overview

Rsearch is designed to assist researchers by automatically creating knowledge graphs from web searches based on research queries. It uses external search APIs (Google Search) and LLM APIs (Gemini) to extract and structure information from the web.

## Core Features

- **Iterative Knowledge Graph Building**: Automatically builds knowledge graphs from web content through multiple iterations
- **Smart Query Refinement**: Generates refined search queries based on discovered relationships and concepts
- **Content Summarization**: Creates concise summaries of web content relevant to research queries
- **Relationship Extraction**: Identifies key concepts and their relationships from web content

## Module Structure

### Exposed Modules and Functions

The following are exposed through `lib.rs` for use by external crates:

- **`build_kg_iteratively`**: Main function to build a knowledge graph iteratively from a research query
- **`utils`**: Utility functions for handling web content and file operations
- **`RsearchError`**: Error type for Rsearch operations

### Internal Modules

- **`knowledge_graph`**: Core functionality for building and navigating knowledge graphs
- **`errors`**: Error handling for the Rsearch module
- **`utils`**: Utility functions for web scraping and file operations

## Dependencies

### Member Crates
- **`reachapi`**: Provides interfaces to external APIs (Google Search, Gemini)
  - Used for: Web searches, LLM queries, content extraction
- **`reachdb`**: Graph database implementation for storing and querying knowledge graphs
  - Used for: Creating and traversing the knowledge graph structure

### External Dependencies
- **`clap`**: Command-line argument parsing
- **`regex`**: Pattern matching for extracting relationships
- **`serde_json`**: JSON serialization/deserialization
- **`log`**: Logging infrastructure
- **`tokio`**: Asynchronous runtime for handling concurrent operations

## Usage

To build a knowledge graph based on a research query:

```rust
use reachdb::{Reachdb, UserDefinedRelationType};
use rsearch::build_kg_iteratively;

async fn example<T: UserDefinedRelationType>(db: &mut Reachdb<T>) {
    // Build a knowledge graph iteratively with:
    // - Initial query: "What are Diffusion Models?"
    // - 3 iterations
    // - Default file type filter ("")
    // - Random walk depth of 2
    // - Consider 3 queries for next iteration
    build_kg_iteratively(
        db,
        "What are Diffusion Models?",
        "",
        3,  // num_iter
        2,  // num_depth
        3,  // num_queries
    ).await.unwrap();
}
```

## API Key Requirements

The following API keys need to be configured:
- Google Search API key
- Google Search Engine ID
- Gemini API key

These should be configured via the `ApiConfig` module from `reachapi`.
