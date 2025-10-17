//! Project templates for the Bevy editor.
//!
//! This crate provides directory-based templates for different types of Bevy games.
//! Templates are embedded at compile time and can be copied to create new projects.

use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Embedded templates directory
static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid template structure: {0}")]
    InvalidStructure(String),
}

/// Represents a project template
#[derive(Debug, Clone)]
pub struct Template {
    /// Internal identifier (directory name)
    pub id: &'static str,
    /// Display name shown in UI
    pub name: &'static str,
    /// Brief description of what this template provides
    pub description: &'static str,
}

/// All available templates
pub const TEMPLATES: &[Template] = &[
    Template {
        id: "empty",
        name: "Empty/Basic",
        description: "Minimal Bevy app with just a camera. Clean slate for any game type.",
    },
    Template {
        id: "sprite_2d",
        name: "2D Sprite Game",
        description: "2D game with sprite rendering, camera controls, and basic player movement example.",
    },
    Template {
        id: "editor_game",
        name: "Editor Game (Scene Loading)",
        description: "Minimal setup with scene loading plugin for seamless editor integration.",
    },
    Template {
        id: "tilemap_2d",
        name: "2D Tilemap Game",
        description: "2D game with tilemap support, scene loading, and level rendering.",
    },
];

/// Get a template by its ID
pub fn get_template(id: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.id == id)
}

/// List all available templates
pub fn list_templates() -> &'static [Template] {
    TEMPLATES
}

/// Variables that can be substituted in template files
#[derive(Debug, Clone)]
pub struct TemplateVariables {
    /// User-provided project name (e.g., "My Game")
    pub project_name: String,
    /// Sanitized package name for Cargo.toml (e.g., "my-game")
    pub package_name: String,
    /// Relative path to bevy_editor_runtime crate
    pub editor_runtime_path: String,
}

impl TemplateVariables {
    /// Create template variables from a project name
    pub fn new(project_name: String) -> Self {
        let package_name = sanitize_package_name(&project_name);
        Self {
            project_name,
            package_name,
            editor_runtime_path: "../bevy_experimental_editor/crates/bevy_editor_runtime".to_string(),
        }
    }

    /// Perform variable substitution on a string
    pub fn substitute(&self, content: &str) -> String {
        content
            .replace("{{PROJECT_NAME}}", &self.project_name)
            .replace("{{PACKAGE_NAME}}", &self.package_name)
            .replace("{{EDITOR_RUNTIME_PATH}}", &self.editor_runtime_path)
    }
}

/// Sanitize a project name into a valid Cargo package name
fn sanitize_package_name(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

/// Copy a template to the specified destination path
pub fn copy_template(
    template_id: &str,
    dest_path: &Path,
    variables: &TemplateVariables,
) -> Result<(), TemplateError> {
    // Verify template exists
    if get_template(template_id).is_none() {
        return Err(TemplateError::TemplateNotFound(template_id.to_string()));
    }

    // Get the template directory
    let template_dir = TEMPLATES_DIR
        .get_dir(template_id)
        .ok_or_else(|| TemplateError::TemplateNotFound(template_id.to_string()))?;

    // Create destination directory
    fs::create_dir_all(dest_path)?;

    // Recursively copy all files from template
    copy_dir_recursive(template_dir, dest_path, variables)?;

    Ok(())
}

/// Recursively copy a directory from the embedded template to the filesystem
fn copy_dir_recursive(
    src_dir: &Dir,
    dest_path: &Path,
    variables: &TemplateVariables,
) -> Result<(), TemplateError> {
    // Create the destination directory
    fs::create_dir_all(dest_path)?;

    // Copy all files in this directory
    for file in src_dir.files() {
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Handle .template files specially (they need variable substitution)
        let (dest_file_name, needs_substitution) = if file_name.ends_with(".template") {
            (file_name.trim_end_matches(".template"), true)
        } else {
            (file_name, false)
        };

        let dest_file_path = dest_path.join(dest_file_name);

        if needs_substitution {
            // Read as UTF-8, perform substitution, write
            let content = file.contents_utf8()
                .ok_or_else(|| TemplateError::InvalidStructure(
                    format!("File {} is not valid UTF-8", file_name)
                ))?;
            let substituted = variables.substitute(content);
            fs::write(dest_file_path, substituted)?;
        } else {
            // Copy binary file as-is
            fs::write(dest_file_path, file.contents())?;
        }
    }

    // Recursively copy subdirectories
    for subdir in src_dir.dirs() {
        let subdir_name = subdir.path().file_name().unwrap();
        let dest_subdir = dest_path.join(subdir_name);
        copy_dir_recursive(subdir, &dest_subdir, variables)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_package_name() {
        assert_eq!(sanitize_package_name("My Game"), "my-game");
        assert_eq!(sanitize_package_name("Test_Project"), "test_project");
        assert_eq!(sanitize_package_name("Game#123!"), "game123");
    }

    #[test]
    fn test_template_variables() {
        let vars = TemplateVariables::new("My Cool Game".to_string());
        assert_eq!(vars.project_name, "My Cool Game");
        assert_eq!(vars.package_name, "my-cool-game");

        let content = "name = \"{{PACKAGE_NAME}}\"\ntitle = \"{{PROJECT_NAME}}\"";
        let result = vars.substitute(content);
        assert_eq!(result, "name = \"my-cool-game\"\ntitle = \"My Cool Game\"");
    }

    #[test]
    fn test_list_templates() {
        let templates = list_templates();
        assert!(templates.len() >= 4);
        assert!(templates.iter().any(|t| t.id == "empty"));
        assert!(templates.iter().any(|t| t.id == "sprite_2d"));
        assert!(templates.iter().any(|t| t.id == "editor_game"));
        assert!(templates.iter().any(|t| t.id == "tilemap_2d"));
    }

    #[test]
    fn test_get_template() {
        assert!(get_template("empty").is_some());
        assert!(get_template("nonexistent").is_none());
    }
}
