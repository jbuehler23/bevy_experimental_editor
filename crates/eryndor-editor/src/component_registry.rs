//! Registry of all Bevy 2D components that can be added in the editor

use bevy::prelude::*;

/// Categories of components available in the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    Core,        // Transform, Name, Visibility
    Rendering,   // Sprite, Camera2D, etc.
    Audio,       // AudioSource, etc.
    UI,          // Node, Button, Text, etc.
    Physics,     // Avian physics components
    Animation,   // Animation components
    Custom,      // User-defined components
}

/// Information about a component type that can be added in the editor
#[derive(Clone)]
pub struct ComponentInfo {
    pub name: &'static str,
    pub category: ComponentCategory,
    pub description: &'static str,
    /// Function to spawn a default instance of this component
    pub spawn_fn: fn(&mut EntityWorldMut),
}

/// Registry of all available component types
pub struct ComponentRegistry {
    components: Vec<ComponentInfo>,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            components: Vec::new(),
        };

        // Register all built-in Bevy 2D components
        registry.register_core_components();
        registry.register_rendering_components();
        registry.register_audio_components();
        registry.register_ui_components();

        registry
    }

    /// Get all components in a category
    pub fn get_by_category(&self, category: ComponentCategory) -> Vec<&ComponentInfo> {
        self.components
            .iter()
            .filter(|c| c.category == category)
            .collect()
    }

    /// Get all categories
    pub fn categories(&self) -> Vec<ComponentCategory> {
        vec![
            ComponentCategory::Core,
            ComponentCategory::Rendering,
            ComponentCategory::Audio,
            ComponentCategory::UI,
            ComponentCategory::Physics,
            ComponentCategory::Animation,
            ComponentCategory::Custom,
        ]
    }

    /// Get category display name
    pub fn category_name(category: ComponentCategory) -> &'static str {
        match category {
            ComponentCategory::Core => "Core",
            ComponentCategory::Rendering => "Rendering",
            ComponentCategory::Audio => "Audio",
            ComponentCategory::UI => "UI",
            ComponentCategory::Physics => "Physics",
            ComponentCategory::Animation => "Animation",
            ComponentCategory::Custom => "Custom",
        }
    }

    fn register_core_components(&mut self) {
        self.components.push(ComponentInfo {
            name: "Transform",
            category: ComponentCategory::Core,
            description: "Position, rotation, and scale",
            spawn_fn: |entity| {
                entity.insert(Transform::default());
            },
        });

        self.components.push(ComponentInfo {
            name: "Name",
            category: ComponentCategory::Core,
            description: "Entity name for identification",
            spawn_fn: |entity| {
                entity.insert(Name::new("New Entity"));
            },
        });

        self.components.push(ComponentInfo {
            name: "Visibility",
            category: ComponentCategory::Core,
            description: "Controls visibility in rendering",
            spawn_fn: |entity| {
                entity.insert(Visibility::default());
            },
        });
    }

    fn register_rendering_components(&mut self) {
        self.components.push(ComponentInfo {
            name: "Sprite",
            category: ComponentCategory::Rendering,
            description: "2D sprite rendering",
            spawn_fn: |entity| {
                entity.insert(Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                });
            },
        });

        self.components.push(ComponentInfo {
            name: "Camera2D",
            category: ComponentCategory::Rendering,
            description: "2D camera",
            spawn_fn: |entity| {
                entity.insert(Camera2d::default());
            },
        });

        // Note: TextureAtlas requires TextureAtlasLayout handle, skip for now
    }

    fn register_audio_components(&mut self) {
        // Note: Audio components will be added once we figure out the correct API
        // The audio system in Bevy 0.16 uses AudioPlayer with AudioSource
    }

    fn register_ui_components(&mut self) {
        self.components.push(ComponentInfo {
            name: "Node",
            category: ComponentCategory::UI,
            description: "UI container element",
            spawn_fn: |entity| {
                entity.insert(Node::default());
            },
        });

        self.components.push(ComponentInfo {
            name: "Button",
            category: ComponentCategory::UI,
            description: "Interactive button",
            spawn_fn: |entity| {
                entity.insert(Button);
            },
        });

        self.components.push(ComponentInfo {
            name: "Text",
            category: ComponentCategory::UI,
            description: "Text rendering",
            spawn_fn: |entity| {
                entity.insert(Text::new("New Text"));
            },
        });

        self.components.push(ComponentInfo {
            name: "BackgroundColor",
            category: ComponentCategory::UI,
            description: "UI element background color",
            spawn_fn: |entity| {
                entity.insert(BackgroundColor(Color::WHITE));
            },
        });
    }
}

/// Resource for the component registry
#[derive(Resource)]
pub struct EditorComponentRegistry {
    pub registry: ComponentRegistry,
}

impl Default for EditorComponentRegistry {
    fn default() -> Self {
        Self {
            registry: ComponentRegistry::new(),
        }
    }
}
