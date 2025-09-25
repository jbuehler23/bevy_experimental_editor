use bevy::prelude::*;
use crate::components::{ArenaConfig, Border};

pub fn setup_arena(
    mut commands: Commands,
    arena_config: Res<ArenaConfig>,
) {
    let world_size = arena_config.world_size;
    let thickness = arena_config.border_thickness;

    // North border
    spawn_border(
        &mut commands,
        Vec2::new(world_size / 2.0, world_size + thickness / 2.0),
        Vec2::new(world_size + thickness * 2.0, thickness),
    );

    // South border
    spawn_border(
        &mut commands,
        Vec2::new(world_size / 2.0, -thickness / 2.0),
        Vec2::new(world_size + thickness * 2.0, thickness),
    );

    // East border
    spawn_border(
        &mut commands,
        Vec2::new(world_size + thickness / 2.0, world_size / 2.0),
        Vec2::new(thickness, world_size + thickness * 2.0),
    );

    // West border
    spawn_border(
        &mut commands,
        Vec2::new(-thickness / 2.0, world_size / 2.0),
        Vec2::new(thickness, world_size + thickness * 2.0),
    );
}

fn spawn_border(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 1.0),
        Border,
    ));
}