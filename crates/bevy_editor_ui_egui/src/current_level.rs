use bevy::prelude::*;
use bevy_editor_formats::LevelData;

/// Legacy resource holding level data for UI workflows (to be phased out).
#[derive(Resource)]
pub struct CurrentLevel {
    pub level_data: LevelData,
    pub file_path: Option<String>,
    pub is_modified: bool,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            level_data: LevelData::new("Untitled Level".to_string(), 2000.0, 1000.0),
            file_path: None,
            is_modified: false,
        }
    }
}
