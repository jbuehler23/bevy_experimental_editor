//! Scene tree panel for viewing and editing the entity hierarchy

use crate::icons::Icons;
use crate::scene_editor::{EditorScene, EditorSceneEntity};
use bevy::prelude::*;
use bevy_egui::egui;

/// Data needed to render an entity node (extracted from queries)
#[derive(Clone)]
pub struct EntityNodeData {
    pub entity: Entity,
    pub name: String,
    pub has_children: bool,
    pub children: Vec<Entity>,
}

/// Render the scene tree panel content
pub fn render_scene_tree_panel(
    ui: &mut egui::Ui,
    editor_scene: &mut EditorScene,
    entity_data: &[EntityNodeData],
) {
    ui.heading("Scene Tree");
    ui.separator();

    // Add entity button
    ui.horizontal(|ui| {
        if ui.button(format!("{} Add Entity", Icons::NEW)).clicked() {
            // Signal to add a new entity (will be handled in a system)
            info!("Add entity clicked");
        }

        if let Some(selected) = editor_scene.selected_entity {
            if ui.button(format!("{} Delete", Icons::CLOSE)).clicked() {
                info!("Delete entity clicked: {:?}", selected);
                // Signal to delete entity
            }
        }
    });

    ui.separator();

    // Render the entity tree
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            if let Some(root_entity) = editor_scene.root_entity {
                render_entity_node(ui, root_entity, editor_scene, entity_data, 0);
            } else {
                ui.label("No scene loaded");
            }
        });
}

/// Recursively render an entity and its children
fn render_entity_node(
    ui: &mut egui::Ui,
    entity: Entity,
    editor_scene: &mut EditorScene,
    entity_data: &[EntityNodeData],
    depth: usize,
) {
    // Find this entity's data
    let Some(data) = entity_data.iter().find(|d| d.entity == entity) else {
        return;
    };

    // Indentation for hierarchy
    let indent = depth as f32 * 16.0;
    ui.add_space(indent);

    // Entity row
    ui.horizontal(|ui| {
        // Expand/collapse icon (if has children)
        if data.has_children {
            ui.label(Icons::CHEVRON_DOWN);
        } else {
            ui.add_space(12.0); // Space for alignment
        }

        // Entity icon
        ui.label(Icons::NODE);

        // Entity name (selectable)
        let is_selected = editor_scene.is_selected(entity);
        let response = ui.selectable_label(is_selected, &data.name);

        if response.clicked() {
            editor_scene.select_entity(entity);
            info!("Selected entity: {} ({:?})", data.name, entity);
        }

        // Show entity ID on hover
        response.on_hover_text(format!("Entity ID: {:?}", entity));
    });

    // Render children recursively
    if data.has_children {
        for child in &data.children {
            render_entity_node(ui, *child, editor_scene, entity_data, depth + 1);
        }
    }
}

/// Panel state for scene tree
#[derive(Resource)]
pub struct SceneTreePanel {
    pub visible: bool,
    pub width: f32,
}

impl Default for SceneTreePanel {
    fn default() -> Self {
        Self {
            visible: true,
            width: 250.0,
        }
    }
}

impl SceneTreePanel {
    pub fn new() -> Self {
        Self::default()
    }
}

/// System to render the scene tree panel
pub fn scene_tree_panel_system(
    mut contexts: bevy_egui::EguiContexts,
    mut editor_scene: ResMut<EditorScene>,
    mut panel: ResMut<SceneTreePanel>,
    entity_query: Query<(Entity, Option<&Name>, Option<&Children>), With<EditorSceneEntity>>,
) {
    if !panel.visible {
        return;
    }

    // Extract entity data from queries
    let entity_data: Vec<EntityNodeData> = entity_query
        .iter()
        .map(|(entity, name, children)| EntityNodeData {
            entity,
            name: name.map(|n| n.to_string()).unwrap_or_else(|| "Unnamed".to_string()),
            has_children: children.map_or(false, |c| !c.is_empty()),
            children: children.map_or_else(Vec::new, |c| c.iter().collect()),
        })
        .collect();

    egui::SidePanel::left("scene_tree_panel")
        .default_width(panel.width)
        .min_width(200.0)
        .max_width(400.0)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            render_scene_tree_panel(ui, &mut editor_scene, &entity_data);
        });
}

/// Commands for entity operations
#[derive(Event)]
pub enum SceneTreeCommand {
    AddEntity { parent: Option<Entity> },
    DeleteEntity { entity: Entity },
    RenameEntity { entity: Entity, new_name: String },
    ReparentEntity { entity: Entity, new_parent: Option<Entity> },
}

/// System to handle scene tree commands
pub fn handle_scene_tree_commands(
    mut commands: Commands,
    mut events: EventReader<SceneTreeCommand>,
    mut editor_scene: ResMut<EditorScene>,
) {
    for event in events.read() {
        match event {
            SceneTreeCommand::AddEntity { parent } => {
                let entity = commands
                    .spawn((
                        Name::new("New Entity"),
                        Transform::default(),
                        Visibility::default(),
                        EditorSceneEntity,
                    ))
                    .id();

                // Set parent if specified
                if let Some(parent_entity) = parent {
                    commands.entity(entity).set_parent(*parent_entity);
                } else if let Some(root) = editor_scene.root_entity {
                    commands.entity(entity).set_parent(root);
                }

                editor_scene.select_entity(entity);
                editor_scene.mark_modified();
                info!("Added new entity: {:?}", entity);
            }

            SceneTreeCommand::DeleteEntity { entity } => {
                commands.entity(*entity).despawn_recursive();
                editor_scene.clear_selection();
                editor_scene.mark_modified();
                info!("Deleted entity: {:?}", entity);
            }

            SceneTreeCommand::RenameEntity { entity, new_name } => {
                commands.entity(*entity).insert(Name::new(new_name.clone()));
                editor_scene.mark_modified();
                info!("Renamed entity {:?} to: {}", entity, new_name);
            }

            SceneTreeCommand::ReparentEntity { entity, new_parent } => {
                if let Some(parent) = new_parent {
                    commands.entity(*entity).set_parent(*parent);
                } else {
                    commands.entity(*entity).remove_parent();
                }
                editor_scene.mark_modified();
                info!("Reparented entity {:?} to {:?}", entity, new_parent);
            }
        }
    }
}
