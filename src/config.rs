use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_key: String,
    pub default_model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiSettings {
    pub theme: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub settings: Settings,
    pub models: HashMap<String, String>,
    pub ui: UiSettings,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                api_key: String::new(),
                default_model: String::new(),
            },
            models: HashMap::new(),
            ui: UiSettings {
                theme: "dark".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        if path.as_ref().exists() {
            let content = fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn discover_models(&mut self, models_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.models.clear();
        
        if let Ok(entries) = fs::read_dir(models_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(path) = entry.path().to_str() {
                        if path.ends_with(".py") {
                            if let Some(filename) = entry.file_name().to_str() {
                                let model_name = filename.trim_end_matches(".py").to_string();
                                self.models.insert(model_name, path.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}