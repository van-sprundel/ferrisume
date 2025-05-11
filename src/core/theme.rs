#![allow(unused)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use directories::ProjectDirs;
use log::{debug, info, warn};
use serde::Deserialize;
use tempfile;

use crate::core::resources;

lazy_static! {
    static ref TEMP_DIRS: Arc<Mutex<Vec<tempfile::TempDir>>> = Arc::new(Mutex::new(Vec::new()));
}

fn temp_dir_storage() -> Arc<Mutex<Vec<tempfile::TempDir>>> {
    TEMP_DIRS.clone()
}

#[derive(Deserialize, Clone, Debug)]
pub struct ThemeConfig {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
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

    fn get_theme_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // curr directory
        paths.push(PathBuf::from("themes"));

        // XDG standard directories
        if let Some(proj_dirs) = ProjectDirs::from("com", "ferrisume", "ferrisume") {
            paths.push(proj_dirs.data_dir().join("themes"));

            #[cfg(not(target_os = "windows"))]
            paths.push(PathBuf::from("/usr/share/ferrisume/themes"));
        }

        // check next to the executable
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                paths.push(exe_dir.join("themes"));
            }
        }

        paths
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

    pub fn get_all_themes(&self) -> Vec<(&String, &ThemeConfig)> {
        self.themes
            .iter()
            .map(|(name, theme)| (name, &theme.config))
            .collect()
    }

    pub fn discover_themes(&mut self) {
        let mut theme_found = false;

        for theme_dir in Self::get_theme_paths() {
            let default_theme_path = theme_dir.join("default");
            if default_theme_path.exists() && self.discover_themes_in_directory(&default_theme_path)
            {
                info!("Found default theme in {:?}", default_theme_path);
                theme_found = true;
            }

            if self.discover_themes_in_directory(&theme_dir) {
                theme_found = true;
            }
        }

        if !theme_found {
            // use embedded themes as fallback
            self.register_embedded_themes();
        }

        if self.themes.is_empty() {
            warn!("No themes were discovered!");
        } else {
            debug!("Discovered {} themes", self.themes.len());
        }
    }

    fn register_embedded_themes(&mut self) {
        let temp_dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(e) => {
                warn!(
                    "Could not create temporary directory for embedded themes: {}",
                    e
                );
                return;
            }
        };

        let default_theme_dir = temp_dir.path().join("default");

        let embedded_theme = resources::get_embedded_default_theme();
        if let Err(e) = resources::extract_embedded_theme(&embedded_theme, &default_theme_dir) {
            warn!("Failed to extract embedded theme: {}", e);
            return;
        }

        if self.discover_themes_in_directory(&default_theme_dir) {
            info!("Registered embedded default theme");

            match temp_dir_storage().lock() {
                Ok(mut storage) => storage.push(temp_dir),
                Err(_) => {
                    // fallback if mutex is poisoned
                    std::mem::forget(temp_dir);
                }
            }
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
