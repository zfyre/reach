# Reach-CLI (Rust-Search-CLI / Research-CLI)

**Reach-CLI** is a command-line tool written in Rust that lets you perform web searches using the Google Custom Search API and leverage Gemini LLM for enhanced search results. With a future vision of integrating an Arxive paper finder, this tool aims to streamline your research workflow right from the terminal.

## Features

- **Dual Search Modes**  
  - **Google Search:** Retrieve search results using the Google Custom Search API.  
  - **LLM Search:** Use Gemini LLM to generate search results based on your query.

- **Configurable API Keys**  
  Easily configure your API keys for:
  - Google Search API (`REACH_GOOGLE_SEARCH_API_KEY`)
  - Google Search Engine ID (`REACH_GOOGLE_SEARCH_ENGINE_ID`)
  - Gemini API (`REACH_GEMINI_API_KEY`)
  
  These are stored in a hidden configuration file (`.reach-config`) in your home directory.

- **Future Integrations**  
  Plans to include an Arxive paper finder and pipeline to refine queries and retrieve academic papers.

## Installation

1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/reach.git
   cd reach
   ```

2. **Build the Project Ensure you have Rust installed. Then build with Cargo:**
    ```bash
    cargo build --release
    ```

## Configuration

1. **Before using the tool, set up your API keys. Run the configuration subcommand to input your keys:**  
   ```bash
   cargo run -- config
   ```
2. **To view your current configuration:**
    ```bash
    cargo run -- config --show
    ```

## Usage

1. **Perform a Google Search:**
    ```bash
    cargo run -- "What is the meaning of life?"
    ```
    **Note:** To restrict the filetype, use the `--filetype` flag:
    ```bash
    cargo run -- "Rust async programming" --ftype doc
    ```

2. **Perform an LLM Search:**
    ```bash
    cargo run --llm "What is 'Attention is all you need' ?"
    ```

## Future Work
- **Arxive Integration:**
    - Incorporate a Python-based Arxive paper finder to enable a seamless pipeline for academic research.

- **Enhanced Query Refinement:**
    - Utilize Gemini LLM to refine user queries further and extract relevant content from search results.