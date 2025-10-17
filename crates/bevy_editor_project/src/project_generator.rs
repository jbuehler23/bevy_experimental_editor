//! Project generation from templates.
//!
//! This module handles creating new Bevy projects from various templates.
//! Templates are provided by the `bevy_editor_templates` crate.

use bevy::prelude::*;
use bevy_editor_formats::ProjectConfig;
use bevy_editor_templates::{self, TemplateVariables};
use std::fs;
use std::path::Path;

/// Template for a new Bevy project
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectTemplate {
    /// Empty Bevy project with minimal setup
    Empty,
    /// 2D sprite game with camera controls and player movement
    Sprite2D,
    /// Editor-integrated game with scene loading support
    EditorGame,
    /// 2D tilemap game with bevy_ecs_tilemap
    Tilemap2D,
    /// bevy_new_2d template (requires bevy CLI)
    BevyNew2D,
}

impl ProjectTemplate {
    pub fn name(&self) -> &str {
        match self {
            ProjectTemplate::Empty => "Empty/Basic",
            ProjectTemplate::Sprite2D => "2D Sprite Game",
            ProjectTemplate::EditorGame => "Editor Game (Scene Loading)",
            ProjectTemplate::Tilemap2D => "2D Tilemap Game",
            ProjectTemplate::BevyNew2D => "Bevy 2D Game (bevy_new_2d)",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ProjectTemplate::Empty => "Minimal Bevy app with just a camera. Clean slate for any game type.",
            ProjectTemplate::Sprite2D => "2D game with sprite rendering, camera controls, and basic player movement example.",
            ProjectTemplate::EditorGame => "Minimal setup with scene loading plugin for seamless editor integration.",
            ProjectTemplate::Tilemap2D => "2D game with tilemap support, scene loading, and level rendering.",
            ProjectTemplate::BevyNew2D => "Official bevy_new_2d template with hot-reloading and fast builds (requires bevy CLI)",
        }
    }

    pub fn requires_bevy_cli(&self) -> bool {
        matches!(self, ProjectTemplate::BevyNew2D)
    }

    /// Get the template ID for the templates crate
    fn template_id(&self) -> Option<&'static str> {
        match self {
            ProjectTemplate::Empty => Some("empty"),
            ProjectTemplate::Sprite2D => Some("sprite_2d"),
            ProjectTemplate::EditorGame => Some("editor_game"),
            ProjectTemplate::Tilemap2D => Some("tilemap_2d"),
            ProjectTemplate::BevyNew2D => None, // Uses bevy CLI
        }
    }
}

/// Generate a new Bevy project from a template
pub fn generate_project(
    project_path: &Path,
    project_name: &str,
    template: ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    // Special handling for bevy_new_2d template
    if matches!(template, ProjectTemplate::BevyNew2D) {
        return generate_from_bevy_cli(project_path, project_name);
    }

    // Get template ID
    let template_id = template.template_id()
        .ok_or("Template does not have a template ID")?;

    // Create template variables
    let variables = TemplateVariables::new(project_name.to_string());

    // Copy template files
    bevy_editor_templates::copy_template(template_id, project_path, &variables)?;

    // Generate project.bvy (editor config)
    generate_project_config(project_path, project_name)?;

    info!("Successfully created project '{}' from template '{}'", project_name, template.name());

    Ok(())
}

/// Generate project using bevy CLI (for bevy_new_2d template)
fn generate_from_bevy_cli(
    project_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    // Get parent directory where bevy will create the project
    let parent_dir = project_path.parent().ok_or("Invalid project path")?;

    // Check if directory already exists - bevy new fails if it does
    if project_path.exists() {
        return Err(format!("Directory already exists: {:?}", project_path).into());
    }

    info!("Running 'bevy new {} --template 2d'...", project_name);

    // Run bevy new command with environment variable to skip itch.io prompt
    let status = Command::new("bevy")
        .args(["new", project_name, "--template", "2d"])
        .env("CARGO_GENERATE_VALUE_ITCH_USERNAME", "") // Skip itch.io username prompt
        .current_dir(parent_dir)
        .status()
        .map_err(|e| format!("Failed to run 'bevy new': {}. Is bevy CLI installed?", e))?;

    if !status.success() {
        return Err(format!("bevy new failed with exit code: {:?}", status.code()).into());
    }

    // Add project.bvy (editor config) to the generated project
    generate_project_config(project_path, project_name)?;

    // Add editor-specific assets directories
    fs::create_dir_all(project_path.join("assets/world"))?;
    fs::create_dir_all(project_path.join("assets/tilesets"))?;

    // Add .cargo/config.toml for fast linking (bevy_new_2d doesn't include this)
    generate_cargo_config_for_bevy_new_2d(project_path)?;

    // Update DEVELOPMENT.md with editor info
    let dev_md_path = project_path.join("DEVELOPMENT.md");
    if dev_md_path.exists() {
        let existing = fs::read_to_string(&dev_md_path)?;
        let updated = format!("{}\n\n## Bevy Editor Integration\n\nThis project was created with the Bevy Editor using the bevy_new_2d template.\n\n- Use the editor to create and edit levels\n- Levels are saved in `assets/world/` as `.scn.ron` files\n- Use the toolbar buttons to run, test, and build your game\n\n", existing);
        fs::write(&dev_md_path, updated)?;
    }

    info!("Successfully created project '{}' from bevy_new_2d template", project_name);

    Ok(())
}

/// Generate project.bvy (editor configuration)
fn generate_project_config(
    project_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = ProjectConfig::new(project_name.to_string());
    config.client_config.window_title = project_name.to_string();

    let config_path = project_path.join("project.bvy");
    config.save_to_file(config_path)?;

    Ok(())
}

/// Generate .cargo/config.toml for bevy_new_2d projects (adds fast linker)
fn generate_cargo_config_for_bevy_new_2d(
    project_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create .cargo directory
    let cargo_dir = project_path.join(".cargo");
    fs::create_dir_all(&cargo_dir)?;

    // bevy_new_2d template doesn't include .cargo/config.toml
    // Adding fast linker configuration dramatically speeds up builds
    let cargo_config = r#"# Cargo configuration for fast Bevy builds
# Added by Bevy Editor

[target.x86_64-pc-windows-msvc]
# Use LLD linker (much faster than default MSVC linker)
linker = "rust-lld.exe"
rustdocflags = ["-Clinker=rust-lld.exe"]

[target.x86_64-unknown-linux-gnu]
# Use mold linker on Linux (fastest available)
# Install: sudo apt install mold clang
# Uncomment to use:
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.x86_64-apple-darwin]
# macOS uses zld linker (much faster than default ld)
# Install: brew install michaeleisel/zld/zld
# Uncomment to use:
# rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
# M1/M2/M3 Macs - use zld linker
# Install: brew install michaeleisel/zld/zld
# Uncomment to use:
# rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/bin/zld"]
"#;

    fs::write(cargo_dir.join("config.toml"), cargo_config)?;
    info!("Added .cargo/config.toml with fast linker configuration");
    Ok(())
}

/// Get the package name from a project's Cargo.toml
pub fn get_package_name_from_cargo_toml(
    project_path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let contents = fs::read_to_string(cargo_toml_path)?;

    let cargo_toml: toml::Value = toml::from_str(&contents)?;

    if let Some(package) = cargo_toml.get("package") {
        if let Some(name) = package.get("name") {
            if let Some(name_str) = name.as_str() {
                return Ok(name_str.to_string());
            }
        }
    }

    Err("Could not find package name in Cargo.toml".into())
}
