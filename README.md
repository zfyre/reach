# Reach - Research Assistant CLI Tool
Reach is a command-line tool designed to enhance academic research by integrating search capabilities from Google, arXiv, and Gemini to provide comprehensive information gathering, summarization, and knowledge graph generation.
Features

- 🔍 Multi-source Search: Integrated search across Google, arXiv, and Gemini
- 📝 Web Content Extraction: Automatically extracts relevant content from web pages
- 📊 Summarization: Generates concise summaries of research content
- 🔗 Knowledge Graph Generation: Creates knowledge graphs to visualize relationships between concepts
- ⚙️ Configurable: Customize search parameters, keywords, and categories

# Prerequisites

Rust (latest stable version)
Python 3.12 or higher
API keys:

Google Search API key
Google Custom Search Engine ID
Gemini API key



# Installation
1. Clone the repository
```bash
git clone https://github.com/yourusername/reach.git
cd reach
```
2. Set up Python environment
```bash
# Create virtual environment
python -m venv .venv

# Activate virtual environment (Windows)
.venv\Scripts\activate

# Activate virtual environment (Linux/macOS)
source .venv/bin/activate

# Install dependencies
pip install --upgrade pip
pip install -U crawl4ai
crawl4ai-setup
```
3. Build the Rust project
```bash
cargo build --release
```

# Configuration
1. Reach requires several API keys to function properly. You can configure these using the CLI:
```bash
# Launch the interactive configuration
cargo run -- config

# Or set each key individually
cargo run -- config --google-api-key YOUR_GOOGLE_API_KEY
cargo run -- config --search-engine-id YOUR_SEARCH_ENGINE_ID
cargo run -- config --gemini-api-key YOUR_GEMINI_API_KEY
```

2. You can customize your arXiv search preferences:
```bash
# Launch the interactive configuration
cargo run -- arxiv-config

# Show current configuration
cargo run -- arxiv-config --show
```
The arXiv configuration allows you to set:

- Keywords to include in searches
- Keywords to exclude from searches
- Specific authors to focus on
- arXiv categories to search within

# Usage
## Basic Search
```bash
# Search with default parameters
cargo run -- search "Diffusion Models"

# Specify result limit
cargo run -- search "Flow based Diffusion Models" --max-results 5
```
## Generate Summaries
```bash
cargo run -- summarize "What are Diffusion Models?"
```
## Generate Knowledge Graph
```bash
cargo run -- knowledge-graph "What are Flow based Diffusion Models?"
```

# Project Structure
```
Copyreach/
├── src/
│   ├── apis/               # API integrations (Google, Gemini, arXiv)
│   ├── config/             # Configuration handling
│   ├── rsearch/            # Research functionality
│   │   ├── knowledge_graph.rs  # Knowledge graph generation
│   │   └── utils.rs        # Utility functions
│   ├── scripts/            # Python scripts for web scraping
│   ├── display/            # Output formatting
│   └── errors/             # Error handling
├── .venv/                  # Python virtual environment
└── data/                   # Output data storage
    ├── summaries.json      # Generated summaries
    └── knowledge_graph.json # Generated knowledge graphs
```

# Development
## Running Tests
```bash
# Run all tests
cargo test

# Run tests that require configuration
cargo test --features requires_config
```

# GitHub Actions
The project includes GitHub Actions workflows that:
- Build the project
- Set up the Python environment
- Install dependencies
- Run tests

# License
[Specify your license here]

# Contributing
[Add contribution guidelines if applicable]
