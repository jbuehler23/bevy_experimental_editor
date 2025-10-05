use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use eryndor_common::*;

use crate::{CurrentLevel, EditorState, EditorTool, EntityPalette, CollisionEditor};

/// Main UI system - draws all editor UI panels
pub fn ui_system(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    mut current_level: ResMut<CurrentLevel>,
    mut entity_palette: ResMut<EntityPalette>,
    mut collision_editor: ResMut<CollisionEditor>,
    workspace: Option<Res<crate::workspace::EditorWorkspace>>,
    mut project_selection: Option<ResMut<crate::project_manager::ProjectSelection>>,
    open_scenes: Res<crate::scene_tabs::OpenScenes>,  // Multi-scene support
) {
    let ctx = contexts.ctx_mut();

    // We need to draw panels in the correct order for egui 0.32
    // Bottom and Top panels first, then side panels, then central panel

    // Bottom status bar
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if let Some(scene) = open_scenes.active_scene() {
                ui.label(format!("Scene: {}", scene.name));
                ui.separator();
                ui.label(format!("Platforms: {}", scene.level_data.platforms.len()));
                ui.separator();
                ui.label(format!("Entities: {}", scene.level_data.entities.len()));
                ui.separator();
                if scene.is_modified {
                    ui.label("● Modified");
                } else {
                    ui.label("○ Saved");
                }
            } else {
                ui.label("No scene loaded");
            }
        });
    });

    // Top menu bar
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                ui.label("Save/Load via Ctrl+S / Ctrl+O");
                ui.label("(Saves all entities, platforms, and tilemap data)");
                ui.separator();

                // Recent Projects submenu
                if let Some(ref workspace) = workspace {
                    ui.menu_button("Recent Projects", |ui| {
                        if workspace.recent_projects.is_empty() {
                            ui.label("No recent projects");
                        } else {
                            for (idx, project_path) in workspace.recent_projects.iter().enumerate() {
                                // Extract project name from path
                                let project_name = std::path::Path::new(project_path)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(project_path);

                                if ui.button(format!("{}. {}", idx + 1, project_name))
                                    .on_hover_text(project_path)
                                    .clicked()
                                {
                                    // Open this project
                                    if let Some(ref mut selection) = project_selection {
                                        selection.state = crate::project_manager::ProjectSelectionState::Opening {
                                            path: project_path.clone(),
                                        };
                                        info!("Opening recent project: {}", project_path);
                                    }
                                    ui.close_menu();
                                }
                            }

                            ui.separator();
                            if ui.button("Clear Recent Projects").clicked() {
                                // Note: We can't mutate workspace here, will need a separate system
                                info!("Clear recent projects requested (not yet implemented)");
                                ui.close_menu();
                            }
                        }
                    });
                    ui.separator();
                }

                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo (Ctrl+Z)").clicked() {
                    // TODO: Undo system
                    ui.close_menu();
                }
                if ui.button("Redo (Ctrl+Y)").clicked() {
                    // TODO: Redo system
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                ui.checkbox(&mut editor_state.grid_snap_enabled, "Grid Snap");
                ui.add(egui::Slider::new(&mut editor_state.grid_size, 8.0..=128.0).text("Grid Size"));
            });

            ui.menu_button("Window", |ui| {
                if ui.checkbox(&mut collision_editor.active, "Collision Editor").clicked() {
                    ui.close_menu();
                }
            });
        });
    });

    // Left toolbar - Tools
    // NOTE: Commented out temporarily - layer panel now occupies left side
    // Will be integrated into layer panel or moved to top toolbar
    /* egui::SidePanel::left("toolbar").show(ctx, |ui| {
        ui.heading("Tools");
        ui.separator();

        if ui.selectable_label(editor_state.current_tool == EditorTool::Select, "🖱 Select").clicked() {
            editor_state.current_tool = EditorTool::Select;
        }
        if ui.selectable_label(editor_state.current_tool == EditorTool::Platform, "▬ Platform").clicked() {
            editor_state.current_tool = EditorTool::Platform;
        }
        if ui.selectable_label(editor_state.current_tool == EditorTool::EntityPlace, "🎭 Entity").clicked() {
            editor_state.current_tool = EditorTool::EntityPlace;
        }
        if ui.selectable_label(editor_state.current_tool == EditorTool::Erase, "✖ Erase").clicked() {
            editor_state.current_tool = EditorTool::Erase;
        }

        ui.separator();
        ui.label(format!("Current Tool: {:?}", editor_state.current_tool));
    }); */

    // Right panel - Entity Palette & Properties
    // NOTE: Temporarily disabled while tileset panel is active
    // Will be re-enabled with proper editor mode switching
    /* egui::SidePanel::right("properties").min_width(300.0).show(ctx, |ui| {
        ui.heading("Entity Palette");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("NPCs", |ui| {
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Npc(NpcType::Friendly))),
                    "Friendly NPC"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Npc(NpcType::Friendly));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Npc(NpcType::Hostile))),
                    "Hostile NPC"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Npc(NpcType::Hostile));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Npc(NpcType::Neutral))),
                    "Neutral NPC"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Npc(NpcType::Neutral));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Npc(NpcType::Vendor))),
                    "Vendor NPC"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Npc(NpcType::Vendor));
                }
            });

            ui.collapsing("Resources", |ui| {
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Resource(ResourceType::Tree))),
                    "Tree"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Resource(ResourceType::Tree));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Resource(ResourceType::Rock))),
                    "Rock"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Resource(ResourceType::Rock));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Resource(ResourceType::IronOre))),
                    "Iron Ore"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Resource(ResourceType::IronOre));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Resource(ResourceType::GoldOre))),
                    "Gold Ore"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Resource(ResourceType::GoldOre));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Resource(ResourceType::Bush))),
                    "Bush"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Resource(ResourceType::Bush));
                }
            });

            ui.collapsing("Interactive Objects", |ui| {
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Interactive(InteractiveType::Door))),
                    "Door"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Interactive(InteractiveType::Door));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Interactive(InteractiveType::Chest))),
                    "Chest"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Interactive(InteractiveType::Chest));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Interactive(InteractiveType::Lever))),
                    "Lever"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Interactive(InteractiveType::Lever));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::Interactive(InteractiveType::Portal))),
                    "Portal"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::Interactive(InteractiveType::Portal));
                }
            });

            ui.collapsing("Spawn Points", |ui| {
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::SpawnPoint(SpawnType::PlayerStart))),
                    "Player Start"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::SpawnPoint(SpawnType::PlayerStart));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::SpawnPoint(SpawnType::EnemySpawn))),
                    "Enemy Spawn"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::SpawnPoint(SpawnType::EnemySpawn));
                }
                if ui.selectable_label(
                    matches!(&entity_palette.selected_entity, Some(EntityType::SpawnPoint(SpawnType::ItemSpawn))),
                    "Item Spawn"
                ).clicked() {
                    entity_palette.selected_entity = Some(EntityType::SpawnPoint(SpawnType::ItemSpawn));
                }
            });
        });

        ui.separator();
        ui.label("Selected Entity:");
        if let Some(entity) = &entity_palette.selected_entity {
            ui.label(format!("{:?}", entity));
        } else {
            ui.label("None");
        }
    }); */
}
