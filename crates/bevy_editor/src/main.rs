//! Bevy Experimental Editor
//!
//! An experimental game editor built with Bevy Engine.

#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Import core editor primitives from bevy_editor_core
use bevy_editor_core::{self as core, EditorCameraPlugin};

mod asset_browser;
mod asset_browser_panel;
mod bevy_cli_runner;
mod build_progress_ui;
mod cli_output_panel;
mod collision_editor;
mod component_registry;
mod editor_commands;
mod entity_templates;
mod formats;
mod gizmos;  // Keep local gizmos module for drawing logic
mod icons;
mod inspector_panel;
mod layer_manager;
mod layer_panel;
mod map_canvas;
mod panel_manager;
mod project_browser;
mod project_browser_panel;
mod project_generator;
mod project_manager;
mod project_wizard;
mod rendering;
mod scene_editor;
mod scene_loader;
mod scene_loader_template;
mod scene_tabs;
mod scene_tree_panel;
mod selection;  // Keep local selection module for editor-specific selection logic
mod shortcuts;  // Keep local shortcuts module for editor-specific shortcuts
mod systems;
mod tile_painter;
mod tilemap_component;
mod tileset_manager;
mod tileset_panel;
mod toolbar;
mod tools;
mod ui;
mod viewport_selection;
mod workspace;

use bevy_cli_runner::*;
use build_progress_ui::*;
use cli_output_panel::*;
use collision_editor::*;
use gizmos::*;
use layer_manager::*;
// use layer_panel::*;  // Commented out - using panel_manager instead
use map_canvas::*;
use project_manager::*;
use project_wizard::*;
use scene_loader::*;
use scene_tabs::*;
use selection::*;
use shortcuts::*;
use systems::*;
use tile_painter::*;
use tileset_manager::*;
// use tileset_panel::*;  // Commented out - using panel_manager instead
use tileset_panel::{handle_tile_selection_events, SelectTileEvent, SelectTilesetEvent};
use ui::*;
use workspace::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Editor".to_string(),
                        resolution: (1920.0, 1080.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
            EguiPlugin {
                enable_multipass_for_primary_context: false,
            },
            WorldInspectorPlugin::default(),
            bevy_ecs_tilemap::TilemapPlugin,
        ))
        // Editor resources
        .init_resource::<EditorState>()
        .init_resource::<CurrentLevel>() // Keep for backward compatibility temporarily
        .init_resource::<OpenScenes>() // Multi-scene tab system
        .init_resource::<EntityPalette>()
        .init_resource::<Selection>()
        .init_resource::<EditorEntityMap>()
        .init_resource::<systems::PendingTilemapRestore>()
        // Scene editor resources
        .init_resource::<scene_editor::EditorScene>()
        .init_resource::<component_registry::EditorComponentRegistry>()
        .init_resource::<scene_tree_panel::SceneTreePanel>()
        .init_resource::<inspector_panel::InspectorPanel>()
        .init_resource::<panel_manager::PanelManager>()
        .init_resource::<panel_manager::NameEditBuffer>()
        .init_resource::<viewport_selection::GizmoDragState>()
        .init_resource::<gizmos::GizmoState>()
        .init_resource::<core::EditorHistory>()
        // Project resources
        .init_resource::<ProjectSelection>()
        .init_resource::<ProjectWizard>()
        .init_resource::<BevyCLIRunner>()
        .init_resource::<CLIOutputPanel>()
        // Tilemap resources
        .init_resource::<TilesetManager>()
        .init_resource::<tileset_panel::TilesetZoom>()
        .init_resource::<LayerManager>()
        .init_resource::<TilePainter>()
        .init_resource::<CollisionEditor>()
        .init_resource::<MapDimensions>()
        .init_resource::<SceneAutoLoader>()
        // Asset browser resources (deprecated - keeping for backward compatibility)
        .init_resource::<asset_browser::AssetBrowser>()
        .init_resource::<asset_browser_panel::AssetBrowserPanel>()
        // Project browser resources
        .init_resource::<project_browser::ProjectBrowser>()
        .init_resource::<project_browser_panel::ProjectBrowserPanel>()
        // Tilemap events
        .add_event::<LoadTilesetEvent>()
        .add_event::<SelectTileEvent>()
        .add_event::<SelectTilesetEvent>()
        .add_event::<PaintTileEvent>()
        // Scene editor events
        .add_event::<scene_tree_panel::SceneTreeCommand>()
        .add_event::<scene_editor::TransformEditEvent>()
        .add_event::<scene_editor::NameEditEvent>()
        .add_event::<scene_editor::SpriteTextureEvent>()
        .add_event::<scene_tabs::SceneTabChanged>()
        // Systems - Split into smaller groups to avoid tuple size limit
        .add_systems(
            Startup,
            (
                setup_editor,
                scene_editor::setup_editor_scene,
                ensure_default_layer_system,
                load_workspace_system,
            ),
        )
        // Keyboard shortcuts MUST run before UI to capture shortcuts
        .add_systems(
            Update,
            (handle_global_shortcuts, handle_gizmo_mode_shortcuts),
        )
        // Camera controls from bevy_editor_core
        .add_plugins(EditorCameraPlugin)
        .add_systems(
            Update,
            (
                handle_project_selection,
                auto_load_scene_system,
                project_selection_ui,
                project_wizard_ui,
                update_cli_runner,                 // Update bevy CLI runner
                build_progress_overlay_ui, // Build progress overlay (MUST run before ui_system to be on top)
                asset_browser::scan_assets_system, // Scan for texture assets (deprecated)
                project_browser::refresh_project_browser_system, // Refresh project browser
                (
                    ui_system,
                    panel_manager::render_left_panel, // Left panel with Scene Tree and Layers tabs
                    panel_manager::render_right_panel, // Right panel with Inspector and Tilesets tabs
                )
                    .chain(), // Ensure panels render in strict order within same frame
                scene_tree_panel::handle_scene_tree_commands, // Handle scene tree commands
                scene_editor::handle_transform_edit_events, // Handle transform edit events
                scene_editor::handle_name_edit_events, // Handle name edit events
                scene_editor::handle_sprite_texture_events, // Handle sprite texture assignment events
            )
                .after(handle_global_shortcuts),
        )
        .add_systems(
            Update,
            (
                sync_tilemap_on_scene_switch, // Sync tilemap when tab changes
                scene_tabs::sync_editor_scene_on_tab_change, // Sync EditorScene when tab changes
                scene_tabs::mark_loaded_scene_entities, // Mark loaded scene entities with EditorSceneEntity
                handle_tile_selection_events,
                handle_entity_placement,
                handle_save_load,
                handle_platform_editing,
            ),
        )
        .add_systems(
            Update,
            (
                viewport_selection::viewport_entity_selection_system,
                viewport_selection::gizmo_drag_interaction_system,
                viewport_selection::transform_with_undo_system
                    .after(viewport_selection::gizmo_drag_interaction_system),
                handle_selection,
                handle_entity_deletion,
                draw_grid,
                draw_selection_gizmos, // Handles both old and new editor entities
                draw_gizmo_mode_indicator, // Show current gizmo mode in viewport
            ),
        )
        // Tilemap systems
        .add_systems(
            Update,
            (
                handle_tileset_load_requests,
                update_tileset_dimensions,
                setup_map_canvas,
                handle_tile_painting,
                update_map_canvas_on_layer_changes,
                handle_paint_tile_events,
                handle_canvas_click_painting,
                // Tilemap component systems
                tilemap_component::sync_tilemap_entities,
                tilemap_component::cleanup_tilemap_entities,
            ),
        )
        // Collision editor systems
        .add_systems(
            Update,
            (
                collision_editor_ui,
                handle_collision_input,
                render_collision_shapes,
            ),
        )
        // Level restore system
        .add_systems(Update, restore_tilemap_from_level)
        .run();
}

fn setup_editor(mut commands: Commands) {
    // Spawn camera with custom editor controls from bevy_editor_core
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.2, 0.25)),
            ..default()
        },
        core::EditorCamera::default(),
    ));

    info!("Bevy Experimental Editor initialized!");
}

#[derive(Resource)]
pub struct EditorState {
    pub current_tool: EditorTool,
    pub grid_snap_enabled: bool,
    pub grid_size: f32,
    pub is_playing: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            current_tool: EditorTool::default(),
            grid_snap_enabled: true,
            grid_size: 32.0,
            is_playing: false,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum EditorTool {
    #[default]
    Select,
    Platform,
    EntityPlace,
    Erase,
    Eyedropper,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_data: crate::formats::LevelData,
    pub file_path: Option<String>,
    pub is_modified: bool,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            level_data: crate::formats::LevelData::new(
                "Untitled Level".to_string(),
                2000.0,
                1000.0,
            ),
            file_path: None,
            is_modified: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct EntityPalette {
    pub selected_entity: Option<crate::formats::EntityType>,
}
