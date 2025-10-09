//! Inspector panel for viewing and editing entity components

use crate::component_registry::ComponentRegistry;
use crate::icons::Icons;
use crate::scene_editor::{EditorScene, TransformEditEvent, NameEditEvent};
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
    transform_events: &mut EventWriter<TransformEditEvent>,
    name_events: &mut EventWriter<NameEditEvent>,
    name_edit_buffer: &mut String,
    project_root: Option<&std::path::PathBuf>,
    asset_server: &AssetServer,
    texture_events: &mut bevy::ecs::event::EventWriter<crate::scene_editor::SpriteTextureEvent>,
    sprite_texture_id: Option<egui::TextureId>,
    images: &Assets<Image>,
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

    // Entity header with inline name editing
    ui.horizontal(|ui| {
        ui.label(Icons::NODE);

        // Initialize buffer with current name
        if name_edit_buffer.is_empty() {
            *name_edit_buffer = data.name.clone().unwrap_or_else(|| "Unnamed".to_string());
        }

        let response = ui.text_edit_singleline(name_edit_buffer);

        // Send event when user finished editing (lost focus or pressed enter)
        if response.lost_focus() && !name_edit_buffer.is_empty() {
            name_events.write(NameEditEvent {
                entity: selected_entity,
                new_name: name_edit_buffer.clone(),
            });
        }
    });

    ui.label(format!("Entity ID: {:?}", selected_entity));
    ui.separator();

    // Scrollable area for components
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            // Show existing components
            render_existing_components(ui, data, selected_entity, transform_events, project_root, asset_server, texture_events, sprite_texture_id, images);

            ui.separator();

            // Add Component button
            render_add_component_menu(ui, component_registry);
        });
}

/// Render existing components on the entity
fn render_existing_components(
    ui: &mut egui::Ui,
    data: &EntityComponentData,
    entity: Entity,
    transform_events: &mut EventWriter<TransformEditEvent>,
    project_root: Option<&std::path::PathBuf>,
    asset_server: &AssetServer,
    texture_events: &mut bevy::ecs::event::EventWriter<crate::scene_editor::SpriteTextureEvent>,
    sprite_texture_id: Option<egui::TextureId>,
    images: &Assets<Image>,
) {
    // Transform component
    if let Some(transform) = &data.transform {
        render_transform_component(ui, transform, entity, transform_events);
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
        render_sprite_component(ui, sprite, entity, project_root, asset_server, texture_events, sprite_texture_id, images);
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
fn render_transform_component(
    ui: &mut egui::Ui,
    transform: &Transform,
    entity: Entity,
    transform_events: &mut EventWriter<TransformEditEvent>,
) {
    egui::CollapsingHeader::new(format!("{} Transform", Icons::TRANSFORM))
        .default_open(true)
        .show(ui, |ui| {
            let mut position = transform.translation.truncate();
            let mut rotation_z = transform.rotation.to_euler(EulerRot::XYZ).2.to_degrees();
            let mut scale = transform.scale.truncate();

            ui.label("Position");
            ui.horizontal(|ui| {
                ui.label("X:");
                if ui.add(egui::DragValue::new(&mut position.x).speed(1.0)).changed() {
                    transform_events.send(TransformEditEvent::SetPosition { entity, position });
                }
                ui.label("Y:");
                if ui.add(egui::DragValue::new(&mut position.y).speed(1.0)).changed() {
                    transform_events.send(TransformEditEvent::SetPosition { entity, position });
                }
            });

            ui.label("Rotation (degrees)");
            if ui.add(egui::DragValue::new(&mut rotation_z).speed(1.0)).changed() {
                transform_events.send(TransformEditEvent::SetRotation {
                    entity,
                    rotation: rotation_z.to_radians(),
                });
            }

            ui.label("Scale");
            ui.horizontal(|ui| {
                ui.label("X:");
                if ui.add(egui::DragValue::new(&mut scale.x).speed(0.01)).changed() {
                    transform_events.send(TransformEditEvent::SetScale { entity, scale });
                }
                ui.label("Y:");
                if ui.add(egui::DragValue::new(&mut scale.y).speed(0.01)).changed() {
                    transform_events.send(TransformEditEvent::SetScale { entity, scale });
                }
            });
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
fn render_sprite_component(
    ui: &mut egui::Ui,
    sprite: &Sprite,
    entity: Entity,
    project_root: Option<&std::path::PathBuf>,
    asset_server: &AssetServer,
    texture_events: &mut bevy::ecs::event::EventWriter<crate::scene_editor::SpriteTextureEvent>,
    texture_id: Option<egui::TextureId>,
    images: &Assets<Image>,
) {
    egui::CollapsingHeader::new(format!("{} Sprite", Icons::SPRITE))
        .default_open(true)
        .show(ui, |ui| {
            ui.heading("Texture");

            // Display current texture with preview
            if !sprite.image.is_strong() {
                ui.label("None");
            } else {
                // Get asset path from handle
                if let Some(path) = asset_server.get_path(&sprite.image) {
                    ui.label(format!("{}", path.path().display()));

                    // Display texture preview if loaded and texture_id provided
                    if let Some(texture_id) = texture_id {
                        if let Some(image) = images.get(&sprite.image) {
                            // Calculate preview size (max 128x128 while maintaining aspect ratio)
                            let size = image.size();
                            let aspect_ratio = size.x as f32 / size.y as f32;
                            let (preview_width, preview_height) = if aspect_ratio > 1.0 {
                                (128.0, 128.0 / aspect_ratio)
                            } else {
                                (128.0 * aspect_ratio, 128.0)
                            };

                            // Display preview with border
                            ui.add_space(4.0);
                            egui::Frame::canvas(ui.style())
                                .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                                .show(ui, |ui| {
                                    ui.image(egui::ImageSource::Texture(egui::load::SizedTexture::new(
                                        texture_id,
                                        [preview_width, preview_height],
                                    )));
                                });

                            // Display texture info
                            ui.add_space(4.0);
                            ui.label(format!("Size: {}x{}", { size.x }, { size.y }));
                        } else {
                            ui.label("(Loading...)");
                        }
                    } else {
                        ui.label("(Texture preview unavailable)");
                    }
                } else {
                    ui.label("<loaded>");
                }
            }

            ui.add_space(4.0);

            // File picker button
            if ui.button(format!("{} Select Texture...", Icons::IMAGE)).clicked() {
                if let Some(root) = project_root {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(root)
                        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp"])
                        .pick_file()
                    {
                        // Load texture using absolute path since the AssetServer needs full path for user project assets
                        let texture_path = path.to_string_lossy().to_string().replace('\\', "/");

                        info!("Loading texture from absolute path: '{}'", texture_path);
                        let texture_handle: Handle<Image> = asset_server.load(&texture_path);

                        texture_events.send(crate::scene_editor::SpriteTextureEvent {
                            entity,
                            texture_handle,
                        });

                        info!("Assigned texture to sprite {:?}", entity);
                    }
                } else {
                    ui.label("No project loaded");
                }
            }

            ui.separator();

            ui.label(format!("Color: {:?}", sprite.color));
            ui.label(format!("Custom Size: {:?}", sprite.custom_size));
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

// Old inspector_panel_system - now replaced by panel_manager integration
// /// System to render the inspector panel
// pub fn inspector_panel_system(
//     mut contexts: bevy_egui::EguiContexts,
//     editor_scene: Res<EditorScene>,
//     mut panel: ResMut<InspectorPanel>,
//     component_registry: Res<EditorComponentRegistry>,
//     entity_query: Query<(
//         Entity,
//         Option<&Name>,
//         Option<&Transform>,
//         Option<&Visibility>,
//         Option<&Sprite>,
//         Option<&Camera2d>,
//         Option<&Node>,
//         Option<&Button>,
//         Option<&Text>,
//     )>,
// ) {
// }
