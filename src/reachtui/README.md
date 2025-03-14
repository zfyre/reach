# ReachTUI - Terminal User Interface

This is the terminal user interface component for the Reach application. It provides an interactive console-based interface for querying and building knowledge graphs.

## Overview

ReachTUI is built using [Ratatui](https://github.com/ratatui-org/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm) to create a responsive terminal UI that allows users to interact with the Reach knowledge graph system through multiple sessions.

## Features

- Multiple session management
- Two operational modes: Query and Search/Knowledge Graph Building
- Interactive conversation history
- Visual knowledge graph representation
- Action tracking for each interaction

## Key Components

### Data Structures

#### `InputMode`
Controls the state of user input:
- `Normal`: For navigation and command input
- `Editing`: For text entry in the input field

#### `AppMode`
Determines the current operational mode:
- `Query`: For asking questions to the system
- `Search`: For building and exploring the knowledge graph

#### `Session`
Represents a user session with:
- `id`: Unique identifier
- `name`: Display name
- `conversation`: Vector of Message objects
- `knowledge_graph`: Data representation of the knowledge graph (currently a placeholder string)

#### `Message`
Represents a conversation entry:
- `sender`: Who sent the message ("User", "Assistant", "System")
- `action`: Optional description of action performed (shown in the actions panel)
- `content`: The actual message text

#### `App`
The main application state containing:
- Current input text and mode
- List of sessions and current session index
- Scroll position for messages
- Help visibility toggle
- Quit flag

### Main Functions

#### `App::new()`
Creates a new application instance with default settings and an empty first session.

#### `App::current_session()` and `App::current_session_mut()`
Accessors for the current session, returning references and mutable references respectively.

#### `App::send_message()`
Processes user input, adds it to the conversation, and generates appropriate responses based on the current mode.

#### `App::toggle_mode()`
Switches between Query and Search modes.

#### `App::new_session()`
Creates a new empty session and makes it the current session.

#### `run_app()`
The main event loop that handles user input, updates the UI, and manages the application state.

#### `ui()`
Renders the interface components based on the current application state, including:
- Session tabs at the top
- Action list on the left
- Conversation history in the center
- Knowledge graph on the right
- Input field at the bottom
- Optional help overlay

#### `setup_terminal()` and `restore_terminal()`
Handle terminal initialization and cleanup.

#### `centered_rect()`
Helper function to create centered rectangles for popup displays like the help menu.

## Controls

- `e` - Enter edit mode for text input
- `q` - Quit the application
- `t` - Toggle between Query and Search modes
- `n` - Create a new session
- `h` - Show/hide help
- `Left/Right` arrows - Navigate between sessions
- `Up/Down` arrows - Scroll through message history
- `Enter` - Submit input (when in edit mode)
- `Esc` - Exit edit mode

## Layout
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

## Customization

You can modify this codebase to integrate with your specific knowledge graph implementation by updating:

1. The `send_message()` method to connect to your actual LLM or knowledge processing system
2. The `Session` structure to store your knowledge graph data in a more structured format
3. The UI rendering in the `ui()` function to display your knowledge graph representation