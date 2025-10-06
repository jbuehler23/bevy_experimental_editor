//! Inspector panel for viewing and editing entity components

use crate::component_registry::{ComponentCategory, ComponentRegistry, EditorComponentRegistry};
use crate::icons::Icons;
use crate::scene_editor::EditorScene;
use bevy::prelude::*;
use bevy_egui::egui;

/// Component data extracted from queries
#[derive(Clone)]
pub struct EntityComponentData {
    pub entity: Entity,
    pub name: Option<String>,
    pub transform: Option<Transform>,
    pub visibility: Option<Visibility>,
    pub sprite: Option<Sprite>,
    pub has_camera2d: bool,
    pub node: Option<Node>,
    pub has_button: bool,
    pub text: Option<Text>,
}

/// Render the inspector panel content
pub fn render_inspector_panel(
    ui: &mut egui::Ui,
    editor_scene: &EditorScene,
    component_data: Option<&EntityComponentData>,
    component_registry: &ComponentRegistry,
) {
    ui.heading("Inspector");
    ui.separator();

    // Check if an entity is selected
    let Some(selected_entity) = editor_scene.selected_entity else {
        ui.label("No entity selected");
        return;
    };

    // Check if we have component data
    let Some(data) = component_data else {
        ui.label("Entity not found");
        return;
    };

    // Entity header
    ui.horizontal(|ui| {
        ui.label(Icons::NODE);
        ui.heading(data.name.as_deref().unwrap_or("Unnamed"));
    });

    ui.label(format!("Entity ID: {:?}", selected_entity));
    ui.separator();

    // Scrollable area for components
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            // Show existing components
            render_existing_components(ui, data);

            ui.separator();

            // Add Component button
            render_add_component_menu(ui, component_registry);
        });
}

/// Render existing components on the entity
fn render_existing_components(ui: &mut egui::Ui, data: &EntityComponentData) {
    // Transform component
    if let Some(transform) = &data.transform {
        render_transform_component(ui, transform);
    }

    // Name component
    if let Some(name) = &data.name {
        render_name_component(ui, name);
    }

    // Visibility component
    if let Some(visibility) = &data.visibility {
        render_visibility_component(ui, visibility);
    }

    // Sprite component
    if let Some(sprite) = &data.sprite {
        render_sprite_component(ui, sprite);
    }

    // Camera2d component
    if data.has_camera2d {
        render_camera2d_component(ui);
    }

    // UI Node component
    if let Some(node) = &data.node {
        render_node_component(ui, node);
    }

    // Button component
    if data.has_button {
        render_button_component(ui);
    }

    // Text component
    if let Some(text) = &data.text {
        render_text_component(ui, text);
    }
}

/// Render Transform component editor
fn render_transform_component(ui: &mut egui::Ui, transform: &Transform) {
    egui::CollapsingHeader::new(format!("{} Transform", Icons::TRANSFORM))
        .default_open(true)
        .show(ui, |ui| {
            ui.label(format!("Position: {:.2?}", transform.translation));
            ui.label(format!("Rotation: {:.2?}", transform.rotation));
            ui.label(format!("Scale: {:.2?}", transform.scale));
            ui.label("(Editing coming soon)");
        });
}

/// Render Name component editor
fn render_name_component(ui: &mut egui::Ui, name: &str) {
    egui::CollapsingHeader::new(format!("{} Name", Icons::SCRIPT))
        .default_open(true)
        .show(ui, |ui| {
            ui.label(format!("Name: {}", name));
            ui.label("(Editing coming soon)");
        });
}

/// Render Visibility component editor
fn render_visibility_component(ui: &mut egui::Ui, visibility: &Visibility) {
    egui::CollapsingHeader::new(format!("{} Visibility", Icons::EYE))
        .default_open(true)
        .show(ui, |ui| {
            ui.label(format!("Visibility: {:?}", visibility));
            ui.label("(Editing coming soon)");
        });
}

/// Render Sprite component editor
fn render_sprite_component(ui: &mut egui::Ui, sprite: &Sprite) {
    egui::CollapsingHeader::new(format!("{} Sprite", Icons::SPRITE))
        .default_open(true)
        .show(ui, |ui| {
            ui.label(format!("Color: {:?}", sprite.color));
            ui.label(format!("Custom Size: {:?}", sprite.custom_size));
            ui.label("(Editing coming soon)");
        });
}

/// Render Camera2d component editor
fn render_camera2d_component(ui: &mut egui::Ui) {
    egui::CollapsingHeader::new(format!("{} Camera2D", Icons::CAMERA))
        .default_open(true)
        .show(ui, |ui| {
            ui.label("2D Camera");
            ui.label("(Editing coming soon)");
        });
}

/// Render Node component editor
fn render_node_component(ui: &mut egui::Ui, _node: &Node) {
    egui::CollapsingHeader::new("Node")
        .default_open(true)
        .show(ui, |ui| {
            ui.label("UI Node");
            ui.label("(Editing coming soon)");
        });
}

/// Render Button component editor
fn render_button_component(ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Button")
        .default_open(true)
        .show(ui, |ui| {
            ui.label("UI Button");
            ui.label("(Editing coming soon)");
        });
}

/// Render Text component editor
fn render_text_component(ui: &mut egui::Ui, text: &Text) {
    egui::CollapsingHeader::new("Text")
        .default_open(true)
        .show(ui, |ui| {
            // In Bevy 0.16, Text is a simple wrapper around String
            ui.label(format!("Text: {}", text.0));
            ui.label("(Editing coming soon)");
        });
}

/// Render "Add Component" menu
fn render_add_component_menu(ui: &mut egui::Ui, component_registry: &ComponentRegistry) {
    ui.menu_button(format!("{} Add Component", Icons::NEW), |ui| {
        for category in component_registry.categories() {
            let components = component_registry.get_by_category(category);
            if components.is_empty() {
                continue;
            }

            ui.menu_button(
                ComponentRegistry::category_name(category),
                |ui| {
                    for component_info in components {
                        if ui.button(component_info.name).clicked() {
                            info!("Add component: {}", component_info.name);
                            ui.close_menu();
                        }
                    }
                },
            );
        }
    });
}

/// Panel state for inspector
#[derive(Resource)]
pub struct InspectorPanel {
    pub visible: bool,
    pub width: f32,
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            visible: true,
            width: 300.0,
        }
    }
}

impl InspectorPanel {
    pub fn new() -> Self {
        Self::default()
    }
}

/// System to render the inspector panel
pub fn inspector_panel_system(
    mut contexts: bevy_egui::EguiContexts,
    editor_scene: Res<EditorScene>,
    mut panel: ResMut<InspectorPanel>,
    component_registry: Res<EditorComponentRegistry>,
    entity_query: Query<(
        Entity,
        Option<&Name>,
        Option<&Transform>,
        Option<&Visibility>,
        Option<&Sprite>,
        Option<&Camera2d>,
        Option<&Node>,
        Option<&Button>,
        Option<&Text>,
    )>,
) {
    if !panel.visible {
        return;
    }

    // Find component data for selected entity
    let component_data = editor_scene.selected_entity.and_then(|selected| {
        entity_query.iter().find_map(|(entity, name, transform, visibility, sprite, camera2d, node, button, text)| {
            if entity == selected {
                Some(EntityComponentData {
                    entity,
                    name: name.map(|n| n.to_string()),
                    transform: transform.copied(),
                    visibility: visibility.copied(),
                    sprite: sprite.cloned(),
                    has_camera2d: camera2d.is_some(),
                    node: node.cloned(),
                    has_button: button.is_some(),
                    text: text.cloned(),
                })
            } else {
                None
            }
        })
    });

    egui::SidePanel::right("inspector_panel")
        .default_width(panel.width)
        .min_width(250.0)
        .max_width(500.0)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            render_inspector_panel(ui, &editor_scene, component_data.as_ref(), &component_registry.registry);
        });
}
