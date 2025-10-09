//! Asset browser for managing textures and sprites in the editor

use bevy::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resource managing all loaded assets in the editor
#[derive(Resource, Default)]
pub struct AssetBrowser {
    /// All discovered texture assets (path -> AssetInfo)
    pub textures: HashMap<PathBuf, TextureAssetInfo>,
    /// Currently selected texture path
    pub selected_texture: Option<PathBuf>,
    /// Base assets directory
    pub assets_directory: Option<PathBuf>,
    /// Whether assets need to be rescanned
    pub needs_rescan: bool,
}

/// Information about a texture asset
#[derive(Clone)]
pub struct TextureAssetInfo {
    /// Relative path from assets directory
    pub path: PathBuf,
    /// Loaded texture handle
    pub handle: Handle<Image>,
    /// Display name (filename without extension)
    pub name: String,
    /// File size in bytes
    pub file_size: u64,
}

impl AssetBrowser {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            selected_texture: None,
            assets_directory: None,
            needs_rescan: true,
        }
    }

    /// Set the assets directory and mark for rescan
    pub fn set_assets_directory(&mut self, path: PathBuf) {
        self.assets_directory = Some(path);
        self.needs_rescan = true;
    }

    /// Scan the assets directory for texture files
    pub fn scan_assets(&mut self, asset_server: &AssetServer) {
        if !self.needs_rescan {
            return;
        }

        let Some(assets_dir) = self.assets_directory.clone() else {
            return;
        };

        self.textures.clear();

        // Supported image extensions
        let extensions = ["png", "jpg", "jpeg", "bmp", "tga", "webp"];

        // Recursively scan for texture files
        if let Ok(entries) = std::fs::read_dir(&assets_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let path = entry.path();

                        if let Some(ext) = path.extension() {
                            if extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                                self.add_texture_asset(asset_server, &path, &assets_dir, metadata.len());
                            }
                        }
                    } else if metadata.is_dir() {
                        // Recursively scan subdirectories
                        self.scan_directory_recursive(asset_server, &entry.path(), &assets_dir, &extensions);
                    }
                }
            }
        }

        self.needs_rescan = false;
        info!("Asset browser scanned {} textures", self.textures.len());
    }

    /// Recursively scan a directory for texture files
    fn scan_directory_recursive(
        &mut self,
        asset_server: &AssetServer,
        dir: &Path,
        assets_root: &Path,
        extensions: &[&str],
    ) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let path = entry.path();

                    if metadata.is_file() {
                        if let Some(ext) = path.extension() {
                            if extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                                self.add_texture_asset(asset_server, &path, assets_root, metadata.len());
                            }
                        }
                    } else if metadata.is_dir() {
                        self.scan_directory_recursive(asset_server, &path, assets_root, extensions);
                    }
                }
            }
        }
    }

    /// Add a texture asset to the browser
    fn add_texture_asset(
        &mut self,
        asset_server: &AssetServer,
        full_path: &Path,
        assets_root: &Path,
        file_size: u64,
    ) {
        // Get relative path from assets directory
        let relative_path = full_path
            .strip_prefix(assets_root)
            .ok()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| full_path.to_path_buf());

        // Get filename without extension for display
        let name = full_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Load the texture using AssetServer
        // Convert path to use forward slashes for asset loading
        let asset_path = relative_path
            .to_str()
            .unwrap_or("")
            .replace('\\', "/");

        let handle: Handle<Image> = asset_server.load(&asset_path);

        let info = TextureAssetInfo {
            path: relative_path.clone(),
            handle,
            name,
            file_size,
        };

        self.textures.insert(relative_path, info);
    }

    /// Select a texture by path
    pub fn select_texture(&mut self, path: &Path) {
        if self.textures.contains_key(path) {
            self.selected_texture = Some(path.to_path_buf());
        }
    }

    /// Get the currently selected texture info
    pub fn get_selected_texture(&self) -> Option<&TextureAssetInfo> {
        self.selected_texture
            .as_ref()
            .and_then(|path| self.textures.get(path))
    }

    /// Get a texture by path
    pub fn get_texture(&self, path: &Path) -> Option<&TextureAssetInfo> {
        self.textures.get(path)
    }

    /// Clear all loaded textures
    pub fn clear(&mut self) {
        self.textures.clear();
        self.selected_texture = None;
    }

    /// Get all texture paths sorted by name
    pub fn get_sorted_textures(&self) -> Vec<&TextureAssetInfo> {
        let mut textures: Vec<_> = self.textures.values().collect();
        textures.sort_by(|a, b| a.name.cmp(&b.name));
        textures
    }
}

/// System to scan assets when the browser is initialized or marked for rescan
pub fn scan_assets_system(
    mut asset_browser: ResMut<AssetBrowser>,
    asset_server: Res<AssetServer>,
) {
    if asset_browser.needs_rescan {
        asset_browser.scan_assets(&asset_server);
    }
}
