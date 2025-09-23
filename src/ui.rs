use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame,
};
use crate::config::Config;
use crate::model_executor::ModelExecutor;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    MainMenu,
    ModelList,
    ModelInput,
    ModelOutput,
    Settings,
    Help,
}

#[derive(Debug, Clone)]
pub struct App {
    pub config: Config,
    pub state: AppState,
    pub selected_menu_item: usize,
    pub selected_model: Option<String>,
    pub model_list_state: ListState,
    pub input_text: String,
    pub output_text: String,
    pub error_text: String,
    pub model_executor: ModelExecutor,
    pub should_quit: bool,
    pub editing_input: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut model_list_state = ListState::default();
        if !config.models.is_empty() {
            model_list_state.select(Some(0));
        }

        App {
            config,
            state: AppState::MainMenu,
            selected_menu_item: 0,
            selected_model: None,
            model_list_state,
            input_text: String::new(),
            output_text: String::new(),
            error_text: String::new(),
            model_executor: ModelExecutor::new(),
            should_quit: false,
            editing_input: false,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match self.state {
            AppState::MainMenu => self.handle_main_menu_input(key),
            AppState::ModelList => self.handle_model_list_input(key),
            AppState::ModelInput => self.handle_model_input_input(key),
            AppState::ModelOutput => self.handle_model_output_input(key),
            AppState::Settings => self.handle_settings_input(key),
            AppState::Help => self.handle_help_input(key),
        }
    }

    fn handle_main_menu_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.selected_menu_item > 0 {
                    self.selected_menu_item -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_menu_item < 4 {
                    self.selected_menu_item += 1;
                }
            }
            KeyCode::Enter => {
                match self.selected_menu_item {
                    0 => self.state = AppState::ModelList,
                    1 => self.state = AppState::Settings,
                    2 => self.state = AppState::Help,
                    3 => {
                        // Refresh models
                        if let Err(e) = self.config.discover_models("models") {
                            self.error_text = format!("Error discovering models: {}", e);
                        }
                    }
                    4 => self.should_quit = true,
                    _ => {}
                }
            }
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_model_list_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                let selected = self.model_list_state.selected().unwrap_or(0);
                if selected > 0 {
                    self.model_list_state.select(Some(selected - 1));
                }
            }
            KeyCode::Down => {
                let selected = self.model_list_state.selected().unwrap_or(0);
                if selected < self.config.models.len().saturating_sub(1) {
                    self.model_list_state.select(Some(selected + 1));
                }
            }
            KeyCode::Enter => {
                if let Some(selected) = self.model_list_state.selected() {
                    let model_names: Vec<_> = self.config.models.keys().collect();
                    if let Some(model_name) = model_names.get(selected) {
                        self.selected_model = Some((*model_name).clone());
                        self.input_text.clear();
                        self.output_text.clear();
                        self.error_text.clear();
                        self.state = AppState::ModelInput;
                    }
                }
            }
            KeyCode::Esc => self.state = AppState::MainMenu,
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_model_input_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) if self.editing_input => {
                self.input_text.push(c);
            }
            KeyCode::Backspace if self.editing_input => {
                self.input_text.pop();
            }
            KeyCode::Enter if self.editing_input => {
                // Execute the model
                self.execute_current_model();
            }
            KeyCode::Tab => {
                self.editing_input = !self.editing_input;
            }
            KeyCode::Esc => {
                if self.editing_input {
                    self.editing_input = false;
                } else {
                    self.state = AppState::ModelList;
                }
            }
            KeyCode::Enter if !self.editing_input => {
                self.editing_input = true;
            }
            KeyCode::Char('q') if !self.editing_input => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_model_output_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.state = AppState::ModelInput,
            KeyCode::Enter => self.state = AppState::ModelInput,
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_settings_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.state = AppState::MainMenu,
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_help_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.state = AppState::MainMenu,
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn execute_current_model(&mut self) {
        if let Some(ref model_name) = self.selected_model {
            if let Some(model_path) = self.config.models.get(model_name) {
                let input = if self.input_text.is_empty() {
                    "{}".to_string()
                } else {
                    self.input_text.clone()
                };

                match self.model_executor.execute_model(model_path, &input) {
                    Ok(result) => {
                        if result.success {
                            self.output_text = result.output;
                            self.error_text.clear();
                            self.state = AppState::ModelOutput;
                        } else {
                            self.error_text = result.error;
                            self.output_text.clear();
                        }
                    }
                    Err(e) => {
                        self.error_text = format!("Execution error: {}", e);
                        self.output_text.clear();
                    }
                }
            }
        }
        self.editing_input = false;
    }
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    match app.state {
        AppState::MainMenu => draw_main_menu(f, app),
        AppState::ModelList => draw_model_list(f, app),
        AppState::ModelInput => draw_model_input(f, app),
        AppState::ModelOutput => draw_model_output(f, app),
        AppState::Settings => draw_settings(f, app),
        AppState::Help => draw_help(f, app),
    }
}

fn draw_main_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("🔬 Grok Physics Models TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let menu_items = vec![
        "📁 Browse Models",
        "⚙️  Settings",
        "❓ Help",
        "🔄 Refresh Models",
        "🚪 Quit",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_menu_item {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*item).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default().title("Main Menu").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(menu, chunks[1]);

    let status = Paragraph::new(format!("Models: {} | Use ↑↓ to navigate, Enter to select, 'q' to quit", app.config.models.len()))
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn draw_model_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("📋 Available Models")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if app.config.models.is_empty() {
        let empty_message = Paragraph::new("No models found. Place .py files in the models/ directory.")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().title("Model List").borders(Borders::ALL));
        f.render_widget(empty_message, chunks[1]);
    } else {
        let items: Vec<ListItem> = app.config.models
            .keys()
            .map(|name| ListItem::new(format!("🐍 {}", name)))
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Model List").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(list, chunks[1], &mut app.model_list_state.clone());
    }

    let status = Paragraph::new("↑↓ to navigate, Enter to select model, Esc to go back, 'q' to quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn draw_model_input(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    let unknown = "Unknown".to_string();
    let model_name = app.selected_model.as_ref().unwrap_or(&unknown);
    let title = Paragraph::new(format!("🔬 Model: {}", model_name))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let input_style = if app.editing_input {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let input_title = if app.editing_input {
        "Input (Editing - Press Enter to execute, Tab to stop editing)"
    } else {
        "Input (Press Enter to edit, Tab to edit)"
    };

    let input = Paragraph::new(app.input_text.as_str())
        .style(input_style)
        .block(Block::default().title(input_title).borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(input, chunks[1]);

    if !app.error_text.is_empty() {
        let error = Paragraph::new(app.error_text.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().title("Error").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(error, chunks[2]);
    }

    let help_text = if app.editing_input {
        "Type JSON input, Enter to execute, Tab to stop editing, Esc to cancel"
    } else {
        "Enter to edit input, Esc to go back, 'q' to quit"
    };

    let status = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[3]);
}

fn draw_model_output(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let unknown = "Unknown".to_string();
    let model_name = app.selected_model.as_ref().unwrap_or(&unknown);
    let title = Paragraph::new(format!("📊 Model Output: {}", model_name))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let output = Paragraph::new(app.output_text.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Output").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(output, chunks[1]);

    let status = Paragraph::new("Press Enter or Esc to go back to input, 'q' to quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn draw_settings(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("⚙️ Settings")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let settings_text = format!(
        "API Key: {}\nDefault Model: {}\nTheme: {}\nModels Directory: models/\nConfig File: config.toml",
        if app.config.settings.api_key.is_empty() { "Not set" } else { "***" },
        if app.config.settings.default_model.is_empty() { "Not set" } else { &app.config.settings.default_model },
        app.config.ui.theme
    );

    let settings = Paragraph::new(settings_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Current Settings").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(settings, chunks[1]);

    let status = Paragraph::new("Settings can be modified in config.toml file. Press Esc to go back.")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn draw_help(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("❓ Help")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let help_text = vec![
        "🔬 Grok Physics Models TUI - Help",
        "",
        "Navigation:",
        "  ↑/↓ - Move up/down in menus",
        "  Enter - Select item",
        "  Esc - Go back",
        "  'q' - Quit application",
        "",
        "Model Management:",
        "  • Place .py files in models/ directory",
        "  • Use 'Refresh Models' to scan for new files",
        "  • Models should accept JSON input and return JSON output",
        "",
        "Model Interaction:",
        "  • Select a model from the list",
        "  • Enter JSON input (e.g., {\"energy\": 1.0})",
        "  • Press Enter to execute",
        "  • View formatted output",
        "",
        "Configuration:",
        "  • Edit config.toml for API keys and settings",
        "  • Supports API key storage for external models",
        "",
        "Sample Models Included:",
        "  • ternary_elegance.py - Ternary relationships in physics",
        "  • e8_triality.py - E8 Lie group triality",
        "  • phi_scaling.py - Golden ratio scaling in nature",
    ];

    let help = Paragraph::new(help_text.join("\n"))
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Help & Instructions").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(help, chunks[1]);

    let status = Paragraph::new("Press Esc to go back to main menu")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}