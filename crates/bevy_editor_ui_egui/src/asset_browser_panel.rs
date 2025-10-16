//! UI panel for the asset browser

use bevy::prelude::*;
use bevy_editor_assets::{AssetBrowser, TextureAssetInfo};
use bevy_editor_frontend_api::AssetBrowserPanelState;
use bevy_egui::egui;

use crate::icons::Icons;

/// Render the asset browser panel
pub fn asset_browser_panel_ui(
    ui: &mut egui::Ui,
    asset_browser: &mut AssetBrowser,
    panel: &AssetBrowserPanelState,
) {
    ui.heading(format!("{} Assets", Icons::FOLDER_OPEN));
    ui.separator();

    // Toolbar
    ui.horizontal(|ui| {
        if ui.button(format!("{} Refresh", Icons::REFRESH)).clicked() {
            asset_browser.needs_rescan = true;
        }

        ui.label(format!("{} textures", asset_browser.textures.len()));
    });

    ui.separator();

    // Texture grid
    if asset_browser.textures.is_empty() {
        ui.label("No textures found");
        ui.label("Place .png, .jpg, or other image files in the assets directory");
    } else {
        // Collect texture info to avoid borrow checker issues with closures
        let textures: Vec<_> = asset_browser
            .get_sorted_textures()
            .into_iter()
            .cloned()
            .collect();

        egui::ScrollArea::vertical().show(ui, |ui| {
            render_texture_grid(ui, asset_browser, &textures, panel);
        });
    }
}

/// Render texture thumbnails in a grid layout
fn render_texture_grid(
    ui: &mut egui::Ui,
    asset_browser: &mut AssetBrowser,
    textures: &[TextureAssetInfo],
    panel: &AssetBrowserPanelState,
) {
    let thumbnail_size = egui::vec2(panel.thumbnail_size, panel.thumbnail_size);
    let spacing = 8.0;
    let total_width = ui.available_width();
    let item_width = thumbnail_size.x + spacing;
    let columns = (total_width / item_width).floor().max(1.0) as usize;

    egui::Grid::new("asset_grid")
        .spacing([spacing, spacing])
        .show(ui, |ui| {
            for (idx, texture_info) in textures.iter().enumerate() {
                let is_selected = asset_browser
                    .selected_texture
                    .as_ref()
                    .map(|p| p == &texture_info.path)
                    .unwrap_or(false);

                // Render texture item
                render_texture_item(ui, asset_browser, texture_info, thumbnail_size, is_selected);

                // New row after each set of columns
                if (idx + 1) % columns == 0 {
                    ui.end_row();
                }
            }
        });
}

/// Render a single texture item
fn render_texture_item(
    ui: &mut egui::Ui,
    asset_browser: &mut AssetBrowser,
    texture_info: &TextureAssetInfo,
    thumbnail_size: egui::Vec2,
    is_selected: bool,
) {
    ui.vertical(|ui| {
        ui.set_width(thumbnail_size.x);

        // Texture thumbnail (placeholder for now - we'll add actual rendering later)
        let (rect, response) = ui.allocate_exact_size(thumbnail_size, egui::Sense::click());

        // Draw background
        let bg_color = if is_selected {
            egui::Color32::from_rgb(60, 120, 180)
        } else if response.hovered() {
            egui::Color32::from_rgb(50, 50, 50)
        } else {
            egui::Color32::from_rgb(30, 30, 30)
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);

        // Draw placeholder icon or texture
        // TODO: Render actual texture preview using bevy_egui texture integration
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            Icons::IMAGE,
            egui::FontId::proportional(32.0),
            egui::Color32::from_rgb(150, 150, 150),
        );

        // Handle click
        if response.clicked() {
            asset_browser.select_texture(&texture_info.path);
            info!("Selected texture: {}", texture_info.name);
        }

        // Filename label
        ui.label(
            egui::RichText::new(&texture_info.name)
                .size(10.0)
                .color(if is_selected {
                    egui::Color32::from_rgb(150, 200, 255)
                } else {
                    egui::Color32::from_rgb(200, 200, 200)
                }),
        );

        // File size
        let size_kb = texture_info.file_size as f32 / 1024.0;
        ui.label(
            egui::RichText::new(format!("{:.1} KB", size_kb))
                .size(9.0)
                .color(egui::Color32::from_rgb(120, 120, 120)),
        );
    });
}
