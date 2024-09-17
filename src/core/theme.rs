#![allow(unused)]
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use log::{debug, info, warn};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ThemeConfig {
    name: String,
    description: String,
    author: String,
    version: String,
}

#[derive(Clone)]
pub struct Theme {
    name: String,
    pub path: PathBuf,
    config: ThemeConfig,
}

#[derive(Clone)]
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = ThemeManager {
            themes: HashMap::with_capacity(256),
            current_theme: "default".to_string(),
        };

        manager.discover_themes();

        manager
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self
            .themes
            .iter()
            .any(|(k, _)| *k == theme_name.to_ascii_lowercase())
        {
            self.current_theme = theme_name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name).into())
        }
    }

    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme.to_ascii_lowercase())
    }

    pub fn discover_themes(&mut self) {
        let default_theme_path = Path::new("themes").join("default");
        if !self.discover_themes_in_directory(&default_theme_path) {
            warn!("Default theme not found in {:?}", default_theme_path);
        }

        if !self.discover_themes_in_directory("themes") {
            debug!("No additional themes found in 'themes' directory.");
        }

        if self.themes.is_empty() {
            warn!("No themes were discovered!");
        } else {
            debug!("Discovered {} themes", self.themes.len());
        }
    }

    fn discover_themes_in_directory<P: AsRef<Path>>(&mut self, dir: P) -> bool {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return false;
        }

        let mut success = false;

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let theme_toml = path.join("config.toml");
                    if theme_toml.exists() {
                        if let Ok(contents) = fs::read_to_string(&theme_toml) {
                            if let Ok(config) = toml::from_str::<ThemeConfig>(&contents) {
                                info!("Discovered theme: {} in {:?}", config.name, path);
                                self.add_theme(Theme {
                                    name: config.name.clone(),
                                    path: path.to_path_buf(),
                                    config,
                                });
                                success = true;
                            } else {
                                warn!("Failed to parse theme config in {:?}", theme_toml);
                            }
                        } else {
                            warn!("Failed to read theme config file {:?}", theme_toml);
                        }
                    }
                }
            }
        }

        success
    }

    fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.to_ascii_lowercase(), theme);
    }
}
