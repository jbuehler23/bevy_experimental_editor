use serde::{Deserialize, Serialize};
use std::path::Path;
use super::level_format::LevelData;

/// Bevy Scene Format (.bscene) - JSON-based scene storage
/// This format is used for authoring and version control.
/// At runtime, scenes can be exported to .scn.ron for faster loading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BevyScene {
    /// Scene format version for future compatibility
    pub format_version: String,
    /// Scene metadata
    pub metadata: SceneMetadata,
    /// The actual level/scene data
    pub data: LevelData,
}

/// Metadata specific to the scene file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    /// When this scene was last modified
    pub last_modified: Option<String>,
    /// Editor version that last saved this
    pub editor_version: Option<String>,
    /// Optional tags for organization
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Default for SceneMetadata {
    fn default() -> Self {
        Self {
            last_modified: None,
            editor_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            tags: Vec::new(),
        }
    }
}

impl BevyScene {
    /// Create a new scene with given level data
    pub fn new(data: LevelData) -> Self {
        Self {
            format_version: "1.0".to_string(),
            metadata: SceneMetadata::default(),
            data,
        }
    }

    /// Create a new empty scene
    pub fn new_empty(name: String, world_width: f32, world_height: f32) -> Self {
        Self::new(LevelData::new(name, world_width, world_height))
    }

    /// Save scene to .bscene file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        // Update last modified time
        let mut scene = self.clone();
        scene.metadata.last_modified = Some(
            chrono::Utc::now().to_rfc3339()
        );

        let json = serde_json::to_string_pretty(&scene)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load scene from .bscene file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let scene: BevyScene = serde_json::from_str(&json)?;
        Ok(scene)
    }

    /// Export to Bevy's native .scn.ron format (for future use)
    /// This will be implemented when we add the build/export pipeline
    #[allow(dead_code)]
    pub fn export_to_ron<P: AsRef<Path>>(&self, _path: P) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Convert to Bevy DynamicScene and serialize to RON
        // This will be part of the build pipeline when BSN supports assets
        todo!("RON export will be implemented when BSN is ready")
    }
}
