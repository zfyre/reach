use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub enum InputMode {
    Normal,
    Editing,
}

pub enum AppMode {
    Query,
    Search,
}

pub struct Session {
    pub id: String,
    pub name: String,
    pub conversation: Vec<Message>,
    pub knowledge_graph: String, // Placeholder for knowledge graph data
}

pub struct Message {
    pub sender: String,
    pub content: String,
    pub action: Option<String>, // Optional action description to show on the left
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub app_mode: AppMode,
    pub sessions: Vec<Session>,
    pub current_session_index: usize,
    pub message_scroll: usize,
    pub show_help: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let default_session = Session {
            id: "default".to_string(),
            name: "New Session".to_string(),
            conversation: vec![],
            knowledge_graph: "".to_string(),
        };

        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            app_mode: AppMode::Query,
            sessions: vec![default_session],
            current_session_index: 0,
            message_scroll: 0,
            show_help: false,
            should_quit: false,
        }
    }

    pub fn current_session(&self) -> &Session {
        &self.sessions[self.current_session_index]
    }

    pub fn current_session_mut(&mut self) -> &mut Session {
        &mut self.sessions[self.current_session_index]
    }

    pub fn send_message(&mut self, content: String) {
        if content.trim().is_empty() {
            return;
        }
        
        let user_message = Message {
            sender: "User".to_string(),
            content: content.clone(),
            action: None,
        };
        
        self.current_session_mut().conversation.push(user_message);
        
        // This is where you would call your LLM or knowledge graph processing
        // For now, we'll just simulate a response
        match self.app_mode {
            AppMode::Query => {
                // Simulate LLM response to query
                let response_message = Message {
                    sender: "Assistant".to_string(),
                    content: format!("Processing query: {}", content),
                    action: Some("Query Processing".to_string()),
                };
                self.current_session_mut().conversation.push(response_message);
            }
            AppMode::Search => {
                // Simulate knowledge graph building
                let response_message = Message {
                    sender: "System".to_string(),
                    content: format!("Building knowledge graph for: {}", content),
                    action: Some("Knowledge Graph Building".to_string()),
                };
                self.current_session_mut().conversation.push(response_message);
                
                // Update the knowledge graph (simulated)
                self.current_session_mut().knowledge_graph = format!(
                    "{}\nAdded information from query: {}",
                    self.current_session().knowledge_graph,
                    content
                );
            }
        }
    }

    pub fn toggle_mode(&mut self) {
        self.app_mode = match self.app_mode {
            AppMode::Query => AppMode::Search,
            AppMode::Search => AppMode::Query,
        };
    }

    pub fn new_session(&mut self) {
        let new_id = format!("session_{}", self.sessions.len());
        let new_session = Session {
            id: new_id,
            name: format!("Session {}", self.sessions.len() + 1),
            conversation: vec![],
            knowledge_graph: "".to_string(),
        };
        
        self.sessions.push(new_session);
        self.current_session_index = self.sessions.len() - 1;
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                app.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                app.should_quit = true;
                            }
                            KeyCode::Char('t') => {
                                app.toggle_mode();
                            }
                            KeyCode::Char('n') => {
                                app.new_session();
                            }
                            KeyCode::Char('h') => {
                                app.show_help = !app.show_help;
                            }
                            KeyCode::Left => {
                                if app.current_session_index > 0 {
                                    app.current_session_index -= 1;
                                }
                            }
                            KeyCode::Right => {
                                if app.current_session_index < app.sessions.len() - 1 {
                                    app.current_session_index += 1;
                                }
                            }
                            KeyCode::Up => {
                                if app.message_scroll > 0 {
                                    app.message_scroll -= 1;
                                }
                            }
                            KeyCode::Down => {
                                app.message_scroll += 1;
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                let message = app.input.drain(..).collect();
                                app.send_message(message);
                            }
                            KeyCode::Char(c) => {
                                app.input.push(c);
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            KeyCode::Esc => {
                                app.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),  // Sessions bar
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Input bar
            ]
            .as_ref(),
        )
        .split(f.size());

    // Create the tabs for session selection
    let session_titles = app
        .sessions
        .iter()
        .map(|s| Line::from(s.name.clone()))
        .collect();
    
    let tabs = Tabs::new(session_titles)
        .block(Block::default().borders(Borders::ALL).title("Sessions"))
        .select(app.current_session_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    
    f.render_widget(tabs, chunks[0]);

    // Split the main content area into conversation and knowledge graph
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70), // Conversation
                Constraint::Percentage(30), // Knowledge graph
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Split the conversation area into actions and messages
    let conversation_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20), // Actions
                Constraint::Percentage(80), // Messages
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    // Create the actions list
    let actions: Vec<ListItem> = app
        .current_session()
        .conversation
        .iter()
        .filter_map(|m| {
            if let Some(action) = &m.action {
                Some(ListItem::new(Line::from(action.clone())))
            } else {
                None
            }
        })
        .collect();

    let actions_list = List::new(actions)
        .block(Block::default().borders(Borders::ALL).title("Actions"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_widget(actions_list, conversation_chunks[0]);

    // Create the conversation messages
    let messages: Vec<ListItem> = app
        .current_session()
        .conversation
        .iter()
        .map(|m| {
            let content = format!("{}: {}", m.sender, m.content);
            let style = match m.sender.as_str() {
                "User" => Style::default().fg(Color::Blue),
                "Assistant" => Style::default().fg(Color::Green),
                _ => Style::default().fg(Color::Gray),
            };
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let messages_list = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Conversation (Mode: {})",
                    match app.app_mode {
                        AppMode::Query => "Query",
                        AppMode::Search => "Search/Knowledge Graph Building",
                    }
                ))
        )
        .start_corner(ratatui::layout::Corner::BottomLeft);

    f.render_widget(messages_list, conversation_chunks[1]);

    // Knowledge Graph Display
    let knowledge_graph = Paragraph::new(app.current_session().knowledge_graph.as_str())
        .block(Block::default().borders(Borders::ALL).title("Knowledge Graph"));
    
    f.render_widget(knowledge_graph, main_chunks[1]);

    // Input field
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default().borders(Borders::ALL).title(match app.input_mode {
                InputMode::Normal => "Press 'e' to start editing",
                InputMode::Editing => "Press Esc to stop editing, Enter to submit",
            }),
        );
    
    f.render_widget(input, chunks[2]);

    // Show the cursor if we're in editing mode
    if let InputMode::Editing = app.input_mode {
        f.set_cursor(
            chunks[2].x + app.input.len() as u16 + 1,
            chunks[2].y + 1,
        );
    }

    // Show help overlay if requested
    if app.show_help {
        let help_text = vec![
            Line::from("Controls:"),
            Line::from("e - Edit/Input mode"),
            Line::from("q - Quit"),
            Line::from("t - Toggle between Query/Search mode"),
            Line::from("n - New session"),
            Line::from("h - Toggle help"),
            Line::from("Left/Right - Navigate sessions"),
            Line::from("Up/Down - Scroll messages"),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let help_area = centered_rect(60, 40, f.size());
        f.render_widget(help_paragraph, help_area);
    }
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()
}
