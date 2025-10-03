use std::fs;
use std::path::{Path, PathBuf};
use eryndor_common::ProjectConfig;

/// Template for a new Bevy project
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectTemplate {
    /// Empty Bevy project with minimal setup
    Empty,
    /// 2D platformer with tilemap support
    Tilemap2D,
}

impl ProjectTemplate {
    pub fn name(&self) -> &str {
        match self {
            ProjectTemplate::Empty => "Empty Bevy Project",
            ProjectTemplate::Tilemap2D => "2D Tilemap Game",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ProjectTemplate::Empty => "A minimal Bevy project with basic setup",
            ProjectTemplate::Tilemap2D => "A 2D game with tilemap rendering and level loading support",
        }
    }
}

/// Generate a new Bevy project from a template
pub fn generate_project(
    project_path: &Path,
    project_name: &str,
    template: ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create project directory
    fs::create_dir_all(project_path)?;

    // Create directory structure
    create_directory_structure(project_path)?;

    // Generate Cargo.toml
    generate_cargo_toml(project_path, project_name, &template)?;

    // Generate project.bvy (editor config)
    generate_project_config(project_path, project_name)?;

    // Generate main.rs
    generate_main_rs(project_path, &template)?;

    // Generate .gitignore
    generate_gitignore(project_path)?;

    Ok(())
}

/// Create the standard directory structure
fn create_directory_structure(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dirs = vec![
        "src",
        "assets",
        "assets/tilesets",
        "assets/world",
        "assets/sprites",
        "assets/audio",
        ".cargo",
    ];

    for dir in dirs {
        fs::create_dir_all(project_path.join(dir))?;
    }

    // Create .cargo/config.toml to optimize builds for memory usage
    generate_cargo_config(project_path)?;

    Ok(())
}

/// Generate Cargo.toml for the project
fn generate_cargo_toml(
    project_path: &Path,
    project_name: &str,
    template: &ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert project name to valid Rust package name (lowercase, replace spaces with dashes)
    let package_name = project_name
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();

    let dependencies = match template {
        ProjectTemplate::Empty => {
            r#"bevy = "0.16""#
        }
        ProjectTemplate::Tilemap2D => {
            r#"bevy = { version = "0.16", features = ["file_watcher"] }
bevy_ecs_tilemap = "0.16"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0""#
        }
    };

    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
{}

[profile.dev]
opt-level = 1  # Improve development performance

[profile.dev.package."*"]
opt-level = 3  # Optimize dependencies for better performance
"#,
        package_name, package_name, dependencies
    );

    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;

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

/// Generate main.rs based on template
fn generate_main_rs(
    project_path: &Path,
    template: &ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_rs = match template {
        ProjectTemplate::Empty => generate_empty_main(),
        ProjectTemplate::Tilemap2D => generate_tilemap_main(),
    };

    fs::write(project_path.join("src/main.rs"), main_rs)?;

    Ok(())
}

/// Generate a minimal Bevy main.rs
fn generate_empty_main() -> String {
    r#"use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Bevy Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    info!("Game initialized!");
}

fn hello_world() {
    // Your game logic here
}
"#
    .to_string()
}

/// Generate a tilemap-enabled main.rs
fn generate_tilemap_main() -> String {
    r#"use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

/// Level data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub version: String,
    // Add your level data fields here
}

impl LevelData {
    /// Load level from JSON file
    pub fn load_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let level: LevelData = serde_json::from_str(&contents)?;
        Ok(level)
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".to_string(),
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "My Tilemap Game".to_string(),
                        resolution: (1280.0, 720.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
            TilemapPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    info!("Tilemap game initialized!");

    // Load your level here
    // Example: let level = LevelData::load_from_json("assets/world/level1.json").unwrap();
}
"#
    .to_string()
}

/// Generate .gitignore
fn generate_gitignore(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let gitignore = r#"/target
/.vscode
/.idea
*.log
.DS_Store
Cargo.lock
"#;

    fs::write(project_path.join(".gitignore"), gitignore)?;

    Ok(())
}

/// Generate .cargo/config.toml to optimize build performance and reduce memory usage
fn generate_cargo_config(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_config = r#"# Cargo configuration for optimized Bevy builds
# This reduces memory usage during compilation

[build]
# Use a single codegen unit to reduce memory usage (slower but uses less RAM)
# Increase this number if you have more RAM available
# codegen-units = 1

[profile.dev]
# Optimize dependencies even in dev mode to reduce memory usage
opt-level = 1

[profile.dev.package."*"]
# Optimize all dependencies in dev mode
opt-level = 3

# Use LLD linker for faster builds (Windows/Linux)
[target.x86_64-pc-windows-msvc]
linker = "rust-lld"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Reduce parallel jobs to prevent OOM on machines with limited RAM
# Uncomment and adjust if you're still running out of memory
# [build]
# jobs = 4
"#;

    fs::write(project_path.join(".cargo/config.toml"), cargo_config)?;

    Ok(())
}

/// Get the package name from a project's Cargo.toml
pub fn get_package_name_from_cargo_toml(project_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
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