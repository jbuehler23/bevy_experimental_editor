use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Client/game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub asset_watch: bool,
    pub default_level: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            window_title: "Eryndor Game".to_string(),
            window_width: 1280,
            window_height: 720,
            asset_watch: true,
            default_level: "world/level1.bscene".to_string(),
        }
    }
}

/// Project configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub client_config: ClientConfig,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "New Project".to_string(),
            version: "0.1.0".to_string(),
            author: None,
            description: None,
            client_config: ClientConfig::default(),
        }
    }
}

impl ProjectConfig {
    /// Create a new project configuration
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Load project config from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: ProjectConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Save project config to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// Project metadata tracked at runtime
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub config: ProjectConfig,
    pub root_path: PathBuf,
    pub assets_path: PathBuf,
    pub levels_path: PathBuf,
}

impl ProjectMetadata {
    /// Create project metadata from a project root directory
    pub fn from_project_path<P: AsRef<Path>>(project_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let root_path = project_path.as_ref().to_path_buf();
        let config_path = root_path.join("project.bvy");

        let config = if config_path.exists() {
            ProjectConfig::load_from_file(&config_path)?
        } else {
            // Create default config
            let config = ProjectConfig::new(
                root_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("New Project")
                    .to_string()
            );
            config.save_to_file(&config_path)?;
            config
        };

        let assets_path = root_path.join("assets");
        let levels_path = assets_path.join("world");

        // Ensure directories exist
        std::fs::create_dir_all(&assets_path)?;
        std::fs::create_dir_all(&levels_path)?;
        std::fs::create_dir_all(assets_path.join("tilesets"))?;
        std::fs::create_dir_all(assets_path.join("sprites"))?;

        Ok(Self {
            config,
            root_path,
            assets_path,
            levels_path,
        })
    }

    /// Get the full path to an asset
    pub fn asset_path<P: AsRef<Path>>(&self, relative_path: P) -> PathBuf {
        self.assets_path.join(relative_path)
    }

    /// Get the full path to a level file
    pub fn level_path<P: AsRef<Path>>(&self, level_name: P) -> PathBuf {
        self.levels_path.join(level_name)
    }

    /// Save the current config back to disk
    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.root_path.join("project.bvy");
        self.config.save_to_file(config_path)
    }
}
