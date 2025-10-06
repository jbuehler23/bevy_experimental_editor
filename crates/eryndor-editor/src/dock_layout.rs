//! Dockable panel layout system using egui_dock

use bevy::prelude::*;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};

use crate::component_registry::EditorComponentRegistry;
use crate::inspector_panel::{render_inspector_panel, EntityComponentData};
use crate::layer_manager::LayerManager;
use crate::scene_editor::EditorScene;
use crate::scene_tree_panel::{render_scene_tree_panel, EntityNodeData};
use crate::tileset_manager::TilesetManager;

/// Types of panels that can be docked
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanelType {
    SceneTree,
    Inspector,
    Layers,
    Tileset,
}

impl PanelType {
    pub fn title(&self) -> &'static str {
        match self {
            PanelType::SceneTree => "Scene Tree",
            PanelType::Inspector => "Inspector",
            PanelType::Layers => "Layers",
            PanelType::Tileset => "Tileset",
        }
    }
}

/// Resource managing the dock state
#[derive(Resource)]
pub struct EditorDockState {
    pub dock_state: DockState<PanelType>,
}

impl Default for EditorDockState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorDockState {
    pub fn new() -> Self {
        // Create the default layout
        let mut dock_state = DockState::new(vec![PanelType::SceneTree]);

        // Split the main surface to create left, center, and right sections
        // This creates: [SceneTree] | [Center] | [Inspector]

        // Get the main surface (index 0)
        let surface = dock_state.main_surface_mut();

        // Split right to add Inspector
        let [_left, right] = surface.split_right(NodeIndex::root(), 0.75, vec![PanelType::Inspector]);

        // Add Tileset tab to Inspector panel
        surface.push_to_focused_leaf(PanelType::Tileset);

        // Add Layers tab to SceneTree panel
        surface.set_focused_node(NodeIndex::root());
        surface.push_to_focused_leaf(PanelType::Layers);

        Self { dock_state }
    }
}

/// Context data passed to panel rendering
pub struct PanelContext<'a> {
    pub editor_scene: &'a mut EditorScene,
    pub component_registry: &'a EditorComponentRegistry,
    pub layer_manager: &'a LayerManager,
    pub tileset_manager: &'a TilesetManager,
    pub entity_data: &'a [EntityNodeData],
    pub selected_entity_data: Option<&'a EntityComponentData>,
}

/// Implements the TabViewer trait to render panel contents
pub struct EditorTabViewer<'a> {
    pub context: PanelContext<'a>,
}

impl TabViewer for EditorTabViewer<'_> {
    type Tab = PanelType;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            PanelType::SceneTree => {
                render_scene_tree_panel(
                    ui,
                    self.context.editor_scene,
                    self.context.entity_data,
                );
            }
            PanelType::Inspector => {
                render_inspector_panel(
                    ui,
                    self.context.editor_scene,
                    self.context.selected_entity_data,
                    &self.context.component_registry.registry,
                );
            }
            PanelType::Layers => {
                render_layers_panel(ui, self.context.layer_manager);
            }
            PanelType::Tileset => {
                render_tileset_panel(ui, self.context.tileset_manager);
            }
        }
    }
}

/// Render the layers panel content
fn render_layers_panel(ui: &mut egui::Ui, layer_manager: &LayerManager) {
    ui.heading("Layers");
    ui.separator();

    // Show current layer
    if let Some(current_layer) = layer_manager.get_layer(layer_manager.current_layer_index) {
        ui.label(format!("Current: {} ({})", current_layer.name, current_layer.index));
        ui.label(format!("Type: {:?}", current_layer.layer_type));
        ui.label(format!("Visible: {}", current_layer.visible));
    }

    ui.separator();

    // List all layers
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (idx, layer) in layer_manager.layers.iter().enumerate() {
            ui.horizontal(|ui| {
                let is_current = idx == layer_manager.current_layer_index;
                if ui.selectable_label(is_current, &layer.name).clicked() {
                    info!("Selected layer: {} (index {})", layer.name, idx);
                }
            });
        }
    });
}

/// Render the tileset panel content
fn render_tileset_panel(ui: &mut egui::Ui, tileset_manager: &TilesetManager) {
    ui.heading("Tileset");
    ui.separator();

    if let Some(tileset) = tileset_manager.get_active_tileset() {
        ui.label(format!("Active: {}", tileset.name));
        ui.label(format!("Tiles: {}", tileset.tile_count));
        ui.label(format!("Size: {}x{}", tileset.tile_width, tileset.tile_height));
    } else {
        ui.label("No tileset loaded");
        ui.label("Load a tileset using the toolbar");
    }

    ui.separator();
    ui.label("(Full tileset UI will be shown here)");
}

/// System to render the docked panels
pub fn dock_system(
    mut contexts: bevy_egui::EguiContexts,
    mut dock_state: ResMut<EditorDockState>,
    mut editor_scene: ResMut<EditorScene>,
    component_registry: Res<EditorComponentRegistry>,
    layer_manager: Res<LayerManager>,
    tileset_manager: Res<TilesetManager>,
    entity_query: Query<(Entity, Option<&Name>, Option<&Children>), With<crate::scene_editor::EditorSceneEntity>>,
    component_query: Query<(
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

    // Find component data for selected entity
    let selected_entity_data = editor_scene.selected_entity.and_then(|selected| {
        component_query.iter().find_map(|(entity, name, transform, visibility, sprite, camera2d, node, button, text)| {
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

    // Create panel context
    let context = PanelContext {
        editor_scene: &mut editor_scene,
        component_registry: &component_registry,
        layer_manager: &layer_manager,
        tileset_manager: &tileset_manager,
        entity_data: &entity_data,
        selected_entity_data: selected_entity_data.as_ref(),
    };

    // Create tab viewer
    let mut tab_viewer = EditorTabViewer { context };

    // Render the dock area in the central panel
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        DockArea::new(&mut dock_state.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut tab_viewer);
    });
}
