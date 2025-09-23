use anyhow::{Context, Result};
use pancurses::{initscr, endwin, noecho, cbreak, Input, Window};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    api_key: Option<String>,
    model_files: Vec<String>,
    grok_models: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            model_files: Vec::new(),
            grok_models: vec![
                "grok-beta".to_string(),
                "grok-vision-beta".to_string(),
            ],
        }
    }
}

struct App {
    config: Config,
    config_path: PathBuf,
    window: Window,
    current_screen: Screen,
    selected_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Screen {
    MainMenu,
    ModelFiles,
    ApiKey,
    ModelInteraction,
}

impl App {
    fn new() -> Result<Self> {
        let window = initscr();
        noecho();
        cbreak();
        window.keypad(true);
        
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("grok-tui");
        
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
        
        let config_path = config_dir.join("config.json");
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            serde_json::from_str(&content)
                .unwrap_or_default()
        } else {
            Config::default()
        };

        Ok(Self {
            config,
            config_path,
            window,
            current_screen: Screen::MainMenu,
            selected_index: 0,
        })
    }

    fn save_config(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.config)
            .context("Failed to serialize config")?;
        fs::write(&self.config_path, content)
            .context("Failed to write config file")?;
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        loop {
            self.draw()?;
            
            match self.window.getch() {
                Some(Input::Character('q')) => break,
                Some(Input::KeyUp) => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
                Some(Input::KeyDown) => {
                    let max_index = match self.current_screen {
                        Screen::MainMenu => 3,
                        Screen::ModelFiles => self.config.model_files.len().max(1),
                        Screen::ApiKey => 1,
                        Screen::ModelInteraction => self.config.grok_models.len().max(1),
                    };
                    if self.selected_index < max_index - 1 {
                        self.selected_index += 1;
                    }
                }
                Some(Input::Character('\n')) => {
                    self.handle_selection()?;
                }
                Some(Input::Character('\x1b')) => { // ESC key
                    if self.current_screen != Screen::MainMenu {
                        self.current_screen = Screen::MainMenu;
                        self.selected_index = 0;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&self) -> Result<()> {
        self.window.clear();
        self.window.mv(0, 0);
        
        match self.current_screen {
            Screen::MainMenu => self.draw_main_menu(),
            Screen::ModelFiles => self.draw_model_files(),
            Screen::ApiKey => self.draw_api_key(),
            Screen::ModelInteraction => self.draw_model_interaction(),
        }
        
        self.window.refresh();
        Ok(())
    }

    fn draw_main_menu(&self) {
        self.window.addstr("=== Grok TUI - Main Menu ===\n\n");
        
        let menu_items = [
            "1. Manage Model Files (.py)",
            "2. Set API Key",
            "3. Interact with Grok Models",
            "4. Quit",
        ];
        
        for (i, item) in menu_items.iter().enumerate() {
            if i == self.selected_index {
                self.window.addstr("> ");
            } else {
                self.window.addstr("  ");
            }
            self.window.addstr(&format!("{}\n", item));
        }
        
        self.window.addstr("\n");
        self.window.addstr("Controls: ↑/↓ to navigate, Enter to select, q to quit\n");
        
        // Show current status
        self.window.addstr(&format!("\nStatus:\n"));
        self.window.addstr(&format!("API Key: {}\n", 
            if self.config.api_key.is_some() { "Set" } else { "Not set" }));
        self.window.addstr(&format!("Model Files: {} loaded\n", self.config.model_files.len()));
    }

    fn draw_model_files(&self) {
        self.window.addstr("=== Model Files Management ===\n\n");
        
        if self.config.model_files.is_empty() {
            self.window.addstr("No model files loaded.\n\n");
            if self.selected_index == 0 {
                self.window.addstr("> ");
            } else {
                self.window.addstr("  ");
            }
            self.window.addstr("Add new model file\n");
        } else {
            self.window.addstr("Loaded model files:\n");
            for (i, file) in self.config.model_files.iter().enumerate() {
                if i == self.selected_index {
                    self.window.addstr("> ");
                } else {
                    self.window.addstr("  ");
                }
                self.window.addstr(&format!("{}\n", file));
            }
            
            if self.selected_index == self.config.model_files.len() {
                self.window.addstr("> ");
            } else {
                self.window.addstr("  ");
            }
            self.window.addstr("Add new model file\n");
        }
        
        self.window.addstr("\nControls: ↑/↓ to navigate, Enter to select/add file, ESC to go back\n");
        self.window.addstr("Note: This demo shows file management interface.\n");
        self.window.addstr("In a full implementation, this would open a file picker.\n");
    }

    fn draw_api_key(&self) {
        self.window.addstr("=== API Key Configuration ===\n\n");
        
        match &self.config.api_key {
            Some(key) => {
                self.window.addstr(&format!("Current API Key: {}****\n\n", &key[..key.len().min(8)]));
            }
            None => {
                self.window.addstr("No API Key configured.\n\n");
            }
        }
        
        let options = ["Set/Update API Key", "Remove API Key"];
        
        for (i, option) in options.iter().enumerate() {
            if i == self.selected_index {
                self.window.addstr("> ");
            } else {
                self.window.addstr("  ");
            }
            self.window.addstr(&format!("{}\n", option));
        }
        
        self.window.addstr("\nControls: ↑/↓ to navigate, Enter to select, ESC to go back\n");
        self.window.addstr("Note: This demo shows API key management interface.\n");
        self.window.addstr("In a full implementation, this would prompt for secure key input.\n");
    }

    fn draw_model_interaction(&self) {
        self.window.addstr("=== Grok Model Interaction ===\n\n");
        
        if self.config.api_key.is_none() {
            self.window.addstr("⚠️  Please set your API key first!\n\n");
        }
        
        self.window.addstr("Available Grok Models:\n");
        for (i, model) in self.config.grok_models.iter().enumerate() {
            if i == self.selected_index {
                self.window.addstr("> ");
            } else {
                self.window.addstr("  ");
            }
            self.window.addstr(&format!("{}\n", model));
        }
        
        if self.selected_index == self.config.grok_models.len() {
            self.window.addstr("> ");
        } else {
            self.window.addstr("  ");
        }
        self.window.addstr("Send test query\n");
        
        self.window.addstr("\nControls: ↑/↓ to navigate, Enter to interact with model, ESC to go back\n");
        self.window.addstr("Note: This demo shows model interaction interface.\n");
        self.window.addstr("In a full implementation, this would make actual API calls.\n");
    }

    fn handle_selection(&mut self) -> Result<()> {
        match self.current_screen {
            Screen::MainMenu => {
                match self.selected_index {
                    0 => {
                        self.current_screen = Screen::ModelFiles;
                        self.selected_index = 0;
                    }
                    1 => {
                        self.current_screen = Screen::ApiKey;
                        self.selected_index = 0;
                    }
                    2 => {
                        self.current_screen = Screen::ModelInteraction;
                        self.selected_index = 0;
                    }
                    3 => return Ok(()), // This will exit the run loop
                    _ => {}
                }
            }
            Screen::ModelFiles => {
                if self.selected_index == self.config.model_files.len() {
                    // Simulate adding a new file
                    let demo_file = format!("demo_model_{}.py", self.config.model_files.len() + 1);
                    self.config.model_files.push(demo_file);
                    self.save_config()?;
                    
                    // Show feedback
                    self.window.addstr("\nDemo file added! (Press any key to continue)\n");
                    self.window.refresh();
                    self.window.getch();
                }
            }
            Screen::ApiKey => {
                match self.selected_index {
                    0 => {
                        // Simulate setting API key
                        self.config.api_key = Some("demo_api_key_12345".to_string());
                        self.save_config()?;
                        
                        // Show feedback
                        self.window.addstr("\nDemo API key set! (Press any key to continue)\n");
                        self.window.refresh();
                        self.window.getch();
                    }
                    1 => {
                        // Remove API key
                        self.config.api_key = None;
                        self.save_config()?;
                        
                        // Show feedback
                        self.window.addstr("\nAPI key removed! (Press any key to continue)\n");
                        self.window.refresh();
                        self.window.getch();
                    }
                    _ => {}
                }
            }
            Screen::ModelInteraction => {
                if self.selected_index < self.config.grok_models.len() {
                    let model = &self.config.grok_models[self.selected_index];
                    self.window.addstr(&format!("\nSelected model: {}\n", model));
                    self.window.addstr("This would initiate interaction with the model.\n");
                    self.window.addstr("(Press any key to continue)\n");
                    self.window.refresh();
                    self.window.getch();
                } else {
                    // Send test query
                    self.window.addstr("\nSending test query to selected model...\n");
                    self.window.addstr("Response: Hello! This is a demo response from Grok.\n");
                    self.window.addstr("(Press any key to continue)\n");
                    self.window.refresh();
                    self.window.getch();
                }
            }
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        endwin();
    }
}

fn main() -> Result<()> {
    let mut app = App::new()
        .context("Failed to initialize application")?;
    
    app.run()
        .context("Application runtime error")?;
    
    println!("Thank you for using Grok TUI!");
    Ok(())
}
