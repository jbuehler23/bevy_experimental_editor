use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

mod bevy_cli_runner;
mod build_progress_ui;
mod camera;
mod cli_output_panel;
mod collision_editor;
mod component_registry;
mod gizmos;
mod icons;
mod inspector_panel;
mod layer_manager;
mod layer_panel;
mod map_canvas;
mod panel_manager;
mod project_generator;
mod project_manager;
mod project_wizard;
mod rendering;
mod scene_editor;
mod scene_loader;
mod scene_loader_template;
mod scene_tabs;
mod scene_tree_panel;
mod selection;
mod shortcuts;
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
use camera::*;
use cli_output_panel::*;
use collision_editor::*;
use gizmos::*;
use layer_manager::*;
// use layer_panel::*;  // Commented out - using panel_manager instead
use map_canvas::*;
use project_manager::*;
use project_wizard::*;
use rendering::*;
use scene_loader::*;
use scene_tabs::*;
use selection::*;
use shortcuts::*;
use systems::*;
use tile_painter::*;
use tileset_manager::*;
// use tileset_panel::*;  // Commented out - using panel_manager instead
use tileset_panel::{SelectTileEvent, SelectTilesetEvent, handle_tile_selection_events};
use toolbar::*;
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
                        title: "Eryndor Level Editor".to_string(),
                        resolution: (1920.0, 1080.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
            EguiPlugin {
                enable_multipass_for_primary_context: false,
            },
            WorldInspectorPlugin::default(),
            PanCamPlugin,
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
        // Tilemap events
        .add_event::<LoadTilesetEvent>()
        .add_event::<SelectTileEvent>()
        .add_event::<SelectTilesetEvent>()
        .add_event::<PaintTileEvent>()
        // Scene editor events
        .add_event::<scene_tree_panel::SceneTreeCommand>()
        .add_event::<scene_editor::TransformEditEvent>()
        .add_event::<scene_editor::NameEditEvent>()
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
        .add_systems(Update, handle_global_shortcuts)
        .add_systems(
            Update,
            (
                handle_project_selection,
                auto_load_scene_system,
                project_selection_ui,
                project_wizard_ui,
                update_cli_runner,         // Update bevy CLI runner
                build_progress_overlay_ui, // Build progress overlay (MUST run before ui_system to be on top)
                (
                    ui_system,
                    panel_manager::render_left_panel,  // Left panel with Scene Tree and Layers tabs
                    panel_manager::render_right_panel, // Right panel with Inspector and Tilesets tabs
                ).chain(), // Ensure panels render in strict order within same frame
                scene_tree_panel::handle_scene_tree_commands, // Handle scene tree commands
                scene_editor::handle_transform_edit_events, // Handle transform edit events
                scene_editor::handle_name_edit_events, // Handle name edit events
            )
                .after(handle_global_shortcuts),
        )
        .add_systems(
            Update,
            (
                sync_tilemap_on_scene_switch, // Sync tilemap when tab changes
                scene_tabs::sync_editor_scene_on_tab_change, // Sync EditorScene when tab changes
                scene_tabs::mark_loaded_scene_entities, // Mark loaded scene entities with EditorSceneEntity
                disable_pancam_over_ui
                    .after(panel_manager::render_right_panel)
                    .after(ui_system),
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
                handle_selection,
                handle_entity_deletion,
                draw_grid,
                draw_selection_gizmos,
                draw_scene_entity_gizmos,
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
    // Spawn camera with pan/zoom controls
    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.2, 0.25)),
            ..default()
        },
        PanCam {
            grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
            enabled: true,
            zoom_to_cursor: false, // Disable scroll zoom to avoid conflicts with UI
            min_scale: 0.1,
            max_scale: 5.0,
            ..default()
        },
    ));

    info!("Eryndor Level Editor initialized!");
}

/// Disable PanCam when mouse is over egui UI
fn disable_pancam_over_ui(
    mut contexts: bevy_egui::EguiContexts,
    mut pancam_query: Query<&mut PanCam>,
) {
    let ctx = contexts.ctx_mut();
    let is_over_ui = ctx.is_pointer_over_area();

    for mut pancam in &mut pancam_query {
        let prev_enabled = pancam.enabled;
        pancam.enabled = !is_over_ui;

        // Debug: Log when state changes
        if prev_enabled != pancam.enabled {
            if pancam.enabled {
                debug!("PanCam enabled (mouse left UI)");
            } else {
                debug!("PanCam disabled (mouse over UI)");
            }
        }
    }
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
    pub level_data: eryndor_common::LevelData,
    pub file_path: Option<String>,
    pub is_modified: bool,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            level_data: eryndor_common::LevelData::new(
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
    pub selected_entity: Option<eryndor_common::EntityType>,
}
