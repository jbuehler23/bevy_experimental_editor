use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use eryndor_common::{LevelData, BevyScene};
use std::path::PathBuf;

/// Represents a single open scene/level
#[derive(Clone)]
pub struct OpenScene {
    pub name: String,
    pub file_path: Option<String>,
    pub level_data: LevelData,
    pub is_modified: bool,
}

impl OpenScene {
    pub fn new(name: String, level_data: LevelData) -> Self {
        Self {
            name,
            file_path: None,
            level_data,
            is_modified: false,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let scene = BevyScene::load_from_file(path)?;
        let name = PathBuf::from(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Ok(Self {
            name,
            file_path: Some(path.to_string()),
            level_data: scene.data,
            is_modified: false,
        })
    }
}

/// Resource managing multiple open scenes with tabs
#[derive(Resource)]
pub struct OpenScenes {
    pub scenes: Vec<OpenScene>,
    pub active_index: usize,
}

impl Default for OpenScenes {
    fn default() -> Self {
        // Start with one untitled scene
        Self {
            scenes: vec![OpenScene::new(
                "Untitled".to_string(),
                LevelData::new("Untitled Level".to_string(), 2000.0, 1000.0),
            )],
            active_index: 0,
        }
    }
}

impl OpenScenes {
    /// Get the currently active scene
    pub fn active_scene(&self) -> Option<&OpenScene> {
        self.scenes.get(self.active_index)
    }

    /// Get the currently active scene mutably
    pub fn active_scene_mut(&mut self) -> Option<&mut OpenScene> {
        self.scenes.get_mut(self.active_index)
    }

    /// Add a new scene and make it active
    pub fn add_scene(&mut self, scene: OpenScene) {
        self.scenes.push(scene);
        self.active_index = self.scenes.len() - 1;
    }

    /// Close a scene by index
    pub fn close_scene(&mut self, index: usize) {
        if self.scenes.len() <= 1 {
            // Don't close the last scene, just reset it
            if let Some(scene) = self.scenes.get_mut(0) {
                *scene = OpenScene::new(
                    "Untitled".to_string(),
                    LevelData::new("Untitled Level".to_string(), 2000.0, 1000.0),
                );
            }
            self.active_index = 0;
            return;
        }

        self.scenes.remove(index);

        // Adjust active index
        if self.active_index >= self.scenes.len() {
            self.active_index = self.scenes.len() - 1;
        } else if self.active_index > index {
            self.active_index -= 1;
        }
    }

    /// Switch to a specific scene
    pub fn set_active(&mut self, index: usize) {
        if index < self.scenes.len() {
            self.active_index = index;
        }
    }

    /// Check if any scenes are modified
    pub fn has_unsaved_changes(&self) -> bool {
        self.scenes.iter().any(|s| s.is_modified)
    }
}

/// Render scene tabs content (called from ui_system within a panel)
pub fn render_scene_tabs_content(
    ui: &mut egui::Ui,
    open_scenes: &mut OpenScenes,
) {
    ui.horizontal(|ui| {
                let mut scene_to_close: Option<usize> = None;
                let mut new_active_index: Option<usize> = None;

                for (idx, scene) in open_scenes.scenes.iter().enumerate() {
                    let is_active = idx == open_scenes.active_index;

                    // Tab button styling
                    let button_text = if scene.is_modified {
                        format!("● {}", scene.name)
                    } else {
                        scene.name.clone()
                    };

                    let response = ui.selectable_label(is_active, button_text);

                    if response.clicked() {
                        new_active_index = Some(idx);
                    }

                    // Close button (X) for each tab
                    if ui.small_button("✖")
                        .on_hover_text("Close scene")
                        .clicked()
                    {
                        scene_to_close = Some(idx);
                    }

                    ui.separator();
                }

                // New tab button
                if ui.button("➕").on_hover_text("New scene").clicked() {
                    let new_scene = OpenScene::new(
                        format!("Untitled {}", open_scenes.scenes.len() + 1),
                        LevelData::new("New Level".to_string(), 2000.0, 1000.0),
                    );
                    open_scenes.add_scene(new_scene);
                }

                // Apply changes after iteration
                if let Some(index) = new_active_index {
                    open_scenes.set_active(index);
                }
                if let Some(index) = scene_to_close {
                    if open_scenes.scenes[index].is_modified {
                        // TODO: Show "Save changes?" dialog
                        info!("Scene has unsaved changes, close anyway? (dialog not yet implemented)");
                    }
                    open_scenes.close_scene(index);
                }
            });
}
