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
    pub name: String,
    pub path: PathBuf,
    pub config: ThemeConfig,
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
        // if the theme_name contains '/' or '\', or starts with '.', it's treated as a path
        if theme_name.contains('/') || theme_name.contains('\\') || theme_name.starts_with('.') {
            let path = std::path::Path::new(theme_name);
            if !path.exists() {
                return Err(format!("Theme path '{}' not found", theme_name).into());
            }

            if !path.is_dir() {
                return Err(format!("Theme path '{}' is not a directory", theme_name).into());
            }

            if let Some((name, _)) = self.themes.iter().find(|(_, theme)| theme.path == path) {
                self.current_theme = name.to_string();
                info!(
                    "Using registered theme '{}' from path '{}'",
                    self.current_theme, theme_name
                );
                return Ok(());
            }

            if self.discover_themes_in_directory(path) {
                if let Ok(theme_config) = self.get_theme_config_from_path(path) {
                    self.current_theme = theme_config.name.to_ascii_lowercase();
                    info!(
                        "Loaded theme '{}' from path '{}'",
                        self.current_theme, theme_name
                    );
                    Ok(())
                } else {
                    Err(
                        format!("Failed to load theme config from path '{}'", theme_name).into(),
                    )
                }
            } else {
                Err(format!("Failed to load theme from path '{}'", theme_name).into())
            }
        } else if self
            .themes
            .iter()
            .any(|(k, _)| *k == theme_name.to_ascii_lowercase())
        {
            self.current_theme = theme_name.to_ascii_lowercase();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name).into())
        }
    }

    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme.to_ascii_lowercase())
    }

    pub fn get_all_themes(&self) -> &HashMap<String, Theme> {
        &self.themes
    }

    pub fn discover_themes(&mut self) {
        let mut any_theme_found = false;
        let mut default_theme_found = false;

        for theme_dir in Self::get_theme_paths() {
            let default_theme_path = theme_dir.join("default");
            if default_theme_path.exists() && self.discover_themes_in_directory(&default_theme_path)
            {
                info!("Found default theme in {:?}", default_theme_path);
                default_theme_found = true;
                any_theme_found = true;
            }

            if self.discover_themes_in_directory(&theme_dir) {
                any_theme_found = true;
            }
        }

        if !default_theme_found {
            info!("No default theme found in filesystem, registering embedded default theme");
            self.register_embedded_themes();
        }

        if self.themes.is_empty() {
            warn!("No themes were discovered!");
        } else {
            let theme_names = self.themes.keys().cloned().collect::<Vec<_>>().join(", ");
            info!("Discovered {} themes: {}", self.themes.len(), theme_names);
        }
    }

    fn register_embedded_themes(&mut self) {
        let temp_dir = match tempfile::Builder::new()
            .prefix("ferrisume-themes")
            .tempdir()
        {
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
            info!("Successfully registered embedded default theme");

            match temp_dir_storage().lock() {
                Ok(mut storage) => {
                    storage.push(temp_dir);
                    debug!("Stored temporary directory for embedded themes");
                }
                Err(_) => {
                    // fallback if mutex is poisoned
                    std::mem::forget(temp_dir);
                    debug!("Mutex poisoned, using mem::forget on temp_dir");
                }
            }
        } else {
            warn!("Failed to register embedded default theme");
        }
    }

    fn get_theme_config_from_path<P: AsRef<Path>>(
        &self,
        dir: P,
    ) -> Result<ThemeConfig, Box<dyn std::error::Error>> {
        let dir = dir.as_ref();
        let theme_toml = dir.join("config.toml");

        if !theme_toml.exists() {
            return Err(format!("Theme config not found at {:?}", theme_toml).into());
        }

        let contents = fs::read_to_string(&theme_toml)?;
        let config = toml::from_str::<ThemeConfig>(&contents)?;
        Ok(config)
    }

    fn discover_themes_in_directory<P: AsRef<Path>>(&mut self, dir: P) -> bool {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return false;
        }

        let mut success = false;

        let theme_toml = dir.join("config.toml");
        if theme_toml.exists() {
            if let Ok(contents) = fs::read_to_string(&theme_toml) {
                if let Ok(config) = toml::from_str::<ThemeConfig>(&contents) {
                    info!("Discovered theme: {} in {:?}", config.name, dir);
                    self.add_theme(Theme {
                        name: config.name.clone(),
                        path: dir.to_path_buf(),
                        config,
                    });
                    success = true;
                } else {
                    warn!("Failed to parse theme config in {:?}", theme_toml);
                }
            } else {
                warn!("Failed to read theme config file {:?}", theme_toml);
            }
        } else if let Ok(entries) = fs::read_dir(dir) {
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
