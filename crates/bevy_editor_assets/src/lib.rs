//! Bevy editor asset backend.
//!
//! This crate exposes the core asset browser resource and systems used by the
//! modular editor. UI code lives in higher-level crates so this crate can remain
//! headless-friendly.

mod texture_browser;

pub use texture_browser::{
    scan_assets_system, AssetBrowser, AssetBrowserSet, TextureAssetInfo, TextureHandleProvider,
};

use bevy::prelude::*;

/// Plugin that wires the asset browser resource and scanning system into an
/// application.
pub struct AssetBrowserPlugin;

impl Plugin for AssetBrowserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetBrowser>()
            .configure_sets(Update, AssetBrowserSet)
            .add_systems(Update, scan_assets_system.in_set(AssetBrowserSet));
    }
}

/// Plugin variant without the automatic scanner. Consumers can drive the
/// resource manually if they prefer custom scheduling.
pub struct AssetBrowserCorePlugin;

impl Plugin for AssetBrowserCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetBrowser>();
    }
}
