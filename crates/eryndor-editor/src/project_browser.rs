//! Project file browser for navigating and managing project files and folders

use bevy::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Resource managing the project file browser
#[derive(Resource, Default)]
pub struct ProjectBrowser {
    /// Root project directory
    pub project_root: Option<PathBuf>,
    /// Currently expanded folders (path -> is_expanded)
    pub expanded_folders: HashMap<PathBuf, bool>,
    /// Currently selected file/folder
    pub selected_path: Option<PathBuf>,
    /// Cached directory structure
    pub root_entries: Vec<FileEntry>,
    /// Whether the browser needs to refresh
    pub needs_refresh: bool,
    /// Filter for file types (empty = show all)
    pub file_filter: Option<String>,
}

/// Represents a file or folder in the project
#[derive(Clone, Debug)]
pub struct FileEntry {
    /// Full path to the file/folder
    pub path: PathBuf,
    /// Display name (filename/folder name)
    pub name: String,
    /// Whether this is a directory
    pub is_directory: bool,
    /// File size (0 for directories)
    pub size: u64,
    /// File extension (empty for directories)
    pub extension: String,
    /// Children entries (for directories)
    pub children: Vec<FileEntry>,
    /// File type category for icon selection
    pub file_type: FileType,
}

/// File type categories for icons and filtering
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileType {
    Folder,
    Scene,       // .bscene files
    Image,       // .png, .jpg, etc.
    Audio,       // .wav, .ogg, .mp3
    Script,      // .rs files
    Config,      // .toml, .json, .ron
    Text,        // .txt, .md
    Tileset,     // tileset files
    Unknown,
}

impl FileType {
    /// Get file type from extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "bscene" | "scene" => FileType::Scene,
            "png" | "jpg" | "jpeg" | "bmp" | "tga" | "webp" | "gif" => FileType::Image,
            "wav" | "ogg" | "mp3" | "flac" => FileType::Audio,
            "rs" => FileType::Script,
            "toml" | "json" | "ron" | "yaml" | "yml" => FileType::Config,
            "txt" | "md" | "rst" => FileType::Text,
            _ => FileType::Unknown,
        }
    }

    /// Get icon for file type
    pub fn icon(&self) -> &'static str {
        use crate::icons::Icons;
        match self {
            FileType::Folder => Icons::FOLDER,
            FileType::Scene => Icons::FILE,
            FileType::Image => Icons::IMAGE,
            FileType::Audio => Icons::AUDIO,
            FileType::Script => Icons::SCRIPT,
            FileType::Config => Icons::SETTINGS,
            FileType::Text => Icons::FILE,
            FileType::Tileset => Icons::TILEMAP,
            FileType::Unknown => Icons::FILE,
        }
    }
}

impl ProjectBrowser {
    pub fn new() -> Self {
        Self {
            project_root: None,
            expanded_folders: HashMap::new(),
            selected_path: None,
            root_entries: Vec::new(),
            needs_refresh: true,
            file_filter: None,
        }
    }

    /// Set the project root directory
    pub fn set_project_root(&mut self, path: PathBuf) {
        self.project_root = Some(path);
        self.needs_refresh = true;
    }

    /// Refresh the directory structure
    pub fn refresh(&mut self) {
        if !self.needs_refresh {
            return;
        }

        let Some(ref root) = self.project_root else {
            return;
        };

        self.root_entries = Self::scan_directory(root, root, &self.file_filter);
        self.needs_refresh = false;
        info!("Project browser refreshed: {} entries", self.root_entries.len());
    }

    /// Recursively scan a directory
    fn scan_directory(dir: &Path, project_root: &Path, filter: &Option<String>) -> Vec<FileEntry> {
        let mut entries = Vec::new();

        let Ok(read_dir) = fs::read_dir(dir) else {
            return entries;
        };

        for entry in read_dir.flatten() {
            let Ok(metadata) = entry.metadata() else {
                continue;
            };

            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Skip hidden files and common ignore patterns
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }

            if metadata.is_dir() {
                // Don't recursively load children immediately - load on expand
                let entry = FileEntry {
                    path: path.clone(),
                    name,
                    is_directory: true,
                    size: 0,
                    extension: String::new(),
                    children: Vec::new(),
                    file_type: FileType::Folder,
                };
                entries.push(entry);
            } else {
                let extension = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string();

                // Apply filter if set
                if let Some(ref filter_ext) = filter {
                    if !extension.eq_ignore_ascii_case(filter_ext) {
                        continue;
                    }
                }

                let file_type = FileType::from_extension(&extension);

                let entry = FileEntry {
                    path: path.clone(),
                    name,
                    is_directory: false,
                    size: metadata.len(),
                    extension,
                    children: Vec::new(),
                    file_type,
                };
                entries.push(entry);
            }
        }

        // Sort: folders first, then alphabetically
        entries.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        entries
    }

    /// Toggle folder expansion
    pub fn toggle_folder(&mut self, path: &Path) {
        let is_expanded = self.expanded_folders.get(path).copied().unwrap_or(false);
        self.expanded_folders.insert(path.to_path_buf(), !is_expanded);

        // Load children if expanding for the first time
        if !is_expanded {
            self.load_folder_children(path);
        }
    }

    /// Check if a folder is expanded
    pub fn is_expanded(&self, path: &Path) -> bool {
        self.expanded_folders.get(path).copied().unwrap_or(false)
    }

    /// Load children for a folder
    fn load_folder_children(&mut self, folder_path: &Path) {
        let Some(ref root) = self.project_root else {
            return;
        };

        // Find the folder entry and populate its children
        Self::load_children_recursive(&mut self.root_entries, folder_path, root, &self.file_filter);
    }

    fn load_children_recursive(
        entries: &mut [FileEntry],
        target_path: &Path,
        project_root: &Path,
        filter: &Option<String>,
    ) {
        for entry in entries.iter_mut() {
            if entry.path == target_path && entry.is_directory {
                entry.children = Self::scan_directory(&entry.path, project_root, filter);
                return;
            } else if entry.is_directory && !entry.children.is_empty() {
                Self::load_children_recursive(&mut entry.children, target_path, project_root, filter);
            }
        }
    }

    /// Select a file or folder
    pub fn select(&mut self, path: &Path) {
        self.selected_path = Some(path.to_path_buf());
    }

    /// Get the currently selected path
    pub fn get_selected(&self) -> Option<&PathBuf> {
        self.selected_path.as_ref()
    }

    /// Get the selected file entry
    pub fn get_selected_entry(&self) -> Option<FileEntry> {
        let selected = self.selected_path.as_ref()?;
        Self::find_entry(&self.root_entries, selected)
    }

    fn find_entry(entries: &[FileEntry], path: &Path) -> Option<FileEntry> {
        for entry in entries {
            if entry.path == path {
                return Some(entry.clone());
            }
            if entry.is_directory {
                if let Some(found) = Self::find_entry(&entry.children, path) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Create a new folder
    pub fn create_folder(&mut self, parent_path: &Path, name: &str) -> Result<PathBuf, std::io::Error> {
        let new_path = parent_path.join(name);
        fs::create_dir(&new_path)?;
        self.needs_refresh = true;
        Ok(new_path)
    }

    /// Delete a file or folder
    pub fn delete(&mut self, path: &Path) -> Result<(), std::io::Error> {
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        self.needs_refresh = true;
        Ok(())
    }

    /// Rename a file or folder
    pub fn rename(&mut self, old_path: &Path, new_name: &str) -> Result<PathBuf, std::io::Error> {
        let parent = old_path.parent().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "No parent directory")
        })?;
        let new_path = parent.join(new_name);
        fs::rename(old_path, &new_path)?;
        self.needs_refresh = true;
        Ok(new_path)
    }
}

/// System to refresh the project browser when needed
pub fn refresh_project_browser_system(mut browser: ResMut<ProjectBrowser>) {
    if browser.needs_refresh {
        browser.refresh();
    }
}
