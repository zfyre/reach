# Reach CLI Metadata

This module contains metadata and configuration constants used by the Reach CLI application.

## Overview

The metadata module provides centralized access to important constants used throughout the Reach CLI application. This includes author information, version numbers, and configuration file names.

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `AUTHOR` | `"Me <kshitiz4kaushik@gmail.com>"` | Author information for the CLI application |
| `VERSION` | `"1.0.0"` | Current version of the CLI application |
| `CONFIG_FILE` | `".reach-config"` | Name of the configuration file used by the application |

## Usage

These constants can be imported in other modules to maintain consistency across the application:

```rust
use metadata::{AUTHOR, VERSION, CONFIG_FILE};

fn print_app_info() {
    println!("Reach CLI v{}", VERSION);
    println!("Created by {}", AUTHOR);
    println!("Configuration stored in {}", CONFIG_FILE);
}
```

## Purpose

Centralizing these values in a dedicated metadata module offers several advantages:
- Single source of truth for important application constants
- Easy maintenance when values need to be updated
- Consistent access to metadata throughout the codebase
