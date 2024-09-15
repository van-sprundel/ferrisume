#![allow(unused)]
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use log::{debug, info, warn};
use serde::Deserialize;

const DEFAULT_THEME_CONFIG: &str = env!("OUT_DIR");

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
        if !self
            .discover_themes_in_directory(DEFAULT_THEME_CONFIG)
            .expect("Error whole discovering default theme")
        {
            debug!("No local themes found.");
        }

        if !self
            .discover_themes_in_directory("themes")
            .expect("Error whole discovering local theme")
        {
            debug!("No local themes found.");
        }

        if self.themes.is_empty() {
            warn!("No themes were discovered!");
        } else {
            debug!("Discovered {} themes", self.themes.len());
        }
    }

    fn discover_themes_in_directory<P: AsRef<Path>>(
        &mut self,
        dir: P,
    ) -> Result<bool, std::io::Error> {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return Ok(false); // nothing to do
        }

        let mut success = false;

        for entry in fs::read_dir(dir)? {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    warn!("Error reading directory entry in {:?}: {}", dir, e);
                    continue;
                }
            };

            let path = entry.path();
            if path.is_dir() {
                let theme_toml = path.join("default.toml");
                if theme_toml.exists() {
                    match fs::read_to_string(&theme_toml) {
                        Ok(contents) => match toml::from_str::<ThemeConfig>(&contents) {
                            Ok(config) => {
                                info!("Discovered theme: {} in {:?}", config.name, path);
                                self.add_theme(Theme {
                                    name: config.name.clone(),
                                    path: path.to_path_buf(),
                                    config,
                                });

                                success = true;
                            }
                            Err(e) => {
                                warn!("Failed to parse theme config in {:?}: {}", theme_toml, e);
                            }
                        },
                        Err(e) => {
                            warn!("Failed to read theme config file {:?}: {}", theme_toml, e);
                        }
                    }
                }
            }
        }

        Ok(success)
    }

    fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.to_ascii_lowercase(), theme);
    }
}
