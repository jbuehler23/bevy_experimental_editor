use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

mod camera;
mod gizmos;
mod rendering;
mod selection;
mod systems;
mod ui;
mod tools;
mod tileset_manager;
mod tileset_panel;
mod layer_manager;
mod layer_panel;
mod tile_painter;
mod collision_editor;
mod map_canvas;
mod project_manager;
mod client_launcher;
mod project_ui;
mod project_generator;
mod project_wizard;
mod build_manager;

use camera::*;
use gizmos::*;
use rendering::*;
use selection::*;
use systems::*;
use ui::*;
use tileset_manager::*;
use tileset_panel::*;
use layer_manager::*;
use layer_panel::*;
use tile_painter::*;
use map_canvas::*;
use collision_editor::*;
use project_manager::*;
use client_launcher::*;
use project_ui::*;
use project_wizard::*;
use build_manager::*;

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
            EguiPlugin { enable_multipass_for_primary_context: false },
            WorldInspectorPlugin::default(),
            PanCamPlugin,
            bevy_ecs_tilemap::TilemapPlugin,
        ))
        // Editor resources
        .init_resource::<EditorState>()
        .init_resource::<CurrentLevel>()
        .init_resource::<EntityPalette>()
        .init_resource::<Selection>()
        .init_resource::<EditorEntityMap>()
        .init_resource::<systems::PendingTilemapRestore>()
        // Project resources
        .init_resource::<ProjectSelection>()
        .init_resource::<ProjectWizard>()
        .init_resource::<StandaloneClient>()
        .init_resource::<BuildManager>()
        // Tilemap resources
        .init_resource::<TilesetManager>()
        .init_resource::<tileset_panel::TilesetZoom>()
        .init_resource::<LayerManager>()
        .init_resource::<TilePainter>()
        .init_resource::<CollisionEditor>()
        .init_resource::<MapDimensions>()
        // Tilemap events
        .add_event::<LoadTilesetEvent>()
        .add_event::<SelectTileEvent>()
        .add_event::<SelectTilesetEvent>()
        .add_event::<PaintTileEvent>()
        // Systems - Split into smaller groups to avoid tuple size limit
        .add_systems(Startup, setup_editor)
        .add_systems(Update, (
            handle_project_selection,
            project_selection_ui,
            project_wizard_ui,
            play_controls_ui,
            monitor_client_process,
            poll_build_status,
            ui_system,
            tileset_panel_ui,
            layer_panel_ui,
            disable_pancam_over_ui.after(tileset_panel_ui).after(ui_system),
            handle_tile_selection_events,
            handle_entity_placement,
            handle_save_load,
            handle_platform_editing,
        ))
        .add_systems(Update, (
            handle_selection,
            handle_entity_deletion,
            draw_grid,
            draw_selection_gizmos,
        ))
        // Tilemap systems
        .add_systems(Update, (
            handle_tileset_load_requests,
            update_tileset_dimensions,
            setup_map_canvas,
            handle_tile_painting,
            update_map_canvas_on_layer_changes,
            handle_paint_tile_events,
            handle_canvas_click_painting,
        ))
        // Collision editor systems
        .add_systems(Update, (
            collision_editor_ui,
            handle_collision_input,
            render_collision_shapes,
        ))
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
            zoom_to_cursor: false,  // Disable scroll zoom to avoid conflicts with UI
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
