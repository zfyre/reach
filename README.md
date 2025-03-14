# Reach - Research Assistant CLI Tool

Reach is a command-line tool designed to enhance academic research by integrating search capabilities from Google, arXiv, and Gemini to provide comprehensive information gathering, summarization, and knowledge graph generation.

## Features

- 🔍 **Multi-source Search**: Integrated search across Google, arXiv, and Gemini
- 📝 **Web Content Extraction**: Automatically extracts relevant content from web pages
- 📊 **Summarization**: Generates concise summaries of research content
- 🔗 **Knowledge Graph Generation**: Creates knowledge graphs to visualize relationships between concepts
- ⚙️ **Configurable**: Customize search parameters, keywords, and categories
- 🖥️ **Terminal UI**: Interactive terminal interface for query exploration and knowledge graph visualization
- 📚 **Graph Database**: High-performance graph database for storing relationship-based data

## Prerequisites

- **Rust** (latest stable version)
- **Python 3.12** or higher
- **API keys**:
  - Google Search API key
  - Google Custom Search Engine ID
  - Gemini API key

## Installation
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

## Configuration
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

## Usage
### Basic Search
```bash
# Search with default parameters
cargo run -- search "Diffusion Models"

# Specify result limit
cargo run -- search "Flow based Diffusion Models" --max-results 5
```
### Generate Summaries
```bash
cargo run -- summarize "What are Diffusion Models?"
```
### Generate Knowledge Graph
```bash
cargo run -- knowledge-graph "What are Flow based Diffusion Models?"
```

### Interactive Terminal UI
```bash
# Launch the terminal user interface
cargo run -- tui
```

The terminal UI provides:
- Multiple session management
- Interactive conversation history
- Visual knowledge graph representation
- Two operational modes: Query and Search/Knowledge Graph Building

#### TUI Controls
- `e` - Enter edit mode for text input
- `q` - Quit the application
- `t` - Toggle between Query and Search modes
- `n` - Create a new session
- `h` - Show/hide help
- `Left/Right` arrows - Navigate between sessions
- `Up/Down` arrows - Scroll through message history
- `Enter` - Submit input (when in edit mode)
- `Esc` - Exit edit mode

## Core Components

### ReachDB
ReachDB is a high-performance graph database implementation in Rust, designed for efficient storage and traversal of relationship-based data. It uses memory-mapped files for fast access to node and relationship records.

#### Key Features
- **Memory-mapped storage**: Fast access to node and relationship data
- **Bidirectional relationships**: Each relationship connects source and target nodes
- **User-defined relationship types**: Custom relationship semantics through generics
- **Efficient traversals**: Iterators for relationship traversal
- **Persistent storage**: Data remains on disk between sessions

#### Example Usage
```rust
// Define relationship types
#[derive(Debug)]
enum RelationType {
    IsA(u8),
    HasA(u8),
    DependsOn(u8)
}

// Implement the UserDefinedRelationType trait
impl UserDefinedRelationType for RelationType {
    // ... implementation ...
}

// Open or create a new database
let mut db = Reachdb::<RelationType>::open("data", Some(10000), Some(10000))?;

// Add edges (automatically creates nodes if they don't exist)
db.add_edge("Person", "Human", "IS-A")?;
db.add_edge("Person", "Arms", "HAS-A")?;
```

### ReachTUI
ReachTUI is built using [Ratatui](https://github.com/ratatui-org/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm) to create a responsive terminal UI that allows users to interact with the Reach knowledge graph system through multiple sessions.

#### Key Components
- **Multiple session management**: Work with different research topics in separate sessions
- **Two operational modes**: Query and Search/Knowledge Graph Building
- **Interactive conversation history**: View and navigate through past interactions
- **Visual knowledge graph representation**: See relationships between concepts
- **Action tracking**: Monitor actions performed during the research process

#### Layout
```
┌─────────────────────────────────────────────────────────────┐
│                         Sessions                            │
├────────────────────────────────────┬────────────────────────┤
│                                    │                        │
│    ┌──────────┐ ┌───────────────┐  │                        │
│    │          │ │               │  │                        │
│    │ Actions  │ │ Conversation  │  │   Knowledge Graph      │
│    │          │ │               │  │                        │
│    │          │ │               │  │                        │
│    └──────────┘ └───────────────┘  │                        │
│                                    │                        │
├────────────────────────────────────┴────────────────────────┤
│                       Input Field                           │
└─────────────────────────────────────────────────────────────┘
```

### Metadata Module
The metadata module provides centralized access to important constants used throughout the Reach CLI application, including:
- Author information
- Version numbers
- Configuration file names

## Project Structure
```
reach/
├── src/
│   ├── apis/                # API integrations (Google, Gemini, arXiv)
│   ├── config/              # Configuration handling
│   ├── metadata/            # Application metadata constants
│   │   └── README.md        # Metadata documentation
│   ├── reachdb/             # Graph database implementation
│   │   └── README.md        # ReachDB documentation
│   ├── rsearch/             # Research functionality
│   │   ├── knowledge_graph.rs  # Knowledge graph generation
│   │   └── utils.rs         # Utility functions
│   ├── reachtui/            # Terminal user interface components
│   │   └── README.md        # TUI documentation
│   ├── scripts/             # Python scripts for web scraping
│   ├── display/             # Output formatting
│   └── errors/              # Error handling
├── .venv/                   # Python virtual environment
└── data/                    # Output data storage
    ├── summaries.json       # Generated summaries
    └── knowledge_graph.json # Generated knowledge graphs
```

## Development
### Running Tests
```bash
# Run all tests
cargo test

# Run tests that require configuration
cargo test --features requires_config
```

## GitHub Actions
The project includes GitHub Actions workflows that:
- Build the project
- Set up the Python environment
- Install dependencies
- Run tests

## License
[Specify your license here]

## Contributing
[Add contribution guidelines if applicable]

## Author
Created by Me <kshitiz4kaushik@gmail.com>

## Version
Current version: 1.0.0
