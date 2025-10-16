use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Backend resource that tracks discovered files for the project browser.
#[derive(Resource)]
pub struct ProjectBrowser {
    pub project_root: Option<PathBuf>,
    pub expanded_folders: HashMap<PathBuf, bool>,
    pub selected_path: Option<PathBuf>,
    pub root_entries: Vec<FileEntry>,
    pub needs_refresh: bool,
    pub file_filter: Option<String>,
}

impl Default for ProjectBrowser {
    fn default() -> Self {
        Self {
            project_root: None,
            expanded_folders: HashMap::new(),
            selected_path: None,
            root_entries: Vec::new(),
            needs_refresh: true,
            file_filter: None,
        }
    }
}

impl ProjectBrowser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_project_root(&mut self, path: PathBuf) {
        self.project_root = Some(path);
        self.needs_refresh = true;
    }

    pub fn refresh(&mut self) {
        if !self.needs_refresh {
            return;
        }

        let Some(ref root) = self.project_root else {
            return;
        };

        self.root_entries = Self::scan_directory(root, root, &self.file_filter);
        self.needs_refresh = false;
    }

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
            let name = entry.file_name().to_str().unwrap_or("unknown").to_string();

            if metadata.is_dir() {
                let mut dir_entry = FileEntry::directory(path.clone(), name.clone());

                if Self::matches_filter(&name, filter) {
                    dir_entry.children = Self::scan_directory(&path, project_root, filter);
                    entries.push(dir_entry);
                } else {
                    let children = Self::scan_directory(&path, project_root, filter);
                    if !children.is_empty() {
                        dir_entry.children = children;
                        entries.push(dir_entry);
                    }
                }
            } else {
                let size = metadata.len();
                let extension = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_string();

                if !Self::matches_extension(&extension, filter) {
                    continue;
                }

                let file_type = FileType::from_extension(&extension);

                entries.push(FileEntry::file(
                    path,
                    name,
                    size,
                    extension.clone(),
                    file_type,
                ));
            }
        }

        entries.sort_by(|a, b| {
            // Directories first, then alphabetical
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        entries
    }

    fn matches_filter(name: &str, filter: &Option<String>) -> bool {
        match filter {
            Some(filter) if !filter.is_empty() => name.contains(filter),
            _ => true,
        }
    }

    fn matches_extension(extension: &str, filter: &Option<String>) -> bool {
        match filter {
            Some(filter) if !filter.is_empty() => extension.contains(filter),
            _ => true,
        }
    }

    pub fn toggle_folder(&mut self, path: &Path) {
        let is_expanded = self
            .expanded_folders
            .entry(path.to_path_buf())
            .or_insert(false);
        *is_expanded = !*is_expanded;
    }

    pub fn is_expanded(&self, path: &Path) -> bool {
        self.expanded_folders.get(path).copied().unwrap_or_default()
    }

    pub fn select(&mut self, path: &Path) {
        self.selected_path = Some(path.to_path_buf());
    }

    pub fn get_selected(&self) -> Option<&PathBuf> {
        self.selected_path.as_ref()
    }

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

    pub fn create_folder(
        &mut self,
        parent_path: &Path,
        name: &str,
    ) -> Result<PathBuf, std::io::Error> {
        let new_path = parent_path.join(name);
        fs::create_dir(&new_path)?;
        self.needs_refresh = true;
        Ok(new_path)
    }

    pub fn delete(&mut self, path: &Path) -> Result<(), std::io::Error> {
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        self.needs_refresh = true;
        Ok(())
    }

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

/// View model representing a file or directory in the project.
#[derive(Clone, Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub extension: String,
    pub children: Vec<FileEntry>,
    pub file_type: FileType,
}

impl FileEntry {
    fn directory(path: PathBuf, name: String) -> Self {
        Self {
            path,
            name,
            is_directory: true,
            size: 0,
            extension: String::new(),
            children: Vec::new(),
            file_type: FileType::Folder,
        }
    }

    fn file(
        path: PathBuf,
        name: String,
        size: u64,
        extension: String,
        file_type: FileType,
    ) -> Self {
        Self {
            path,
            name,
            is_directory: false,
            size,
            extension,
            children: Vec::new(),
            file_type,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileType {
    Folder,
    Scene,
    Image,
    Audio,
    Script,
    Config,
    Text,
    Tileset,
    Unknown,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "bscene" | "scene" => FileType::Scene,
            "png" | "jpg" | "jpeg" | "bmp" | "tga" | "webp" | "gif" => FileType::Image,
            "wav" | "ogg" | "mp3" | "flac" => FileType::Audio,
            "rs" => FileType::Script,
            "toml" | "json" | "ron" | "yaml" | "yml" => FileType::Config,
            "txt" | "md" | "rst" => FileType::Text,
            "tiles" | "tileset" => FileType::Tileset,
            _ => FileType::Unknown,
        }
    }
}

/// UI-agnostic state for the project browser panel.
#[derive(Resource, Debug)]
pub struct ProjectBrowserPanelState {
    pub visible: bool,
    pub new_folder_name: String,
    pub show_new_folder_dialog: bool,
    pub new_folder_parent: Option<PathBuf>,
}

impl Default for ProjectBrowserPanelState {
    fn default() -> Self {
        Self {
            visible: true,
            new_folder_name: String::new(),
            show_new_folder_dialog: false,
            new_folder_parent: None,
        }
    }
}

impl ProjectBrowserPanelState {
    pub fn new() -> Self {
        Self::default()
    }
}
